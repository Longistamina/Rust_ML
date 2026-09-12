#![allow(dead_code)]
/*
Recall: at each `await` point, Rust gives a runtime a chance to pause the task
and switch to another one if the future being awaited isn’t ready.

The inverse is also true: Rust only pauses `async` blocks
and hands control back to a runtime at an `await` point.

Everything between await points is synchronous.

Therefore, if you do a bunch of work in an async block without an `await` point,
that `future` will block any other `futures` from making progress.
(one future "starves" other futures.)

If you are doing some kind of expensive setup or long-running work,
or if you have a future that will keep doing some particular task indefinitely,
you’ll need to think about when and where to hand control back to the runtime.
*/

use std::thread;
use std::time::Duration;
use trpl;

// ======================================================================
// 0. `slow()` function: illustrate a long-runing operation
// ======================================================================

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms")
}
// This `slow` function uses `thread::sleep()`, so calling it
// will block the current thread for some number of milliseconds.
// => use it as an example of long-running and blocking operation

// ======================================================================
// 1. Demo starvation problem
// ======================================================================

fn demo_starvation() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30); // block the whole thread for 30ms -> task 'b' will starve for 30 ms
            slow("a", 10); // block the whole thread for 10ms -> task 'b' will starve for 10 ms
            slow("a", 20); // block the whole thread for 20ms -> task 'b' will starve for 20 ms
            trpl::sleep(Duration::from_millis(50)).await; // just block this 'a' async task, task 'b' is not starved anymore
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75); // block the whole thread for 75 ms -> trpl::sleep() of 'a' will be starved 75ms
            slow("b", 10); // block the whole thread for 10 ms -> trpl::sleep() of 'a' will be starved 10ms
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await; // just block this 'b' async task, now trpl::sleep() of 'a' can run
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    })
}
/*
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.

--------------------------------------------------------

In `trpl::select(a, b).await`, because we list 'a' before 'b',
it will poll async task 'a' first.
=> That's why we see 'a' started and ran for ...ms.

Since the `slow()` function uses `thread::sleep()`,
it will not only pause task 'a' but also block all other tasks of the thread
=> that's why async task 'b' is be starved for 60ms in total.

Only when task 'a' reaches `trpl::sleep().await` that task 'b' is released,
because `trpl::sleep().await` only pauses task 'a' but does not block
the whole thread, hence task 'b' now can start.

And then, while task 'b' is running, the `slow()` calls in task 'b'
again block the whole thread and make task 'a' starved.
Only when task 'b' reaches `trpl::sleep().await` that task 'a' is released
=> it could print "'a' finished" now.

When the "'a' finished" is printed out,
task 'a' is treated as winner task in the race.
`trpl::select()` will then immediately drop 'b' right after 'a' finished
=> task 'b' can never reach the final `println!()` line
=> It never print "'b' finished"

--------------------------------------------------------

Try running `trpl::select(b, a).await;`
and you will see the results being reversed.

--------------------------------------------------------

You can also try removing the line `trpl::sleep(Duration::from_millis(50)).await`
in task 'a', then you will see that 'a' will finish right away and 'b' can never be started.
*/

// ======================================================================
// 2. Demo starvation problem: interleave
// ======================================================================

fn demo_starvation_interleave() {
    trpl::block_on(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30); // pause and block the thread, 'b' starved
            trpl::sleep(one_ms).await; // pause but does not block the thread, runtime will execute 'b'
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75); // pause and block the thread, 'a' starved
            trpl::sleep(one_ms).await; // pause but does not block the thread, runtime jumps back to execute 'a'
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    })
}
/*
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.

--------------------------------------------------------------------

Now you can see that the runtime jumps back and forth
between task 'a' and task 'b'.

Whennever it sees an 'await' point in 'a', it will jumpt to execute 'b'.
And vice versa, if it sees an 'await' point in 'b', it will jump back to execute 'a'

--------------------------------------------------------------------

We can also say that when the runtime sees an 'await' point in 'a',
it will give the control to 'b' so that 'b' can run.

Is there a general way where we ask the runtime to hand the control
from one async task to other async tasks
=> use `trple::yield_now()`
*/

// =============================================================================
// 3. `trpl::yield_now().await`: pass the control to other async blocks
// =============================================================================

fn demo_yield_now() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30); // pause and block the thread, 'b' starved
            trpl::yield_now().await; // hand the runtime control to 'b' so that 'b' can run
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75); // pause and block the thread, 'a' starved
            trpl::yield_now().await; // hand the runtime control back to 'a' so that 'a' can run
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    })
}
/*
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.

-----------------------------------------------

The results are similar as the same codes above.
The differene here is that we don't use `trpl::sleep().await` anymore.

Here we use `trple::yield_now().await` to pass the runtime control
from task 'a' to task 'b' and vice versa without sleeping for any seconds.

=> this helps us make the progress as fast as we can.
(the `trpl::sleep()` in the previous sections are just for
demonstrating how runtime control is passed between tasks back and forth)

----------------------------------------------------------------------

In real-world code, you won’t usually be alternating function calls
with `await` points on every single line, of course.

While yielding control in this way is relatively inexpensive, it’s not free.
In many cases, trying to break up a compute-bound task might make it significantly slower,
so sometimes it’s better for overall performance to let an operation block briefly.
*/

// ====================================================================================
// 4. Building Our Own Async Abstractions
// ====================================================================================
/*
We can also compose futures together to create new patterns.
For example, we can build a timeout function with async building blocks we already have.

Let’s implement this! To begin, let’s think about the API for timeout:
+ It needs to be an async function itself so we can await it.
+ Its first parameter should be a future to run. We can make it generic to allow it to work with any future.
+ Its second parameter will be the maximum time to wait. If we use a `Duration`, that will make it easy to pass along to `trpl::sleep`.
+ It should return a `Result`.
  If the future completes successfully, the Result will be `Ok` with the value produced by the future.
  If the timeout elapses first, the Result will be `Err` with the duration that the timeout waited for.
*/

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) // `F: Future` is a generic for any future type
    -> Result<F::Output, Duration> { // `F::Output` is the generic of output of any future
        match trpl::select(future_to_try, trpl::sleep(max_time)).await {
            trpl::Either::Left(output) => Ok(output),
            trpl::Either::Right(_) => Err(max_time)
        }
        /*
        So here we set up two futures to race each other:
        + `future_to_try`
        + `trpl::sleep(max_time)`

        `select` will poll the `future_to_try` first.
        If `future_to_try` is not ready (not finish its task yet),
        then `select` will return `Poll::Pending`
        -> the runtime will pass the control to `trpl::sleep(max_time)`.

        Since `trpl::sleep(max_time)` is also a future, while sleeping,
        its poll result would also `Poll::Pending`
        -> the runtime pass the control back to `future_to_try` for it to run.

        If both tasks are `Pending`, then `select` will yield to the runtime executor.

        Time passes, the executor wakes up when either `future_to_try` makes progress
        or the `trpl::sleep` timer hits `max_time`.

        Now, `select` will poll them again, which returns `Poll:Ready` first is the winner.

        For example, if the `future_to_try` fails to finish its task,
        then `trpl::sleep(max_time)` will win
        => the match pattern falls into `trpl::Either::Right(_)`
        => returns `Err(max_time)`
        */
}

fn demo_timeout() {
    trpl::block_on(async {
        let slow = async { // this will be the `future_to_try`
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}
/*
Failed after 2 seconds
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    // demo_starvation();
    // demo_starvation_interleave();
    // demo_yield_now();
    demo_timeout();
}
