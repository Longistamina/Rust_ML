//! Rayon + standard Rust synchronization and channels.
//!
//! Setup: cargo add rayon@1.12
//! Run:   cargo run --bin 05_shared_state_channels
//!
//! Rayon schedules CPU tasks; std supplies Arc, Mutex, atomics, and mpsc.
//! Prefer independent outputs and local accumulation before sharing mutable state.
//! This guide ends by writing small output files in a NEW temporary directory;
//! its path is printed, and the files are left there for inspection.
//!
//! References:
//! https://docs.rs/rayon/1.12.0/rayon/iter/trait.ParallelIterator.html
//! https://doc.rust-lang.org/std/sync/struct.Mutex.html
//! https://doc.rust-lang.org/std/sync/struct.Arc.html
//! https://doc.rust-lang.org/std/sync/atomic/index.html
//! https://doc.rust-lang.org/std/sync/mpsc/index.html

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::fs::{self, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc, Arc, Mutex,
};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!();

    demo_borrowed_mutex();
    println!("\n============================\n");

    demo_arc_ownership();
    println!("\n============================\n");

    demo_atomic_counter();
    println!("\n============================\n");

    demo_local_accumulation();
    println!("\n============================\n");

    demo_send_and_close()?;
    println!("\n============================\n");

    demo_multiple_senders();
    println!("\n============================\n");

    demo_try_recv_and_disconnect();
    println!("\n============================\n");

    demo_bounded_channel()?;
    println!("\n============================\n");

    demo_compute_and_write()?;
    println!("\n============================\n");

    demo_choose_a_pattern();

    Ok(())
}

// =====================================================
// 01. Borrow a Mutex when the operation waits for completion
// =====================================================

fn demo_borrowed_mutex() {
    println!("01. Borrow a Mutex when the operation waits for completion");

    let counter = Mutex::new(0_usize);
    (0_usize..100).into_par_iter().for_each(|_| {
        let mut guard = counter.lock().expect("counter mutex poisoned");
        *guard += 1;
    });
    println!("Counter: {}", *counter.lock().expect("counter mutex poisoned")); // 100
    // for_each completes before returning, so workers can borrow &counter.
    // Mutex provides synchronized interior mutability; Arc is not needed here.
    // The guard unlocks on drop. Keep its scope small: compute first, lock briefly.
    // expect is deliberate for this tiny demo, whose guarded code cannot normally
    // panic. A poisoned mutex requires an application-specific recovery policy.
    // Mutex prevents data races, not all logical errors or deadlocks.
}

// =====================================================
// 02. Arc shares ownership; Mutex protects the inner value
// =====================================================

fn demo_arc_ownership() {
    println!("02. Arc shares ownership; Mutex protects the inner value");

    let messages = Arc::new(Mutex::new(Vec::<String>::new()));
    rayon::scope(|scope| {
        for index in 0..4 {
            let messages = Arc::clone(&messages);
            scope.spawn(move |_| {
                let text = format!("job_{index}");
                messages.lock().expect("messages mutex poisoned").push(text);
            });
        }
    });

    let mut snapshot = messages.lock().expect("messages mutex poisoned").clone();
    snapshot.sort();
    println!("Collected messages, sorted afterward: {snapshot:?}");
    // Arc::clone clones a shared-ownership handle, not the Vec or its strings.
    // The insertion order depends on scheduling; a mutex does not establish
    // input order. We sort only this small snapshot to make reporting predictable.
    // Arc alone does not make arbitrary inner data safe to mutate concurrently.
    // These tasks own their handles. Ownership is useful when independently
    // stored/spawned tasks must retain data beyond a caller's borrowing scope.
    // Since THIS example uses scope, borrowing a plain Mutex would also work.
    // Do not add Arc to every Rayon closure merely because it runs in parallel.
}

// =====================================================
// 03. Atomics for an independent numeric counter
// =====================================================

fn demo_atomic_counter() {
    println!("03. Atomics for an independent numeric counter");

    let completed = AtomicUsize::new(0);
    let squares: Vec<usize> = (0_usize..100).into_par_iter()
        .map(|n| {
            let result = n * n;
            completed.fetch_add(1, Ordering::Relaxed);
            result
        })
        .collect();
    println!("Output values: {}", squares.len());
    println!("Completed operations: {}", completed.load(Ordering::Relaxed)); // 100
    // fetch_add atomically updates this counter; no Mutex guard is involved.
    // Relaxed is enough for this standalone statistic. collect provides the
    // completion boundary; we are NOT using the counter to publish other memory.
    // Do not use 'counter reached N' as a replacement for joining tasks or for
    // synchronizing unrelated data without a correct memory-ordering protocol.
    // An atomic increment is still shared contention. For just a final count,
    // prefer count() or the already available output length.
    // Even SeqCst would not turn several distinct atomic operations into a transaction.
}

// =====================================================
// 04. Reduce contention by accumulating locally first
// =====================================================

fn demo_local_accumulation() {
    println!("04. Reduce contention by accumulating locally first");

    let values: Vec<u64> = (0_u64..1_000).collect();
    let local_sum = values.par_iter()
        .fold(|| 0_u64, |sum, &value| sum + value)
        .reduce(|| 0_u64, |left, right| left + right);
    println!("Local accumulation plus merge: {local_sum}"); // 499500

    let published = Mutex::new(0_u64);
    values.par_chunks(100).for_each(|chunk| {
        let subtotal: u64 = chunk.iter().sum();
        *published.lock().expect("sum mutex poisoned") += subtotal;
    });
    println!("One lock per explicit chunk: {}", *published.lock().expect("sum mutex poisoned"));
    // First pattern has no shared lock; .sum::<u64>() is even simpler here.
    // Second pattern illustrates batching updates when a shared sink is necessary:
    // 10 chunks -> 10 lock acquisitions, rather than 1,000 element-wise locks.
    // Fold accumulators correspond to work subdivisions, not a fixed one per thread.
    // All arithmetic fits in u64 for this deliberately small input.
}

// =====================================================
// 05. Send results, then close the channel explicitly
// =====================================================

fn demo_send_and_close() -> Result<(), mpsc::SendError<(usize, i32)>> {
    println!("05. Send results, then close the channel explicitly");

    let numbers = [2_i32, 3, 8, 9];
    let (tx, rx) = mpsc::channel();
    numbers.par_iter().enumerate().try_for_each(|(index, &number)| {
        tx.send((index, number * number))
    })?;
    drop(tx);

    let mut results: Vec<(usize, i32)> = rx.into_iter().collect();
    println!("Channel arrival order: {results:?}");
    results.sort_by_key(|(index, _)| *index);
    println!("Restored input order: {results:?}");
    // channel() creates an unbounded queue: send does not wait for buffer space.
    // That is why this SMALL demo can produce all messages before consuming them.
    // try_for_each waits for the producer work. drop(tx) removes the last sender.
    // Receiver iteration drains queued messages, then ends once ALL senders are gone.
    // Without drop(tx), iteration would wait forever for a possible future message.
    // Arrival order is not input order across concurrent sends; carry IDs if needed.
    // This buffers all results, so ordinary map(...).collect() is simpler here.

    Ok(())
}

// =====================================================
// 06. Sender clones: every live sender keeps the stream open
// =====================================================

fn demo_multiple_senders() {
    println!("06. Sender clones: every live sender keeps the stream open");

    let (tx, rx) = mpsc::channel();
    rayon::scope(|scope| {
        for index in 0..4 {
            let worker_tx = tx.clone();
            scope.spawn(move |_| {
                let _ = worker_tx.send(format!("result from task {index}"));
                // worker_tx is dropped when this task finishes.
            });
        }
        drop(tx); // the original handle must be dropped too
    }); // all tasks finished, so all their cloned handles have now been dropped

    for message in rx {
        println!("Received: {message}");
    }
    // Cloning a Sender does not create another channel; all handles feed one queue.
    // mpsc means multiple producers, ONE consumer. std Receiver is not cloneable.
    // No stop sentinel is needed for this result stream: sender lifetime signals EOF.
    // A per-producer send sequence is preserved; different producers can interleave.
    // Closing the stream says nothing about whether every job succeeded: for that,
    // send Result values, verify expected IDs/counts, and wait for the producers.
}

// =====================================================
// 07. Empty is different from disconnected
// =====================================================

fn demo_try_recv_and_disconnect() {
    println!("07. Empty is different from disconnected");

    let (tx, rx) = mpsc::channel::<i32>();
    match rx.try_recv() {
        Err(mpsc::TryRecvError::Empty) => println!("Empty: a sender still exists; more may arrive."),
        other => println!("Unexpected state: {other:?}"),
    }
    tx.send(42).expect("receiver is still alive");
    drop(tx);
    println!("Queued message survives sender drop: {:?}", rx.try_recv()); // Ok(42)
    println!("After draining, disconnected: {:?}", rx.try_recv());
    // recv blocks while empty with live senders; try_recv returns immediately.
    // Repeatedly polling try_recv in a tight loop wastes CPU. Prefer blocking recv
    // on an appropriate non-pool consumer thread, or an explicit timeout policy.

    println!();

    let (tx, rx) = mpsc::channel::<String>();
    drop(rx);
    match tx.send(String::from("unsent payload")) {
        Ok(()) => println!("Unexpected successful send"),
        Err(error) => println!("Expected disconnected receiver; recovered {:?}", error.0),
    }
    // SendError<T> contains the unsent T. A dropped receiver is not a worker panic;
    // it may simply mean the consumer stopped early.
}

// =====================================================
// 08. Bounded queues need a consumer running concurrently
// =====================================================

fn demo_bounded_channel() -> Result<(), Box<dyn std::error::Error>> {
    println!("08. Bounded queues need a consumer running concurrently");

    let pool = ThreadPoolBuilder::new().num_threads(1).build()?;
    let (tx, rx) = mpsc::sync_channel(2);
    let consumer = thread::spawn(move || rx.into_iter().collect::<Vec<(usize, usize)>>());

    let produced = pool.install(|| {
        (0_usize..8).into_par_iter().try_for_each_with(tx, |sender, index| {
            sender.send((index, index * index))
        })
    });
    // The final sender clone disappears when the parallel operation ends.
    // The consumer drains while production runs, even with only ONE Rayon worker.
    let results = consumer.join().map_err(|_| io::Error::other("consumer thread panicked"))?;
    produced?;
    println!("Bounded-stream results: {results:?}");
    // sync_channel(2) buffers at most two messages; send waits when that queue is full.
    // This waiting is backpressure. It bounds QUEUED messages, not all memory:
    // in-flight computations and this collected output Vec still occupy memory.
    // The consumer is an independent std thread, not a sibling Rayon job.
    // Do not move consumer creation after install: production could fill the queue
    // and block before execution reaches the receiver. Capacity zero is a valid
    // rendezvous channel, but likewise needs a live concurrent consumer.
    Ok(())
}

// =====================================================
// 09. Practical pipeline: compute in Rayon, write on one thread
// =====================================================

fn demo_compute_and_write() -> Result<(), Box<dyn std::error::Error>> {
    println!("09. Practical pipeline: compute in Rayon, write on one thread");

    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let directory = std::env::temp_dir().join(format!("rayon_results_{}_{stamp}", std::process::id()));
    fs::create_dir(&directory)?; // error on a collision; do not reuse an existing directory
    println!("Output directory: {}", directory.display());

    let writer_directory = directory.clone();
    let (tx, rx) = mpsc::sync_channel::<(usize, Vec<u64>)>(2);
    let writer = thread::spawn(move || -> io::Result<usize> {
        let mut written = 0;
        for (index, squares) in rx {
            let path = writer_directory.join(format!("block_{index:02}.txt"));
            let file = OpenOptions::new().write(true).create_new(true).open(path)?;
            let mut output = BufWriter::new(file);
            for value in squares {
                writeln!(output, "{value}")?;
            }
            output.flush()?;
            written += 1;
        }
        Ok(written)
    });

    let blocks = [vec![1_u64, 2, 3], vec![10, 20], vec![100, 200]];
    let pool = ThreadPoolBuilder::new().num_threads(2);
    // Keep producer failures as a value so the writer is always joined below,
    // including when pool construction fails. Dropping tx lets the writer exit.
    let production: Result<(), Box<dyn std::error::Error>> = (|| {
        let pool = pool.build()?;
        pool.install(|| {
            blocks.par_iter().enumerate().try_for_each_with(tx, |sender, (index, block)| {
                let squares = block.iter().map(|&n| n * n).collect();
                sender.send((index, squares))
            })
        })?;
        Ok(())
    })();

    let writing = writer.join().map_err(|_| io::Error::other("writer thread panicked"))?;
    // Join BEFORE propagating producer errors so this function does not leave a
    // background writer running after returning. Prefer the original I/O error
    // over a secondary send error caused by the receiver dropping.
    let written = writing?;
    production?;
    if written != blocks.len() {
        return Err(io::Error::other("not every block was written").into());
    }
    println!("Wrote {written} files; example block_00.txt contains:");
    print!("{}", fs::read_to_string(directory.join("block_00.txt"))?); // 1, 4, 9
    // Rayon computes owned outputs; send transfers ownership to one writer.
    // File names carry input IDs, regardless of completion/write order.
    // The writer flushes explicitly so buffered write errors are reported.
    // On an I/O error, it returns and drops rx; blocked/future sends then fail.
    // Already written files remain: this pipeline is NOT transactional.
    // The sample arithmetic is tiny; the same structure fits heavier CPU transforms.
    // A slow writer can throttle producers. One thread avoids competing writes,
    // but does not guarantee maximum disk throughput for every workload.
    // Files are deliberately left in the printed directory for you to inspect.
    Ok(())
}

// =====================================================
// 10. Choose the smallest amount of coordination needed
// =====================================================

fn demo_choose_a_pattern() {
    println!("10. Choose the smallest amount of coordination needed");

    println!("Independent outputs: map + collect, or disjoint mutable slices.");
    println!("Final aggregate: local fold + reduce, or a built-in reduction.");
    println!("Shared scalar statistic: an atomic when appropriate.");
    println!("Shared compound invariant: Mutex; add Arc for shared ownership.");
    println!("Transfer results: a channel with explicit lifetime/error handling.");
    // Never block a Rayon task waiting for another queued Rayon task that may
    // need the same worker. Such dependencies can deadlock a small pool.
    // Do not hold a mutex while waiting for child work that needs that mutex.
    // A channel's buffer capacity is not the number of workers or CPU cores.
    // Sender closure, work completion, and successful processing are distinct:
    // drop senders for EOF, wait for tasks, and inspect Result/counts for success.
    // The bounded examples place the consumer outside the Rayon pool so it can
    // drain the queue independently. send can still block, which is a tradeoff.
    // For the later dataloader, bounded buffering will carry prepared batches
    // between a background producer and a training consumer. No Tokio required.
}
