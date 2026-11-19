//! Tokio: timers, racing futures, time limits, and stopping work.
//!
//! Setup: cargo add tokio@1 --features full
//! Cargo.toml: tokio = { version="1", features=["full"] }
//! Run: cargo run --bin 02_time_select_cancellation
//!
//! Review: async creates a future; awaiting it may suspend the current task.
//! join! waits for all branches; select! chooses a matching ready branch.
//! Dropping an owned unfinished future cancels that future's further execution.
//! Dropping a JoinHandle only detaches its separately scheduled task.
//!
//! Examples finish automatically. No network, file writes, or Ctrl+C required.
//! Timer durations illustrate behavior, not exact wall-clock guarantees.
//!
//! References:
//! https://docs.rs/tokio/latest/tokio/time/index.html
//! https://docs.rs/tokio/latest/tokio/macro.select.html
//! https://docs.rs/tokio/latest/tokio/time/fn.timeout.html
//! https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html
//! https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html

use std::future::{pending, ready};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::task::{yield_now, JoinError};
use tokio::time::{interval, sleep, sleep_until, timeout, timeout_at, Instant, MissedTickBehavior};

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), JoinError> {
    println!();

    demo_sleep_and_deadline().await;
    println!("\n============================\n");

    demo_interval().await;
    println!("\n============================\n");

    demo_yield_now().await;
    println!("\n============================\n");

    demo_timeout().await;
    println!("\n============================\n");

    demo_select().await;
    println!("\n============================\n");

    demo_drop_cancellation().await;
    println!("\n============================\n");

    demo_keep_future_across_select().await;
    println!("\n============================\n");

    demo_timeout_and_task_handles().await?;
    println!("\n============================\n");

    demo_abort_task().await;
    println!("\n============================\n");

    demo_cooperative_shutdown().await?;
    println!("\n============================\n");

    demo_one_overall_deadline().await;

    Ok(())
}

// =====================================================
// 01. sleep waits for a duration; sleep_until uses a deadline
// =====================================================

async fn demo_sleep_and_deadline() {
    println!("01. sleep waits for a duration; sleep_until uses a deadline");

    let started = Instant::now();
    sleep(Duration::from_millis(10)).await;
    println!("First wait completed after {:?}", started.elapsed());

    let deadline = Instant::now() + Duration::from_millis(20);
    sleep(Duration::from_millis(5)).await; // other asynchronous preparation
    sleep_until(deadline).await;
    println!("Reached the fixed deadline");
    // sleep(D) sets a deadline D from when the Sleep is created.
    // sleep_until(T) refers to one absolute Tokio Instant, regardless of when
    // you later await it. An already-past deadline needs no further delay.
    // These are monotonic scheduling times, not calendar dates or Jiff timestamps.
    // A pending timer lets this task yield; std::thread::sleep blocks the thread.
    // Creating Sleep needs Tokio's timer context. The main macro enables it here.
    // Timers can run late due to scheduling/load; do not assert exact milliseconds.
}

// =====================================================
// 02. interval follows a schedule, not work-then-sleep timing
// =====================================================

async fn demo_interval() {
    println!("02. interval follows a schedule, not work-then-sleep timing");

    let started = Instant::now();
    let mut ticker = interval(Duration::from_millis(15));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
    for index in 0..3 {
        let scheduled = ticker.tick().await;
        println!("tick {index}: scheduled={:?}, observed={:?}",
            scheduled.duration_since(started), started.elapsed());
    }
    // The FIRST tick is ready immediately; later ticks follow the period.
    // The returned Instant is the scheduled tick time, not necessarily now.
    // A sleep(period) after each piece of work instead adds work time to the spacing.
    // interval_at can choose a first tick in the future. Period must be nonzero.
    // MissedTickBehavior controls what happens if your code falls behind:
    // Burst (default): quickly deliver overdue ticks to catch up.
    // Skip: skip missed slots and retain the original schedule's alignment.
    // Delay: shift future spacing relative to when the delayed tick is observed.
    // None of these automatically spawns a new task per tick. Overlapping jobs
    // require an explicit design and concurrency limit.
}

// =====================================================
// 03. yield_now offers a scheduling opportunity
// =====================================================

async fn demo_yield_now() {
    println!("03. yield_now offers a scheduling opportunity");

    let first = async {
        for step in 0..3 {
            println!("A: small step {step}");
            yield_now().await;
        }
    };
    let second = async {
        for step in 0..3 {
            println!("B: small step {step}");
            yield_now().await;
        }
    };
    tokio::join!(first, second);
    // yield_now requeues the task when its yield reaches the scheduler; a
    // surrounding combinator can affect whether that yield propagates outward.
    // It does NOT guarantee that a particular other task runs next, or that
    // every resource/timer is checked before this task resumes.
    // Ready awaits need not yield at all. Do not depend on A/B alternating.
    // This illustrates cooperation, not a way to turn heavy CPU work into async I/O.
    // Move sustained CPU work to Rayon; blocking adapters are covered later.
}

// =====================================================
// 04. timeout returns Result<Output, Elapsed>
// =====================================================

async fn demo_timeout() {
    println!("04. timeout returns Result<Output, Elapsed>");

    let immediate = timeout(Duration::from_millis(20), ready(42_u32)).await;
    println!("Ready operation: {immediate:?}"); // Ok(42)

    let operation = async { "bad".parse::<u32>() };
    match timeout(Duration::from_millis(20), operation).await {
        Ok(Ok(value)) => println!("Value: {value}"),
        Ok(Err(error)) => println!("Operation finished with a parse error: {error}"),
        Err(error) => println!("Time limit elapsed: {error}"),
    }
    // Just like a task handle around Result, timeout adds its own error layer.
    // An operation's Err does not mean its time limit expired.

    println!();

    match timeout(Duration::from_millis(10), pending::<()>()).await {
        Ok(()) => println!("Unexpected completion"),
        Err(error) => println!("Expected elapsed timeout: {error}"),
    }
    // pending() never becomes ready, so expiration does not rely on a close race
    // between two short timers. The runtime suspends rather than busy-polling it.
    // A timeout is NOT preemptive: code that never yields can overrun its limit.
    // It cannot interrupt a CPU loop or blocking call in the middle of execution.
}

// =====================================================
// 05. select! chooses one matching ready branch
// =====================================================

async fn demo_select() {
    println!("05. select! chooses one matching ready branch");

    let winner = tokio::select! {
        value = ready("ready result") => value,
        _ = pending::<()>() => "never ready",
    };
    println!("Winner: {winner}");
    // Syntax: pattern = future_expression => handler_expression.
    // select polls branches concurrently INSIDE the current task; it does not
    // spawn tasks or execute CPU-heavy branches simultaneously.
    // The handler's value becomes the value of the whole select expression.
    // Like join!, this macro includes waiting: do not add .await afterward.

    let winner = tokio::select! {
        biased;
        value = ready("first") => value,
        value = ready("second") => value,
    };
    println!("Both ready with biased priority: {winner}"); // first
    // Default select randomizes the first branch checked. It is not a guarantee
    // of which operation finished first by wall clock when several are ready.
    // biased checks top to bottom; an always-ready early branch can starve others.
    // A nonmatching output pattern disables that branch for this select call.
    // else runs only if ALL branches are disabled, not merely while they wait.
    // With all branches disabled and no else, select panics.
}

// =====================================================
// 06. Losing owned futures are dropped, not rolled back
// =====================================================

async fn demo_drop_cancellation() {
    println!("06. Losing owned futures are dropped, not rolled back");

    let mut events = Vec::new();
    let unfinished = async {
        let _notice = DropNotice("owned unfinished operation");
        events.push("side effect before waiting");
        pending::<()>().await;
        events.push("unreachable after cancellation");
    };

    tokio::select! {
        biased;
        _ = unfinished => println!("Unexpected completion"),
        _ = sleep(Duration::from_millis(10)) => println!("Timer branch selected"),
    }
    println!("Retained side effects: {events:?}");
    // biased ensures the first branch is polled to its pending state before
    // checking the timer. When it loses, select drops its OWNED unfinished future.
    // The DropNotice prints during destruction, illustrating synchronous cleanup.
    // The Vec mutation remains; cancellation does not undo earlier actions.
    // Dropping a future cannot run arbitrary asynchronous cleanup with .await.
    // Nor does local cancellation guarantee a remote request/transaction was undone.
}

// =====================================================
// 07. Keep one future alive across repeated select calls
// =====================================================

async fn demo_keep_future_across_select() {
    println!("07. Keep one future alive across repeated select calls");

    let operation = async {
        println!("Operation starts once");
        sleep(Duration::from_millis(35)).await;
        99_u32
    };
    tokio::pin!(operation);
    let mut ticker = interval(Duration::from_millis(10));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            result = &mut operation => {
                println!("Operation completed: {result}");
                break;
            }
            _ = ticker.tick() => println!("Progress tick while operation is pending"),
        }
    }
    // select receives a temporary mutable reference, not ownership of operation.
    // Losing drops that reference; the pinned future outside the loop survives.
    // pin! keeps a potentially !Unpin future at a stable location while polled.
    // You only need this small pinning pattern here, not a manual Future impl.
    // Recreating the operation inside every loop iteration would cancel/restart
    // it whenever another branch wins. That can prevent progress entirely.
    // Break when it finishes: do not poll this completed future again.
    // Cancellation safety means dropping/recreating an operation does not lose
    // required progress. It depends on the particular API; consult its docs.
}

// =====================================================
// 08. Timing out a JoinHandle does not cancel its task
// =====================================================

async fn demo_timeout_and_task_handles() -> Result<(), JoinError> {
    println!("08. Timing out a JoinHandle does not cancel its task");

    let (release_tx, release_rx) = oneshot::channel::<()>();
    let (finished_tx, finished_rx) = oneshot::channel();
    let handle = tokio::spawn(async move {
        let _ = release_rx.await;
        println!("Detached task: release received; computation completes");
        let _ = finished_tx.send(123_u32);
    });

    match timeout(Duration::from_millis(10), handle).await {
        Ok(result) => result?,
        Err(_) => println!("Expected timeout: owned JoinHandle dropped; task still exists"),
    }
    // No release has been sent, so the task could not have completed normally.
    // The timeout owned ONLY its handle, not the separately spawned task itself.
    let _ = release_tx.send(());
    println!("Detached task still delivered: {:?}", finished_rx.await);

    println!();

    let mut handle = tokio::spawn(pending::<u32>());
    if timeout(Duration::from_millis(10), &mut handle).await.is_err() {
        println!("Borrowed handle timed out; caller still owns it");
        handle.abort();
    }
    match handle.await {
        Ok(value) => println!("Task returned: {value}"),
        Err(error) if error.is_cancelled() => println!("Task cancellation observed"),
        Err(error) => return Err(error),
    }
    // Borrow the handle when you want to retain control after a timeout.
    // abort requests cancellation; await observes the task's final outcome.
    Ok(())
}

// =====================================================
// 09. abort requests cancellation; await confirms termination
// =====================================================

async fn demo_abort_task() {
    println!("09. abort requests cancellation; await confirms termination");

    let (started_tx, started_rx) = oneshot::channel();
    let handle = tokio::spawn(async move {
        let _notice = DropNotice("aborted task's local state");
        let _ = started_tx.send(());
        pending::<()>().await;
    });
    started_rx.await.expect("worker should notify before waiting");
    handle.abort();
    println!("Cancellation requested; now observe the outcome");
    match handle.await {
        Err(error) if error.is_cancelled() => println!("Confirmed: task was cancelled"),
        Err(error) => println!("Unexpected task failure: {error}"),
        Ok(()) => println!("Task completed before cancellation took effect"),
    }
    // Pending task state is dropped; ordinary local destructors run.
    // Abort cannot preempt arbitrary synchronous code currently running in a poll.
    // A task that already completed can still return Ok even after abort was called.
    // Started spawn_blocking work generally cannot be stopped by abort; file 05
    // explains that separate boundary. Do not assume abort kills an OS thread.
}

// =====================================================
// 10. Cooperative stopping allows asynchronous cleanup
// =====================================================

async fn demo_cooperative_shutdown() -> Result<(), JoinError> {
    println!("10. Cooperative stopping allows asynchronous cleanup");

    let (stop_tx, mut stop_rx) = oneshot::channel::<()>();
    let (started_tx, started_rx) = oneshot::channel();
    let handle = tokio::spawn(async move {
        let mut ticker = interval(Duration::from_millis(10));
        let _ = started_tx.send(());
        loop {
            tokio::select! {
                biased;
                stop = &mut stop_rx => {
                    println!("Worker sees stop message or sender closure: {stop:?}");
                    break;
                }
                _ = ticker.tick() => println!("Worker performs a small periodic step"),
            }
        }
        println!("Worker starts asynchronous cleanup");
        sleep(Duration::from_millis(5)).await;
        println!("Worker cleanup complete");
        "clean shutdown"
    });

    started_rx.await.expect("worker should notify startup");
    let _ = stop_tx.send(());
    println!("Caller requested stop; waiting for cleanup");
    println!("Worker outcome: {}", handle.await?);
    // Decide to stop -> notify -> await task completion. Sending is not joining.
    // This worker explicitly treats either Ok(()) OR a dropped sender as stop.
    // The stop receiver survives each loop iteration because select borrows it.
    // The worker checks the request at cooperative points; a long synchronous
    // operation between checks still delays stopping.
    // oneshot is a one-message channel, previewed here for deterministic control.
    // More channel patterns follow in file 03; larger shutdown designs in file 05.
    Ok(())
}

// =====================================================
// 11. One overall deadline versus a fresh timeout per step
// =====================================================

async fn demo_one_overall_deadline() {
    println!("11. One overall deadline versus a fresh timeout per step");

    let deadline = Instant::now() + Duration::from_millis(30);
    let workflow = async {
        println!("Workflow: preparation");
        sleep(Duration::from_millis(5)).await;
        println!("Workflow: waiting for a response that never arrives");
        pending::<()>().await;
    };
    match timeout_at(deadline, workflow).await {
        Ok(()) => println!("Workflow completed"),
        Err(error) => println!("Expected overall deadline reached: {error}"),
    }
    // One fixed deadline limits the whole workflow, including preparation.
    // A fresh 30ms timeout for EACH of ten steps could allow roughly 300ms overall.
    // Using the same deadline with timeout_at at successive steps preserves one
    // shared budget. It remains cooperative, not a hard preemptive time limit.
    // This workflow future is owned directly, so expiration drops it. If a step
    // starts detached tasks or external side effects, those need their own policy.

    println!();

    println!("join!: all branches. select!: one matching ready branch.");
    println!("timeout: stop waiting after a budget; check what it actually owns.");
    println!("Drop future: local cancellation. Drop handle: detach. abort + await: stop and observe.");
    println!("For asynchronous cleanup: request cooperative shutdown and await it.");
}

// =====================================================
// Helper: visible synchronous cleanup on drop
// =====================================================

struct DropNotice(&'static str);

impl Drop for DropNotice {
    fn drop(&mut self) {
        println!("Drop cleanup: {}", self.0);
    }
}
