//! Rayon: explicit tasks with join, scope, and spawn.
//!
//! Setup: cargo add rayon@1.12
//! Run:   cargo run --bin 04_join_scope_spawn
//!
//! Choose parallel iterators for collection-shaped work (files 01-03).
//! Use join for two computations; scope for a dynamic group of borrowing tasks;
//! spawn for an owned task whose result/completion is communicated explicitly.
//!
//! These are synchronous closures executed by a thread pool, not async futures.
//! Rayon tasks are jobs scheduled on existing workers, not dedicated OS threads.
//! No sleeps are used to guess when a task has finished.
//!
//! References:
//! https://docs.rs/rayon/1.12.0/rayon/fn.join.html
//! https://docs.rs/rayon/1.12.0/rayon/fn.scope.html
//! https://docs.rs/rayon/1.12.0/rayon/struct.Scope.html
//! https://docs.rs/rayon/1.12.0/rayon/fn.spawn.html
//! https://docs.rs/rayon/1.12.0/rayon/struct.ThreadPool.html

use rayon::ThreadPoolBuilder;
use std::sync::mpsc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!();

    demo_join_results();
    println!("\n============================\n");

    demo_join_borrowing();
    println!("\n============================\n");

    demo_disjoint_mutation();
    println!("\n============================\n");

    demo_recursive_join();
    println!("\n============================\n");

    demo_scope_results();
    println!("\n============================\n");

    demo_scope_loop();
    println!("\n============================\n");

    demo_nested_spawning();
    println!("\n============================\n");

    demo_owned_spawn()?;
    println!("\n============================\n");

    demo_custom_pool_tasks()?;
    println!("\n============================\n");

    demo_task_errors();
    println!("\n============================\n");

    demo_choosing_a_task_api();

    Ok(())
}

// =====================================================
// 01. join(): two computations, two returned values
// =====================================================

fn demo_join_results() {
    println!("01. join(): two computations, two returned values");

    let numbers = [2_i32, 3, 8, 9];
    let (squares, cubes) = rayon::join(
        || numbers.iter().map(|n| n * n).collect::<Vec<i32>>(),
        || numbers.iter().map(|n| n * n * n).collect::<Vec<i32>>(),
    );
    println!("Squares: {squares:?}"); // [4, 9, 64, 81]
    println!("Cubes:   {cubes:?}");   // [8, 27, 512, 729]
    // join accepts two closures and returns (left_result, right_result).
    // It waits for both computations; the tuple follows argument order, not
    // completion order. The closures can return different Send types.
    // Parallel execution is possible, not guaranteed: one worker can do both.
    // Unlike thread::spawn, join does not create two dedicated OS threads or
    // return handles that you later join manually.
}

// =====================================================
// 02. join() can borrow surrounding data
// =====================================================

fn demo_join_borrowing() {
    println!("02. join() can borrow surrounding data");

    let text = String::from("Rayon shares borrowed data safely");
    let (byte_count, word_count) = rayon::join(
        || text.len(),
        || text.split_whitespace().count(),
    );
    println!("bytes={byte_count}, words={word_count}");
    println!("Original still usable: {text}");
    // Both closures borrow &text. join finishes before this function can return
    // and drop text, so there is no 'static requirement here.
    // No clone, move, or Arc is needed just to share this immutable String.
    // Send/Sync still govern which values/references may cross worker boundaries.
}

// =====================================================
// 03. Split mutable data before running independent tasks
// =====================================================

fn demo_disjoint_mutation() {
    println!("03. Split mutable data before running independent tasks");

    let mut numbers = [1_i32, 2, 3, 4, 5, 6];
    let (left, right) = numbers.split_at_mut(3);
    rayon::join(
        || left.iter_mut().for_each(|n| *n *= 10),
        || right.iter_mut().for_each(|n| *n *= 100),
    );
    println!("Independent halves: {numbers:?}"); // [10, 20, 30, 400, 500, 600]
    // split_at_mut proves the slices do not overlap. Each task owns an exclusive
    // mutable reference to its half; a shared mutex would add no benefit here.
    // Mutably capturing the WHOLE numbers array in both closures would not compile.
    // This resembles par_iter_mut, but lets you describe two different operations.
}

// =====================================================
// 04. Recursive work splitting with a sequential cutoff
// =====================================================

fn demo_recursive_join() {
    println!("04. Recursive work splitting with a sequential cutoff");

    fn recursive_sum(values: &[u64]) -> u64 {
        const CUTOFF: usize = 256;
        if values.len() <= CUTOFF {
            return values.iter().sum();
        }
        let (left, right) = values.split_at(values.len() / 2);
        let (left_sum, right_sum) = rayon::join(
            || recursive_sum(left),
            || recursive_sum(right),
        );
        left_sum + right_sum
    }

    let numbers: Vec<u64> = (1_u64..=4_096).collect();
    let result = recursive_sum(&numbers);
    println!("Recursive sum: {result}"); // 8390656
    println!("Agrees with sequential sum: {}", result == numbers.iter().sum::<u64>());
    // Nested join calls reuse the current pool rather than creating new threads.
    // A cutoff avoids turning every tiny operation into another pair of jobs.
    // The value 256 is illustrative, not a universal performance recommendation.
    // For an ordinary sum, prefer par_iter().copied().sum(); the point here is
    // learning the divide-and-conquer pattern used by more specialized algorithms.
    // Values are small enough that the u64 additions cannot overflow in this demo.
}

// =====================================================
// 05. scope(): wait for tasks that borrow result slots
// =====================================================

fn demo_scope_results() {
    println!("05. scope(): wait for tasks that borrow result slots");

    let numbers = [1_i32, 2, 3, 4];
    let mut sum = 0_i32;
    let mut product = 0_i32;

    let label = rayon::scope(|scope| {
        scope.spawn(|_| sum = numbers.iter().sum());
        scope.spawn(|_| product = numbers.iter().product());
        "scope body has finished submitting tasks"
    });

    println!("{label}");
    println!("All scoped tasks finished: sum={sum}, product={product}"); // 10, 24
    // scope.spawn takes a closure with one argument: a reference to this scope.
    // |_| ignores it when the task does not need to spawn child tasks.
    // Each task closure returns (); we obtain outputs through separate borrowed slots.
    // The outer scope(...) call returns its BODY's value (label), only after all
    // scoped tasks finish. It does not collect each spawned task's return value.
    // Do not read sum/product in the scope body while their mutable borrows live.
    // Returning from the body is not itself proof that its spawned tasks finished;
    // returning from the entire scope(...) call IS the completion boundary.
}

// =====================================================
// 06. Spawn a dynamic number of scoped tasks in a loop
// =====================================================

fn demo_scope_loop() {
    println!("06. Spawn a dynamic number of scoped tasks in a loop");

    let blocks = [vec![1_i64, 2, 3], vec![10, 20], vec![100]];
    let mut sums = vec![0_i64; blocks.len()];

    rayon::scope(|scope| {
        for (slot, block) in sums.iter_mut().zip(blocks.iter()) {
            scope.spawn(move |_| {
                *slot = block.iter().sum();
            });
        }
    });
    println!("Block sums in input order: {sums:?}"); // [6, 30, 100]
    // iter_mut provides a distinct output slot to each task. No shared push().
    // move moves the REFERENCES slot and block into each task, not the whole
    // sums/blocks collections. Those collections outlive the scope.
    // This works for runtime-sized groups; join always takes exactly two closures.
    // For collection mapping alone, par_iter().map(...).collect() is simpler.
}

// =====================================================
// 07. Child tasks belong to the enclosing scope too
// =====================================================

fn demo_nested_spawning() {
    println!("07. Child tasks belong to the enclosing scope too");

    let mut output = 0_i32;
    rayon::scope(|scope| {
        let slot = &mut output;
        scope.spawn(move |scope| {
            scope.spawn(move |_| {
                *slot = 42;
            });
            // The parent task can return while its child is still pending.
        });
    });
    println!("Child result is available: {output}");
    // scope(...) waits for both directly spawned tasks AND their scoped children.
    // It does not require the parent task to wait manually for its child.
    // A task-local owned value can also be moved into a child; borrowing a local
    // value that dies before the enclosing scope ends would be rejected.
    // Only scope.spawn registers a scoped child. Calling rayon::spawn here
    // instead would create an unscoped task that this scope does not wait for.
}

// =====================================================
// 08. spawn(): move owned data and communicate the result
// =====================================================

fn demo_owned_spawn() -> Result<(), mpsc::RecvError> {
    println!("08. spawn(): move owned data and communicate the result");

    let numbers = vec![2_i64, 3, 8, 9];
    let (tx, rx) = mpsc::channel();
    rayon::spawn(move || {
        let squares: Vec<i64> = numbers.iter().map(|n| n * n).collect();
        let _ = tx.send(squares);
        // Sending fails if the caller dropped the receiver; no need to panic.
    });

    println!("Task submitted; spawn returned no JoinHandle.");
    let squares = rx.recv()?;
    println!("Received the completed calculation: {squares:?}");
    // rayon::spawn requires FnOnce() + Send + 'static and returns ().
    // Owned Vec and Sender move into the task and satisfy that lifetime bound.
    // 'static here does NOT mean the task/data must live forever; it means
    // the closure cannot depend on short-lived borrowed data.
    // move alone cannot extend a reference's lifetime or make a type Send.
    // This receiver waits on the ordinary MAIN thread, outside any pool task.
    // A message proves the calculation before send completed; it is not an OS
    // thread join and would not prove arbitrary code AFTER send had completed.
    // Without explicit coordination, main could exit before detached work finishes.
    Ok(())
}

// =====================================================
// 09. Run scoped and detached tasks on a custom pool
// =====================================================

fn demo_custom_pool_tasks() -> Result<(), Box<dyn std::error::Error>> {
    println!("09. Run scoped and detached tasks on a custom pool");

    let pool = ThreadPoolBuilder::new().num_threads(2).build()?;
    let mut value = 0_u64;
    pool.scope(|scope| {
        scope.spawn(|_| value = (1_u64..=100).sum());
    });
    println!("Custom scoped result: {value}"); // 5050

    let (tx, rx) = mpsc::channel();
    pool.spawn(move || {
        let workers = rayon::current_num_threads();
        let _ = tx.send(format!("Detached task ran in a {workers}-worker pool"));
    });
    println!("{}", rx.recv()?);
    // pool.scope and pool.spawn explicitly select this custom pool.
    // Top-level Rayon operations inside an installed/scoped task use its current
    // pool; top-level operations from outside a pool normally use the global one.
    // Keep the pool alive through this example's result exchange.
    // install(...) waits for its closure; it does not automatically wait for
    // unscoped spawn tasks submitted inside that closure. Use a scope or explicit
    // communication instead of assuming install acts as a join for all submissions.
    Ok(())
}

// =====================================================
// 10. Result values are different from task panics
// =====================================================

fn demo_task_errors() {
    println!("10. Result values are different from task panics");

    fn parse_positive(text: &str) -> Result<u32, String> {
        let value = text.parse::<u32>().map_err(|error| error.to_string())?;
        if value == 0 {
            return Err(String::from("value must be positive"));
        }
        Ok(value)
    }

    let (left, right) = rayon::join(
        || parse_positive("10"),
        || parse_positive("20"),
    );
    println!("Successful task results: {left:?}, {right:?}");

    println!();

    let (left, right) = rayon::join(
        || parse_positive("wrong"),
        || parse_positive("20"),
    );
    println!("Expected failure on left: {left:?}");
    println!("Right still runs: {right:?}");
    // join does not interpret Err as a request to cancel the other closure.
    // It returns a tuple of Results, not Result<(u32, u32), String>.
    // In a Result-returning function, use ? on each returned Result afterward.
    // For scope.spawn, place a Result in a borrowed slot or send it as a message.
    // For unscoped spawn, explicitly send Result<T, E> when the caller needs errors.
    // Panics are separate: join/scope wait for their structured tasks and propagate
    // a panic to the caller (with unwinding enabled). They do not turn it into Err.
    // A detached spawn panic has no joining caller; Rayon uses the pool's panic
    // handler. Without a configured handler, its default behavior is to abort.
    // These demos therefore use Result for anticipated failures, never deliberate
    // panics in detached tasks. panic=abort builds cannot recover by unwinding.
}

// =====================================================
// 11. Choose an API without creating hidden dependencies
// =====================================================

fn demo_choosing_a_task_api() {
    println!("11. Choose an API without creating hidden dependencies");

    println!("Collection mapping: parallel iterators.");
    println!("Two computations with returned values: join.");
    println!("A group of borrowing tasks with a shared end: scope.");
    println!("Owned work with explicit result/completion signaling: spawn.");
    // Scoped tasks and join closures need to be able to make independent progress.
    // Never assume tasks run simultaneously or in submission order. A pool can
    // have one worker, and Rayon may execute apparently parallel work sequentially.
    // A blocking recv in one task waiting for a sibling's send can deadlock:
    // the sibling might still be queued behind that blocked worker.
    // The spawn demos above receive on main, outside the pool, for that reason.
    // Likewise, do not hold a mutex across join/scope while child tasks need it.
    // Sleeping to 'let tasks finish' is not a completion protocol.
    // More message passing, mutexes, and atomics follow in file 05; the later
    // dataloader example will distinguish task scheduling from batch prefetching.
}
