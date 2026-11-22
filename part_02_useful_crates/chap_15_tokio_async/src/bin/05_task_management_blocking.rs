//! Tokio: manage task groups, isolate blocking work, and finish cleanly.
//!
//! Setup (inside chap_15_tokio_async):
//!     cargo add tokio@1 --features full
//!     cargo add rayon@1.12
//!     cargo add tokio-util@0.7 --features rt
//! Run: cargo run --bin 05_task_management_blocking
//!
//! tokio-util supplies CancellationToken; rayon supplies the CPU pool example.
//! Review: async tasks cooperate on runtime threads. A long synchronous call
//! prevents that thread from polling other tasks until the call returns.
//! Awaiting a task's handle waits for its outcome; it does not start the task.
//!
//! Examples finish automatically, use small bounded workloads, and need no
//! internet, files, Ctrl+C, or deliberate panics.
//!
//! References:
//! https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html
//! https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
//! https://tokio.rs/tokio/topics/shutdown
//! https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html
//! https://docs.rs/rayon/1.12.0/rayon/struct.ThreadPool.html

use rayon::prelude::*;
use std::error::Error;
use std::future::pending;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!();

    demo_join_set().await;
    println!("\n============================\n");

    demo_group_errors().await;
    println!("\n============================\n");

    demo_bounded_submission().await?;
    println!("\n============================\n");

    demo_spawn_blocking().await?;
    println!("\n============================\n");

    demo_started_blocking_abort().await?;
    println!("\n============================\n");

    demo_rayon_bridge().await?;
    println!("\n============================\n");

    demo_cancellation_token().await?;
    println!("\n============================\n");

    demo_shutdown_deadline().await;
    println!("\n============================\n");

    demo_failure_shutdown().await;

    Ok(())
}

// =====================================================
// 01. JoinSet tracks tasks and yields completed outcomes
// =====================================================

async fn demo_join_set() {
    println!("01. JoinSet tracks tasks and yields completed outcomes");

    let mut tasks = JoinSet::new();
    for (id, delay) in [(0_u32, 15_u64), (1, 5), (2, 10)] {
        tasks.spawn(async move {
            sleep(Duration::from_millis(delay)).await;
            (id, id * id)
        });
    }
    println!("Tracked tasks: {}", tasks.len());
    while let Some(outcome) = tasks.join_next().await {
        match outcome {
            Ok((id, square)) => println!("Completed job {id}: {square}"),
            Err(error) => println!("Task failed to finish normally: {error}"),
        }
    }
    println!("Set empty: {}", tasks.is_empty());
    // join_next -> Option<Result<T, JoinError>>. None means no tasks remain.
    // Results arrive by completion rather than submission order; do not assert
    // exact timer-based ordering. Carry job IDs when output order matters.
    // All tasks share output type T. Store differing outputs in an enum if needed.
    // Dropping a JoinSet requests abortion of its tracked tasks, unlike dropping
    // an ordinary JoinHandle (which detaches its task). Drop cannot await cleanup.
    // Drain the set when termination must be observed. len includes completed
    // tasks whose results have not yet been removed, not just currently running ones.
}

// =====================================================
// 02. Inspect operation errors and abort/drain remaining tasks
// =====================================================

async fn demo_group_errors() {
    println!("02. Inspect operation errors and abort/drain remaining tasks");

    let mut tasks: JoinSet<Result<u32, &'static str>> = JoinSet::new();
    tasks.spawn(async { Err("invalid job input") });
    tasks.spawn(async { pending::<Result<u32, &'static str>>().await });
    // The pending sibling ensures there is outstanding work when failure arrives.

    println!();

    while let Some(outcome) = tasks.join_next().await {
        match outcome {
            Ok(Ok(value)) => println!("Job succeeded: {value}"),
            Ok(Err(error)) => {
                println!("Expected operation failure: {error}");
                tasks.abort_all();
                break;
            }
            Err(error) => {
                println!("Task panic/cancellation: {error}");
                tasks.abort_all();
                break;
            }
        }
    }
    while let Some(outcome) = tasks.join_next().await {
        println!("Drained remaining outcome: {outcome:?}");
    }
    // abort_all requests cancellation but does NOT remove tasks or wait for them.
    // Draining observes their termination, including cancelled outcomes.
    // Outer Err is JoinError; inner Err is the operation's own error.
    // A naive ? on the first error can skip your intended reporting/cleanup policy.
    // JoinSet::shutdown().await is a shorter abort-and-drain option when you
    // intentionally do not need each remaining outcome (it ignores task panics).
}

// =====================================================
// 03. Bound the set while submitting and collecting work
// =====================================================

async fn demo_bounded_submission() -> Result<(), Box<dyn Error>> {
    println!("03. Bound the set while submitting and collecting work");

    const LIMIT: usize = 2;
    let mut jobs = 0_u32..8;
    let mut tasks = JoinSet::new();
    let mut completed = 0;
    loop {
        while tasks.len() < LIMIT {
            let Some(id) = jobs.next() else { break };
            tasks.spawn(async move {
                sleep(Duration::from_millis(5)).await;
                (id, id * id)
            });
        }
        let Some(outcome) = tasks.join_next().await else { break };
        match outcome {
            Ok((id, value)) => {
                println!("Collected job {id}: {value}");
                completed += 1;
            }
            Err(error) => {
                tasks.shutdown().await;
                return Err(error.into());
            }
        }
    }
    println!("Completed {completed} jobs; at most {LIMIT} tracked at once");
    // Fill a small window -> collect one completion -> admit another job.
    // Unlike spawning all jobs before semaphore acquisition, this bounds the
    // population of task records, including results waiting to be collected.
    // We print results incrementally; collecting every output in a Vec would
    // still require memory proportional to the total output size.
    // The source here is a lazy range, not a materialized million-job input Vec.
    Ok(())
}

// =====================================================
// 04. spawn_blocking isolates a synchronous operation
// =====================================================

async fn demo_spawn_blocking() -> Result<(), Box<dyn Error>> {
    println!("04. spawn_blocking isolates a synchronous operation");

    let input = String::from("10,20,30");
    let handle = tokio::task::spawn_blocking(move || {
        // Stand-in for a synchronous library API. This closure is NOT async.
        input.split(',').map(str::parse::<u32>).collect::<Result<Vec<_>, _>>()
    });
    println!("Async caller can await other operations before retrieving the result");
    let parsed = handle.await??;
    println!("Synchronous parse returned: {parsed:?}");
    // FnOnce + Send + 'static -> output Send + 'static, like other owned task APIs.
    // Awaiting this handle suspends the async caller; the closure occupies a
    // blocking-pool thread, separate from async runtime execution threads.
    // This tiny parse is only an API illustration; do not offload every cheap call.
    // Use it for bounded synchronous work that would otherwise block for too long.
    // Many long-lived blocking loops should use dedicated std threads instead.
    // The blocking pool has a large default thread limit and queues excess jobs;
    // it is NOT a CPU parallelism budget. Limit CPU jobs or use Rayon.
    Ok(())
}

// =====================================================
// 05. Started blocking work cannot be stopped with abort()
// =====================================================

async fn demo_started_blocking_abort() -> Result<(), Box<dyn Error>> {
    println!("05. Started blocking work cannot be stopped with abort()");

    let (started_tx, started_rx) = oneshot::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel::<()>();
    let handle = tokio::task::spawn_blocking(move || -> Result<u32, String> {
        let _ = started_tx.send(());
        // Blocking recv is intentional HERE, on a blocking-pool thread.
        release_rx.recv().map_err(|error| error.to_string())?;
        Ok(7)
    });
    started_rx.await?; // ensures closure has STARTED before abort is requested
    handle.abort();
    println!("Abort requested after blocking closure started");
    release_tx.send(())?; // let the bounded demonstration finish explicitly
    println!("Closure still returned: {}", handle.await??); // 7
    // An unstarted queued blocking task may be cancelled, but started work runs on.
    // Timing out/dropping the handle likewise does not stop that computation.
    // Runtime shutdown normally waits for started blocking work. A runtime's
    // shutdown timeout stops waiting; it does not kill the underlying thread/job.
    // For interruptible blocking algorithms, add an explicit stop flag/token
    // checked at safe boundaries, and make any blocking waits interruptible too.
    // The release message here is deterministic coordination, not a sleep guessing
    // how much time was sufficient for the closure to start.
    Ok(())
}

// =====================================================
// 06. Bridge to Rayon without blocking the async thread
// =====================================================

async fn demo_rayon_bridge() -> Result<(), Box<dyn Error>> {
    println!("06. Bridge to Rayon without blocking the async thread");

    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build()?;
    let numbers: Vec<u64> = (1_u64..=10_000).collect();
    let (tx, rx) = oneshot::channel();
    pool.spawn(move || {
        let result = catch_unwind(AssertUnwindSafe(|| {
            numbers.par_iter().map(|&n| n * n).sum::<u64>()
        }))
        .map_err(|_| String::from("Rayon computation panicked"));
        let _ = tx.send(result);
    });
    let total = rx.await??;
    println!("Sum of squares computed by Rayon: {total}"); // 333383335000
    // Rayon computes on its own pool; Tokio awaits a one-message result channel.
    // Calling pool.install(expensive_work) DIRECTLY on this async thread would
    // synchronously wait there. The spawn + oneshot bridge avoids that wait.
    // In an application, construct and reuse the Rayon pool rather than rebuilding
    // it per request. Its startup here is configuration, not a benchmark.
    // catch_unwind turns an unwinding panic into a reply error for this detached
    // Rayon job. A panic hook may still print; panic=abort cannot be caught.
    // Ordinary expected failures should be explicit Result values from the work.
    // If the async caller is cancelled, rx drops but Rayon work does NOT stop.
    // Its eventual send fails harmlessly; add cooperative CPU cancellation if needed.
    // Limit work BEFORE submitting many jobs, and keep any admission permit owned
    // by the Rayon job until it truly finishes, not just by the waiting async caller.
    // The small calculation here cannot overflow u64; larger workloads need a policy.
    Ok(())
}

// =====================================================
// 07. One CancellationToken can notify multiple workers
// =====================================================

async fn demo_cancellation_token() -> Result<(), Box<dyn Error>> {
    println!("07. One CancellationToken can notify multiple workers");

    let stop = CancellationToken::new();
    let mut workers = JoinSet::new();
    for id in 0..3 {
        let stop = stop.clone();
        workers.spawn(async move {
            stop.cancelled().await;
            println!("Worker {id}: stop observed; cleaning up");
            sleep(Duration::from_millis(5)).await;
            id
        });
    }
    println!("Coordinator: request stop for all workers");
    stop.cancel();
    while let Some(outcome) = workers.join_next().await {
        println!("Cleanly stopped worker {}", outcome?);
    }
    // cancel is a signal, not task abortion: workers decide how/when to react.
    // A clone observes the same cancellation; it is not a fresh independent token.
    // Cancellation is remembered, so workers starting after cancel still see it.
    // child_token permits one-way parent-to-child cancellation propagation.
    // No Arc wrapper is needed around CancellationToken; clone shares its state.
    // Request stop -> workers perform async cleanup -> join them to observe completion.
    // Dropping an ordinary token alone is not the same as calling cancel().
    Ok(())
}

// =====================================================
// 08. Graceful shutdown with a deadline and abort fallback
// =====================================================

async fn demo_shutdown_deadline() {
    println!("08. Graceful shutdown with a deadline and abort fallback");

    let stop = CancellationToken::new();
    let mut workers = JoinSet::new();
    let cooperative_stop = stop.clone();
    workers.spawn(async move {
        cooperative_stop.cancelled().await;
        sleep(Duration::from_millis(5)).await; // represent asynchronous cleanup
        "cooperative worker cleaned up"
    });
    workers.spawn(async { pending::<&'static str>().await }); // ignores the stop signal

    stop.cancel();
    let graceful = timeout(Duration::from_millis(30), async {
        while let Some(outcome) = workers.join_next().await {
            println!("Grace-period outcome: {outcome:?}");
        }
    }).await;

    println!();

    if graceful.is_err() {
        println!("Expected grace deadline: one worker ignores cooperative stop");
        workers.abort_all();
        while let Some(outcome) = workers.join_next().await {
            println!("Fallback drained: {outcome:?}");
        }
    }
    // timeout borrowed the set through this async block. Expiration dropped that
    // waiting future, NOT the set or all its tasks; we still own workers afterward.
    // Async tasks suspended in pending() can be aborted and drained here.
    // This fallback cannot impose a hard deadline on non-yielding synchronous
    // code or started spawn_blocking/Rayon work. The program must design those
    // operations to finish or cooperate separately.
    // Small timer values keep the example quick, not exact real-time guarantees.
}

// =====================================================
// 09. Practical coordinator: failure requests sibling cleanup
// =====================================================

async fn demo_failure_shutdown() {
    println!("09. Practical coordinator: failure requests sibling cleanup");

    let stop = CancellationToken::new();
    let mut workers: JoinSet<Result<&'static str, &'static str>> = JoinSet::new();
    for _ in 0..2 {
        let stop = stop.clone();
        workers.spawn(async move {
            tokio::select! {
                _ = stop.cancelled() => {
                    // Graceful path: finish cleanup rather than just dropping state.
                    sleep(Duration::from_millis(5)).await;
                    Ok("sibling cleaned up after stop")
                }
                _ = pending::<()>() => Ok("unreachable normal completion"),
            }
        });
    }
    workers.spawn(async { Err("simulated service failure") });

    println!();

    let mut first_failure = None;
    while let Some(outcome) = workers.join_next().await {
        match outcome {
            Ok(Ok(message)) => println!("Worker outcome: {message}"),
            Ok(Err(error)) => {
                println!("Expected service error: {error}");
                if first_failure.is_none() {
                    first_failure = Some(error.to_owned());
                }
                stop.cancel();
            }
            Err(error) => {
                println!("Unexpected task failure: {error}");
                if first_failure.is_none() {
                    first_failure = Some(error.to_string());
                }
                stop.cancel();
            }
        }
    }
    println!("All tracked workers finished; original failure={first_failure:?}");
    // We record failure, notify siblings, and CONTINUE draining instead of
    // returning early and skipping their asynchronous cleanup.
    // This finite demo's siblings always cooperate. A real service can combine
    // this policy with the grace deadline from section 08.
    // For a queue-driven service: stop admission, choose whether to drain queued
    // requests, notify workers, then await the workers and any result writer.
    // Ctrl+C is one possible trigger (tokio::signal::ctrl_c().await); here a
    // deterministic internal failure triggers shutdown without user intervention.

    println!();

    println!("JoinSet: own and observe a dynamic task group.");
    println!("spawn_blocking: bounded synchronous work off async execution threads.");
    println!("Rayon: CPU parallelism; oneshot: asynchronously await its result.");
    println!("Graceful stop: notify, clean up, join; use an explicit fallback policy.");
    // A stopped async waiter does not imply stopped blocking/CPU/external work.
    // That boundary is the key to reliable resource limits and shutdown.
}
