//! Tokio: queues, replies, shared state, and concurrency limits.
//!
//! Setup: cargo add tokio@1 --features full
//! Cargo.toml: tokio = { version="1", features=["full"] }
//! Run: cargo run --bin 03_channels_shared_state
//!
//! Review: a pending .await can suspend THIS TASK without blocking its OS thread.
//! An async channel can wait for queue space or a message in that way.
//! A blocking channel/lock instead parks its calling thread while waiting.
//! Ready awaits may finish immediately; .await does not always switch tasks.
//!
//! Queue capacity, active-operation limits, and runtime worker counts are
//! DIFFERENT controls. A capacity of 2 does not mean two threads or two jobs total.
//! Examples are finite and need no network, files, or external services.
//!
//! References:
//! https://tokio.rs/tokio/tutorial/channels
//! https://tokio.rs/tokio/tutorial/shared-state
//! https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
//! https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html
//! https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html
//! https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html

use std::error::Error;
use std::future::ready;
use std::sync::{Arc, Mutex as StdMutex};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot, Mutex as AsyncMutex, Semaphore};
use tokio::time::sleep;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!();

    demo_bounded_queue().await?;
    println!("\n============================\n");

    demo_multiple_senders().await?;
    println!("\n============================\n");

    demo_close_and_drain().await?;
    println!("\n============================\n");

    demo_oneshot_reply().await?;
    println!("\n============================\n");

    demo_state_owner_task().await?;
    println!("\n============================\n");

    demo_standard_mutex().await?;
    println!("\n============================\n");

    demo_async_mutex().await?;
    println!("\n============================\n");

    demo_semaphore_inside_tasks().await?;
    println!("\n============================\n");

    demo_acquire_before_spawn().await?;
    println!("\n============================\n");

    demo_select_and_channel_cancellation().await?;

    Ok(())
}

// =====================================================
// 01. Bounded mpsc: full queues apply backpressure
// =====================================================

async fn demo_bounded_queue() -> Result<(), Box<dyn Error>> {
    println!("01. Bounded mpsc: full queues apply backpressure");

    let (tx, mut rx) = mpsc::channel(1);
    tx.send("A").await?;
    println!("Sent A: queued={}, free capacity={}", rx.len(), tx.capacity());
    // Queue now holds [A]. There is ONE message slot, not one worker.
    // send returned when the queue accepted A, not when anyone processed A.

    println!();

    match tx.try_send("B") {
        Err(mpsc::error::TrySendError::Full(message)) => {
            println!("Expected Full; caller recovered {message:?}");
        }
        other => println!("Unexpected try_send result: {other:?}"),
    }
    // try_send does not wait. send(...).await instead waits for free capacity.

    let producer = async move {
        println!("Producer: requesting a slot for B");
        tx.send("B").await?;
        println!("Producer: B was accepted");
        Ok::<(), mpsc::error::SendError<&'static str>>(())
        // tx is owned here and drops when this future finishes.
    };
    let consumer = async move {
        let mut received = Vec::new();
        while let Some(message) = rx.recv().await {
            println!("Consumer: removed {message}");
            received.push(message);
        }
        received
    };
    let (sent, received) = tokio::join!(producer, consumer);
    sent?;
    println!("Stream ended: {received:?}"); // ["A", "B"]
    // Removing A frees space for B. Either branch can progress while the other
    // awaits readiness, even with this single-thread runtime. No task was spawned.
    // Exact print interleaving can vary; A is received before B.
    // Awaiting producer to completion BEFORE starting consumer would deadlock
    // here: the full queue needs the consumer to run. Capacity must be >0.
    // unbounded_channel exists, but removes this queued-message memory limit.

    Ok(())
}

// =====================================================
// 02. Sender clones and the end of a message stream
// =====================================================

async fn demo_multiple_senders() -> Result<(), Box<dyn Error>> {
    println!("02. Sender clones and the end of a message stream");

    let (tx, mut rx) = mpsc::channel(2);
    let mut handles = Vec::new();
    for producer_id in 0..2 {
        let sender = tx.clone();
        handles.push(tokio::spawn(async move {
            for sequence in 0..3 {
                sender.send((producer_id, sequence)).await?;
            }
            Ok::<(), mpsc::error::SendError<(i32, i32)>>(())
        }));
    }
    drop(tx); // remove the original handle, not just the clones in the tasks

    while let Some((producer, sequence)) = rx.recv().await {
        println!("Received producer={producer}, sequence={sequence}");
    }
    for handle in handles {
        handle.await??;
    }
    // recv returns None after all senders are gone and buffered messages drain.
    // If the original tx stayed alive, this receive loop could wait forever.
    // Each producer's own sequence is preserved; producers can interleave.
    // There is one Receiver, not a cloneable receiver for every worker.
    // Ownership of each sent non-Copy value transfers into the channel.
    // Channel closure is EOF, not proof every job succeeded: also inspect task results.
    Ok(())
}

// =====================================================
// 03. Receiver close(): reject new sends, drain accepted work
// =====================================================

async fn demo_close_and_drain() -> Result<(), Box<dyn Error>> {
    println!("03. Receiver close(): reject new sends, drain accepted work");

    let (tx, mut rx) = mpsc::channel(2);
    tx.send(String::from("accepted A")).await?;
    tx.send(String::from("accepted B")).await?;
    rx.close();
    println!("Receiver stopped accepting ordinary new sends");

    println!();

    match tx.send(String::from("too late")).await {
        Ok(()) => println!("Unexpected successful send"),
        Err(error) => println!("Expected send error; recovered {:?}", error.0),
    }
    while let Some(message) = rx.recv().await {
        println!("Drained: {message}");
    }
    // Explicit close allows this loop to end even while tx is still alive.
    // Previously reserved permits are an exception: they can still submit their
    // messages, and outstanding permits can delay recv returning None.
    // Dropping rx instead discards buffered unread messages and makes sends fail.
    // Stopping intake, draining, and waiting for processing are separate decisions.
    Ok(())
}

// =====================================================
// 04. oneshot: one reply, with no asynchronous send
// =====================================================

async fn demo_oneshot_reply() -> Result<(), Box<dyn Error>> {
    println!("04. oneshot: one reply, with no asynchronous send");

    let (reply_tx, reply_rx) = oneshot::channel();
    let task = tokio::spawn(async move {
        sleep(Duration::from_millis(5)).await;
        let _ = reply_tx.send(42_u32); // send is synchronous; do NOT append .await
    });
    println!("Caller: waiting for one reply");
    println!("Reply: {}", reply_rx.await?);
    task.await?;
    // oneshot has one sender and one receiver; send consumes its sender.
    // Awaiting the receiver suspends this task until a reply or disconnection.
    // If the caller dropped reply_rx, send would return Err(containing the value).

    println!();

    let (tx, rx) = oneshot::channel::<u32>();
    drop(tx);
    println!("Expected missing-reply error: {:?}", rx.await);
    // A reply can itself be Result<T, E>: awaiting then produces another error
    // layer, distinguishing missing reply from an operation's explicit failure.
    Ok(())
}

// =====================================================
// 05. A state-owning task can replace a shared mutex
// =====================================================

async fn demo_state_owner_task() -> Result<(), Box<dyn Error>> {
    println!("05. A state-owning task can replace a shared mutex");

    enum Command {
        Add(u32),
        Get { reply: oneshot::Sender<u32> },
    }

    let (tx, mut rx) = mpsc::channel(4);
    let manager = tokio::spawn(async move {
        let mut total = 0_u32;
        while let Some(command) = rx.recv().await {
            match command {
                Command::Add(value) => total += value,
                Command::Get { reply } => {
                    let _ = reply.send(total);
                }
            }
        }
        total
    });

    tx.send(Command::Add(10)).await?;
    tx.send(Command::Add(20)).await?;
    let (reply, answer) = oneshot::channel();
    tx.send(Command::Get { reply }).await?;
    println!("Total reported by manager: {}", answer.await?); // 30
    drop(tx);
    println!("Manager drained commands and finished: {}", manager.await?);
    // mpsc carries requests; each Get carries its own one-message return address.
    // This one task owns total, so no Arc<Mutex<u32>> is needed.
    // This sender submits Add, Add, Get in order; the reply therefore includes both adds.
    // A real manager could own a connection or another stateful resource.
    // Queue capacity does not limit the time one command takes; lengthy command
    // processing delays later requests. Decide whether serialization is appropriate.
    // Arithmetic is safe for these tiny values; real business logic needs validation.
    Ok(())
}

// =====================================================
// 06. A standard mutex is useful for short synchronous updates
// =====================================================

async fn demo_standard_mutex() -> Result<(), Box<dyn Error>> {
    println!("06. A standard mutex is useful for short synchronous updates");

    let counter = Arc::new(StdMutex::new(0_usize));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            {
                let mut count = counter.lock().expect("counter mutex poisoned");
                *count += 1;
            } // guard dropped BEFORE await
            sleep(Duration::from_millis(5)).await;
        }));
    }
    for handle in handles {
        handle.await?;
    }
    println!("Counter: {}", *counter.lock().expect("counter mutex poisoned")); // 4
    // Arc shares ownership, not permission for unchecked concurrent mutation.
    // Its clone is another handle to the SAME mutex/data, not a deep data copy.
    // A brief, low-contention synchronous critical section can use std::Mutex.
    // Its lock BLOCKS the thread if contended, so avoid long critical sections.
    // Do not retain its guard over await: that can make spawned futures !Send
    // and can deadlock even in cases where the compiler permits it.
    // Explicit braces make the lifetime clear. Async syntax does not require
    // replacing every standard synchronization primitive with an async variant.
    Ok(())
}

// =====================================================
// 07. Tokio mutex: waiting for a lock suspends the task
// =====================================================

async fn demo_async_mutex() -> Result<(), Box<dyn Error>> {
    println!("07. Tokio mutex: waiting for a lock suspends the task");

    let resource = AsyncMutex::new(String::new());
    let first = async {
        let mut guard = resource.lock().await;
        guard.push_str("A starts; ");
        sleep(Duration::from_millis(5)).await;
        guard.push_str("A ends; ");
    };
    let second = async {
        let mut guard = resource.lock().await;
        guard.push_str("B starts; ");
        sleep(Duration::from_millis(5)).await;
        guard.push_str("B ends; ");
    };
    tokio::join!(first, second);
    println!("Serialized access: {}", resource.lock().await);
    // Whichever branch acquires first keeps its start/end pair together.
    // Tokio's guard can span await; a contending task waits asynchronously.
    // Here join borrows the local mutex, so Arc is unnecessary.
    // The example models exclusive use of one async resource. For plain data,
    // prefer preparing work outside the lock and locking only for a short update.
    // Async mutexes cost more and do not make critical sections parallel.
    // They do NOT prevent logical deadlocks (for example, inconsistent lock order).
    // Unlike std::Mutex, Tokio's mutex does not poison after a panic: cancellation
    // or panic can still leave a partially updated value, which is your responsibility.
    Ok(())
}

// =====================================================
// 08. Semaphore: limit active operations, not spawned tasks
// =====================================================

async fn demo_semaphore_inside_tasks() -> Result<(), Box<dyn Error>> {
    println!("08. Semaphore: limit active operations, not spawned tasks");

    let slots = Arc::new(Semaphore::new(2));
    let mut handles = Vec::new();
    for id in 0..6 {
        let slots = Arc::clone(&slots);
        handles.push(tokio::spawn(async move {
            let _permit = slots.acquire_owned().await.expect("semaphore stays open");
            println!("Job {id}: entered limited operation");
            sleep(Duration::from_millis(5)).await;
            println!("Job {id}: leaving limited operation");
            // _permit drops here, returning one slot.
        }));
    }
    for handle in handles {
        handle.await?;
    }
    println!("Permits restored: {}", slots.available_permits()); // 2
    // SIX tasks are spawned; only TWO can hold permits simultaneously.
    // acquire_owned uses an Arc and returns a permit independent of a borrowed
    // semaphore lifetime, convenient for moving between spawned tasks.
    // The permit must live for the WHOLE operation: `let _ = ...` would drop
    // it immediately, unlike the named `_permit` binding used above.
    // This is a concurrency cap, not two jobs per second or two runtime threads.
    // Spawning a million tasks first can still consume excessive memory.
    Ok(())
}

// =====================================================
// 09. Acquire before spawning to slow down job submission
// =====================================================

async fn demo_acquire_before_spawn() -> Result<(), Box<dyn Error>> {
    println!("09. Acquire before spawning to slow down job submission");

    let slots = Arc::new(Semaphore::new(2));
    let mut handles = Vec::new();
    for id in 0..6 {
        let permit = Arc::clone(&slots).acquire_owned().await?;
        // Once permits run out, THIS producer waits before creating another task.
        handles.push(tokio::spawn(async move {
            let _permit = permit;
            println!("Admitted job {id}");
            sleep(Duration::from_millis(5)).await;
            id * id
        }));
    }
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await?);
    }
    println!("Results: {results:?}"); // [0, 1, 4, 9, 16, 25]
    // Completed jobs release permits so the producer can admit new jobs.
    // This avoids an unbounded population waiting INSIDE spawned tasks.
    // We still store every handle/result: total retained output memory is NOT
    // bounded. A streaming JoinSet design can also collect completed tasks as
    // submission proceeds; task-group management belongs in file 05.
    // Semaphores release permits on ordinary guard drop, including task cancellation.
    // They limit only the region during which the permit is actually held.
    Ok(())
}

// =====================================================
// 10. select! cancellation: receive safely; preserve unsent data
// =====================================================

async fn demo_select_and_channel_cancellation() -> Result<(), Box<dyn Error>> {
    println!("10. select! cancellation: receive safely; preserve unsent data");

    let (tx, mut rx) = mpsc::channel(1);
    tokio::select! {
        biased;
        message = rx.recv() => println!("Unexpected message: {message:?}"),
        _ = ready(()) => println!("Another branch won while receive was pending"),
    }
    tx.send(String::from("still deliverable")).await?;
    println!("Next receive: {:?}", rx.recv().await);
    // recv is cancellation-safe: losing a select branch did not consume a message.
    // Do not generalize this guarantee to every async operation.

    let payload = String::from("keep me unless sending is chosen");
    tx.send(String::from("occupies the only slot")).await?;
    tokio::select! {
        biased;
        permit = tx.reserve() => {
            permit?.send(payload);
        }
        _ = ready(()) => {
            println!("Reservation lost; caller retained payload: {payload}");
        }
    }
    println!("Original queued message: {:?}", rx.recv().await);
    drop(tx);
    println!("End of stream: {:?}", rx.recv().await); // None
    // Sending an OWNED payload with tx.send(payload) inside a losing branch
    // drops the send future AND its payload. No message is sent, but the caller
    // no longer owns that value. reserve waits for a slot without taking payload;
    // only the winning handler moves it into permit.send(...).
    // A reserved slot is not yet a queued message. Do not hold permits forever:
    // they occupy capacity and can delay a clean receiver drain after close().
    // Cancelling a reservation/lock acquisition also loses its queue position;
    // repeatedly restarting it can harm fairness or progress.

    println!();

    println!("mpsc: many requests/results, with optional bounded backpressure.");
    println!("oneshot: one reply. A state-owning task can avoid shared mutation.");
    println!("Arc: shared ownership. Mutex: exclusive access. Semaphore: limited permits.");
    // Bounded queue capacity limits buffered messages, not in-flight allocations
    // or total memory. Choose queue size and active-operation limits separately.
    // Await every task whose completion matters; closing a queue does not join it.
    // If ? returns early while other JoinHandles are owned, dropping those handles
    // detaches their tasks; later files cover robust group cancellation/cleanup.
    Ok(())
}
