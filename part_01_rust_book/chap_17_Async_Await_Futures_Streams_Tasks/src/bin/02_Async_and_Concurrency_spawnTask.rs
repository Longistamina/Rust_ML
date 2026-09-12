#![allow(dead_code)]
/*
In many cases, the APIs for working with concurrency using `async`
are very similar to those for using `threads`.
In other cases, they end up being quite different.

Even when the APIs look similar between `threads` and `async`,
they often have different behavior—and they nearly always have different performance characteristics.
*/

use std::time::Duration;
extern crate trpl;

// =====================================================================
// 1. `trpl::spawn_task`: the async version of `thread::spawn`
// =====================================================================
/*
`trpl::spawn_task` is the async version of `thread::spawn`
-> spawn an async thread beside main thread

`trpl::sleep` is the async version of `thread::sleep`
*/

fn demo_spawn_task() {
    trpl::block_on(async { // create a main async block inside the main sync block, still in main thread

        trpl::spawn_task(async { // spawn an async thread beside the main async thread, inside the main async block
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        }); // go out of the spawned async thread

        // get back to the main async main thread
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
/*
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!

-------------------------------------

Again, as you can see, like `thread::spawn`,
the count of the async thread stopped at 5 and cannot reach 9.

That is because the async main finished first (at 4),
and stopped all the execution
*/

// =================================================================================
// 2. `future handle` and `.await` as similar versions of `handle` and `join`
// =================================================================================
/*
Remind a bit about `thread::spawn`, it returns a `JoinHandle<T>`,
so that we can assign the spawn thread to a variable `let handle = thread::spawn(...)`,
and later call `handle.join().unwrap()` to force the main to wait until the spawn thread finishes its task.

Here, with `async`, we have similar versions.
We can also assign the async thread to a variable `let handle = trple::spawn_on(...)`,
and later call `handle.await.unwrap()` to force the async main block to wait for the async thread.

NOTE: when we assign the async thread to a variable, it becomes a `lazy future`, hence must call `variable.await`
*/

fn demo_handle_spawn_task() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async { // assign the async thread to a `handle` variable, this is now lazy
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        };

        handle.await.unwrap(); // force the main async block to wait for the spawned async thread
    })
}
/*
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!

---------------------------------------

Now the task in the spawned async thread could count upto 9.
*/

// =================================================================================
// 3. Use `trpl::join()` to handle multiple async futures concurrently
//                     (WITHOUT SPAWNING another OS thread)
// =================================================================================
/*
In fact, we don't truly need to spawn a different OS thread
to run the 2 counting loops concurrently.

Because async blocks compile to anonymous futures,
we can put each loop in an async block
and have the runtime run them both to completion using the `trpl::join` function.

REMIND: still have to put them in an async block using `block_on`
*/

fn demo_join_futures_no_thread() {
    trpl::block_on(async { // open an async block inside a sync block

        let future_1 = async { // create the first `future` using an `async` block
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let future_2 = async { // create the second `future` using an async block
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // future_1.await;
        // future_2.await
        // => it will wait the `future_1` first, then wait the future_2 later

        trpl::join(future_1, future_2).await; // calling `join()` to join them, and `.await` to execute the lazy
                                              // we don't call `.unwrap()` here because the output are just two unit values ((), ())
    })
}
/*
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
...
hi number 9 from the first task!
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    // demo_spawn_task();
    // demo_handle_spawn_task();
    demo_join_futures_no_thread();
}
