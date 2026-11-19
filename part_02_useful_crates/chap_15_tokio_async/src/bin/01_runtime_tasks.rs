//! Tokio: futures, .await, runtimes, and tasks in small execution traces.
//!
//! Setup (inside chap_15_tokio_async):
//!     cargo add tokio@1 --features full
//! Run:
//!     cargo run --bin 01_runtime_tasks
//!
//! full keeps setup simple across this chapter; this file alone needs only
//! macros, rt, time, and sync. No trpl, network connection, or external service.
//!
//! Vocabulary to revisit while reading:
//! async fn / async block -> produces a Future, describing a computation.
//! Future                 -> something that can be polled toward one output.
//! .await                 -> drives a future as part of the current task;
//!                           that task may suspend if the future is pending.
//! Task                   -> independently scheduled asynchronous computation.
//! Runtime                -> executes tasks and supports timers and async I/O.
//! Thread                 -> OS execution resource on which tasks run.
//!
//! We use ONE execution thread for the async examples. Overlapping waits do
//! not require multiple CPU cores. Tokio can also use a multi-thread runtime.
//! A future need not ever finish, and an .await need not actually suspend.
//!
//! References:
//! https://tokio.rs/tokio/tutorial/spawning
//! https://docs.rs/tokio/latest/tokio/attr.main.html
//! https://docs.rs/tokio/latest/tokio/macro.join.html
//! https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html

use std::future::{ready, Future};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::task::JoinError;
use tokio::time::{sleep, Instant};

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), JoinError> {
    println!();

    demo_lazy_future().await;
    println!("\n============================\n");

    demo_ready_await().await;
    println!("\n============================\n");

    demo_async_blocks().await;
    println!("\n============================\n");

    demo_sequential_awaits().await;
    println!("\n============================\n");

    demo_join().await;
    println!("\n============================\n");

    demo_spawn().await?;
    println!("\n============================\n");

    demo_progress_before_handle_await().await?;
    println!("\n============================\n");

    demo_async_move().await?;
    println!("\n============================\n");

    demo_send_and_static().await?;
    println!("\n============================\n");

    demo_task_errors().await?;
    println!("\n============================\n");

    demo_multiple_tasks().await?;

    Ok(())
}

// #[tokio::main] expands the entry point into a synchronous function that creates
// a runtime and drives the async body with block_on. Conceptually:
// fn main() {
//     tokio::runtime::Builder::new_current_thread()
//         .enable_all().build().unwrap().block_on(async { /* body above */ });
// }
// Do not paste a second main into this file or nest block_on inside async code.
// current_thread schedules these async tasks on the calling thread. Without
// flavor="current_thread", the macro defaults to the multi-thread runtime
// (requires rt-multi-thread, included in full). A runtime may additionally use
// blocking threads for other APIs; we do not use those APIs in this first file.

// =====================================================
// 01. Calling an async function creates a lazy future
// =====================================================

async fn demo_lazy_future() {
    println!("01. Calling an async function creates a lazy future");

    async fn answer() -> u32 {
        println!("B. answer's body is running");
        42
    }

    let future = answer();
    println!("A. future created; answer's body has not run");
    let value = future.await;
    println!("C. answer returned {value}");
    // Expected order: A -> B -> C. Calling answer() does not enter its body.
    // Its declared output is u32, but answer() produces an anonymous type
    // implementing Future<Output=u32>. Awaiting it obtains the u32.
    // Future is a trait, not a single concrete 'future container' type.

    let unused = answer();
    drop(unused);
    println!("Dropped another unpolled answer; no second B line appeared.");
    // This applies to this async fn's body. An arbitrary API returning a future
    // may still do synchronous setup before returning that future.
}

// =====================================================
// 02. An await is a potential pause, not a forced switch
// =====================================================

async fn demo_ready_await() {
    println!("02. An await is a potential pause, not a forced switch");

    println!("Before an already-ready future");
    let value = ready(7_u32).await;
    println!("After ready: {value}");
    // ready(...) is a standard-library future that immediately yields its value.
    // No waiting is needed, so this await does not force a task switch.

    println!("Before a timer");
    sleep(Duration::from_millis(20)).await;
    println!("After the timer became ready");
    // If the timer is not ready, this task returns control to the runtime.
    // Tokio can run another runnable task instead of blocking this thread.
    // While suspended, this async computation is not executing a waiting loop.
    // The runtime is notified when progress may be possible and polls again.
    // It does not continually poll every pending future in a busy loop.
    // The timer duration is a minimum delay, not an exact scheduling guarantee.
}

// =====================================================
// 03. Async blocks and borrowed inputs do not spawn tasks
// =====================================================

async fn demo_async_blocks() {
    println!("03. Async blocks and borrowed inputs do not spawn tasks");

    let name = String::from("Long");
    let greeting = async {
        println!("Inside the async block");
        format!("Hello, {name}!")
    };
    println!("Async block created; its body has not run yet");
    println!("{}", greeting.await);
    println!("Borrowed name still usable: {name}");
    // async { ... } creates a future just like calling an async fn does.
    // This future borrows name. Directly awaiting it does not require 'static.
    // The block is part of the current task; async does not mean 'spawn'.

    async fn await_value<F: Future<Output=u32>>(future: F) -> u32 {
        future.await
    }
    println!("A generic function can await a future: {}", await_value(ready(9)).await);
    // You normally use .await, not manual poll/Pin/Waker implementations.
    // Those explain the machinery, but are not prerequisites for using Tokio.
}

// =====================================================
// 04. Consecutive awaits execute these operations sequentially
// =====================================================

async fn demo_sequential_awaits() {
    println!("04. Consecutive awaits execute these operations sequentially");

    let started = Instant::now();
    let first = labeled_wait("A", 30);
    let second = labeled_wait("B", 50);
    println!("Both futures created; neither labeled_wait body has run");

    let a = first.await;
    let b = second.await;
    println!("Results: {a}, {b}; elapsed={:?}", started.elapsed());
    // A starts -> A waits -> A finishes -> B starts -> B waits -> B finishes.
    // Expected total is approximately 30+50 ms, plus timer/scheduling overhead.
    // Merely storing two futures did not make them progress concurrently.
    // A's wait could let OTHER scheduled tasks run, but second is not scheduled
    // independently and this code has not reached its await yet.
}

// =====================================================
// 05. join! overlaps futures within the current task
// =====================================================

async fn demo_join() {
    println!("05. join! overlaps futures within the current task");

    let started = Instant::now();
    let (a, b) = tokio::join!(labeled_wait("A", 30), labeled_wait("B", 50));
    println!("Results in argument order: {a}, {b}; elapsed={:?}", started.elapsed());
    // Both start before their waits complete. A pending branch lets join poll
    // another branch. Approximate total is max(30, 50) ms rather than their sum.
    // These are observations to inspect, not timing assertions or benchmarks.
    // join! creates no independent Tokio tasks: it polls branches in THIS task.
    // The branches can borrow local data and have different concrete future types.
    // The macro includes awaiting; do not append .await to tokio::join!(...).
    // All branches finish before the tuple returns; tuple order is input order.
    // A long CPU loop or thread::sleep in one branch blocks progress of the others.
    // join! is concurrency, not parallel CPU execution, even on a multithread runtime.
}

// =====================================================
// 06. spawn() creates independently scheduled tasks
// =====================================================

async fn demo_spawn() -> Result<(), JoinError> {
    println!("06. spawn() creates independently scheduled tasks");

    let first = tokio::spawn(labeled_wait("task A", 30));
    let second = tokio::spawn(labeled_wait("task B", 50));
    println!("Both tasks submitted; main can do more async work");

    let a = first.await?;
    let b = second.await?;
    println!("Task outputs: {a}, {b}");
    // spawn hands the future to the runtime; a JoinHandle represents its outcome.
    // The tasks are scheduled without waiting for their handles to be awaited.
    // Awaiting first and then second does NOT serialize their already-spawned work.
    // Contrast section 04: those were unspawned operation futures.
    // On our current-thread runtime, tasks share a single execution thread.
    // spawn does not synchronously poll the submitted future. On this runtime,
    // tasks get a chance to run when the current task yields back to Tokio.
    // On a multi-thread runtime, another worker may start a task before the
    // spawning task reaches its next await. Never rely on task start order.
    // ? here propagates a JoinError, not an error returned by labeled_wait.
    Ok(())
}

// =====================================================
// 07. A task can finish before its handle is awaited
// =====================================================

async fn demo_progress_before_handle_await() -> Result<(), JoinError> {
    println!("07. A task can finish before its handle is awaited");

    let (tx, rx) = oneshot::channel();
    let handle = tokio::spawn(async move {
        sleep(Duration::from_millis(10)).await;
        println!("Worker: calculation complete");
        let _ = tx.send(());
        123_u32
    });

    // A one-message channel makes this ordering explicit, without guessing
    // how much time to sleep. Channels are explained fully in file 03.
    rx.await.expect("worker dropped its notification sender");
    println!("Caller: worker sent its completion notification; handle not awaited yet");
    println!("Caller: now retrieve output={}", handle.await?);
    // Awaiting rx yields so the worker can run. No await of handle was needed
    // to START its computation. Awaiting handle retrieves the final task outcome.
    // The notification proves work BEFORE send completed, not arbitrary work after it.
    // Dropping a JoinHandle detaches a task; it does not cancel that task.
    // Runtime shutdown can drop unfinished tasks, so retain/await their handles
    // when completion matters. Cancellation and shutdown get fuller treatment later.
    Ok(())
}

// =====================================================
// 08. async move transfers captured ownership into a future
// =====================================================

async fn demo_async_move() -> Result<(), JoinError> {
    println!("08. async move transfers captured ownership into a future");

    let name = String::from("Long");
    let task_name = name.clone();
    let handle = tokio::spawn(async move {
        sleep(Duration::from_millis(10)).await;
        format!("Hello from {task_name}")
    });
    println!("Caller kept its original: {name}");
    println!("{}", handle.await?);
    // The clone creates another owned String; async move captures that String.
    // Without a clone, moving name would make it unavailable in the caller.
    // Ownership transfers when constructing the async move block, even though
    // its body runs later. move itself does not spawn or execute the future.
    // Moving a borrowed reference only moves the reference; it does not turn
    // the borrowed object into owned data or extend its lifetime.
    Ok(())
}

// =====================================================
// 09. Send and 'static still apply on a current-thread runtime
// =====================================================

async fn demo_send_and_static() -> Result<(), JoinError> {
    println!("09. Send and 'static still apply on a current-thread runtime");

    let message = String::from("owned input can cross a task boundary");
    let handle = tokio::spawn(async move {
        sleep(Duration::from_millis(5)).await;
        message.len()
    });
    println!("Task returned byte length: {}", handle.await?);
    // tokio::spawn requires its future AND output to be Send + 'static.
    // Send permits transferring ownership between threads. A multithread runtime
    // may move suspended tasks between workers; spawn keeps the same API bounds
    // even when configured with current_thread.
    // 'static means no dependency on short-lived borrows, NOT 'live forever'.
    // Owned data can be dropped normally when its task is done.
    // Values retained across a suspension become part of the future's state.
    // Keeping Rc/RefCell across await can therefore make a spawned future !Send.

    println!();

    println!("Intentionally invalid patterns remain commented out:");
    // let name = String::from("borrowed local");
    // tokio::spawn(async { println!("{name}"); });
    // ERROR: the task borrows a local String; use ownership via async move.

    // let shared = std::rc::Rc::new(String::from("not Send"));
    // tokio::spawn(async move {
    //     sleep(Duration::from_millis(5)).await;
    //     println!("{shared}");
    // });
    // ERROR: Rc is retained across await and is not Send.
    // Arc can share suitable thread-safe data; it does not automatically make
    // every inner type thread-safe. LocalSet/spawn_local are specialized options
    // for !Send tasks, not something needed for these introductory examples.
    Ok(())
}

// =====================================================
// 10. Task failure and operation failure are separate layers
// =====================================================

async fn demo_task_errors() -> Result<(), JoinError> {
    println!("10. Task failure and operation failure are separate layers");

    async fn parse_count(text: &'static str) -> Result<u32, std::num::ParseIntError> {
        text.parse()
    }

    let handle = tokio::spawn(parse_count("42"));
    let outcome = handle.await;
    match outcome {
        Ok(Ok(value)) => println!("Task and operation succeeded: {value}"),
        Ok(Err(error)) => println!("Task finished but parsing failed: {error}"),
        Err(error) => println!("Task did not finish normally: {error}"),
    }
    // JoinHandle<Result<T,E>>.await -> Result<Result<T,E>, JoinError>.
    // Outer Err: task panic/cancellation. Inner Err: your operation's failure.

    println!();

    let handle = tokio::spawn(parse_count("not a number"));
    match handle.await? {
        Ok(value) => println!("Parsed: {value}"),
        Err(error) => println!("Expected parsing error: {error}"),
    }
    // Await alone does not unwrap either layer. In a suitably typed function,
    // handle.await?? can propagate both errors; here we explain them separately.
    // A spawned task's panic is reported by JoinError when unwinding is enabled;
    // it is not the same as returning Err. The panic hook may also print output.
    // We do not deliberately panic in this guide. panic=abort builds abort instead.
    // join! waits for all branches even if a branch returns Err; try_join! has
    // different early-return behavior, which must be considered with cancellation.
    Ok(())
}

// =====================================================
// 11. Spawn all tasks before waiting for their outputs
// =====================================================

async fn demo_multiple_tasks() -> Result<(), JoinError> {
    println!("11. Spawn all tasks before waiting for their outputs");

    let mut handles = Vec::new();
    for (id, milliseconds) in [(0_u32, 30_u64), (1, 10), (2, 20)] {
        handles.push(tokio::spawn(async move {
            sleep(Duration::from_millis(milliseconds)).await;
            println!("Task {id}: operation finished");
            (id, id * id)
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await?);
    }
    println!("Outputs gathered in submission order: {results:?}");
    // Tasks overlap their waits; gathering handles in a loop does not undo that.
    // Completion prints may differ in order; do not assert timer-based ordering.
    // Spawning and immediately awaiting INSIDE the same loop would serialize jobs.
    // This is a deliberately small fixed group, not a bounded-concurrency design.
    // If an await here returned JoinError early, ? would return and drop remaining
    // handles, detaching them. Robust group cleanup is covered later with JoinSet.

    println!();

    println!("async creates a future; await drives it within the current task.");
    println!("join! overlaps futures here; spawn schedules a separate task.");
    println!("Await handles when their results and completion matter.");
    // Wrapping CPU-heavy/blocking code in async does not make it nonblocking.
    // tokio::time::sleep suspends a task; std::thread::sleep blocks its thread.
    // Async waiting overlaps naturally; heavy CPU calculation still belongs on
    // an appropriate worker pool, such as Rayon, as discussed in the later files.
    Ok(())
}

// =====================================================
// Helper: one labeled asynchronous wait
// =====================================================

async fn labeled_wait(label: &'static str, milliseconds: u64) -> &'static str {
    println!("{label}: started");
    sleep(Duration::from_millis(milliseconds)).await;
    println!("{label}: finished");
    label
    // While this timer is pending, the enclosing task can yield to Tokio.
    // This models waiting; it does not simulate a CPU-intensive computation.
}
