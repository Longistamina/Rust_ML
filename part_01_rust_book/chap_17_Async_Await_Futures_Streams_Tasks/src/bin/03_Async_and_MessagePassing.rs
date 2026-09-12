#![allow(dead_code)]
/*
Like `thread` and `sync` have `mpsc` to create a channel
and transfer data between threads with that channel,
we can use `trpl::channel` to do similar things in async way
*/

extern crate trpl;
use std::time::Duration;

// =============================================================
// 2. Demo `trpl::channel`
// =============================================================
/*
```let (tx, mut rx) = trpl::channel();```

 The async version of the `channel()` API is a little different from `sync::mpsc::channel()`,
 it uses a `mutable` rather than an immutable receiver `rx`.
 => that's why we have to use `mut rx` here.

And its `rx.recv()` method produces a future we need to `await`
rather than producing the value directly.
*/

fn demo_trpl_channel() {
    trpl::block_on(async { // open an async block
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap(); // receiver gets `val` from sender in the same async block
        println!("received '{received}'");
    })
}

// =============================================================
// 2. `trpl::channel` without concurrency
// =============================================================
/*
Let's ask sender to send multiple messages,
and then receiver to receive multiple messages from the sender

Here, both `tx` and `rx` are in the same async block,
so this code is not concurrent, everything will happen in a sequence.
*/

fn demo_trpl_channel_no_concurrency() {
    trpl::block_on(async { // open an async block
        let (tx, mut rx) = trpl::channel();

        // Create a vect of `str`, then convert into to `String`
        let messages: Vec<String> = vec!["hi", "from", "the", "future"]
            .iter().map(|mess| mess.to_string()).collect();

        for mess in messages {
            tx.send(mess).unwrap(); // send the messages one by one
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(received_mess) = rx.recv().await {
            println!("Receiver received: '{received_mess}'");
        }
    })
}
/*
=== wait 500ms * 4 = 2s for the senser `tx` to finish the job ===

received 'hi'
received 'from'
received 'the'
received 'future'

(It waited for all the sender `tx` to send everything first.
only when sender finished its job, receiver `tx` started to `await`
and get the message one by one, then print out)

-------------------------

`rx.recv().await` returns the type `Option<T>`,
generally we use `.unwrap()` right after to get the value.

Here, we can use `while let` instead.

If `rx.recv().await` returns `Some(T)`
=> take the value and print out.

If `rx.recv().await` returns None (when there is no more message to receive)
=> break the while loop
*/

// =============================================================================
// 3. Make `trpl::channel` message passing concurrent using `spawn_task`
//    to spawn another thread.
// =============================================================================
/*
To make this message passing channel concurent,
we can spawn an async thread, move the sender there,
and send value back to the main async thread.

In the main async thread, the receiver will try to resolve what it receives

=> 2 different threads => concurrency
*/

fn demo_trpl_channel_spawn_thread() {
    let (tx, mut rx) = trpl::channel();

    trpl::block_on(async { // open an async block

        trpl::spawn_task(async move { // spawn an async thread in the async block
                                      // also use `move` keyword to move the sender `tx` into the thread

            // Create a vect of `str`, then convert into to `String`
            let messages: Vec<String> = vec!["hi", "from", "the", "spawned", "async thread"]
                .iter().map(|mess| mess.to_string()).collect();

            for mess in messages {
                tx.send(mess).unwrap(); // send the messages one by one
                trpl::sleep(Duration::from_millis(500)).await;
            }
        }); // go back to the main async thread

        while let Some(received_mess) = rx.recv().await {
            println!("Main async thread got sometthing: '{}'", received_mess)
        }
    });
}
/*
=== wait 500ms for the sender ===
Main async thread got sometthing: 'hi'
=== wait 500ms for the sender ===
Main async thread got sometthing: 'from'
=== wait 500ms for the sender ===
Main async thread got sometthing: 'the'
=== wait 500ms for the sender ===
Main async thread got sometthing: 'spawned'
=== wait 500ms for the sender ===
Main async thread got sometthing: 'async thread'

(The sender from spawned async thread waited then sent,
then the receiver from main async thread received and printed,
and kept doing like that until there is nothing left to send and receive.)
*/

// =====================================================================================
// 4. `trpl::channel` and message passing with async blocks
//                    (WITHOUT SPAWNING another thread)
// =====================================================================================
/*
As being said, because async blocks compile to anonymous futures,
we don't need to put them in another thread to demonstrate.

In the previous file (02_Async_and_Concurrency.rs),
we put 2 different loops in 2 different async blocks to achieve concurrency,
then call `trpl.join(..., ...).await`.

Here, we can do the same thing: put the `tx` and `rx` in 2 different async blocks

-------------------------------------------------------------------------------------

NOTE: remember to `drop(tx)` after sending all messages to close the channel,
      otherwise the `while let Some(...) = rx.recv().await` will never break,
      hence the code will never reach the final `println!()` line.

(Actually, in reality, we won't `drop(tx)` because we want our applications
like webserver to run like forever without closing the message passing channel,
so that it can handle incoming request forever.)
*/

fn demo_trpl_channel_async_blocks() {
    let (tx, mut rx) = trpl::channel();

    trpl::block_on(async { // open an async block

        let fut_send = async { // put sender `tx` into the first async block, retunrns a future
            let messages: Vec<String> = vec!["hi", "from", "the", "1st", "async block"]
                .iter().map(|mess| mess.to_string()).collect();

            for mess in messages {
                tx.send(mess).unwrap(); // send the messages one by one
                trpl::sleep(Duration::from_millis(500)).await;
            }

            drop(tx); // Explicitly drop `tx` here to close the channel!
            // If we don't do so, the `while let Some(...) = rx.recv().await` will never break,
            // and the final `println!()` line will never be reached
            // (In reality, we don't need to `drop(tx)`, because we would like to keep the channel stay alive forever
            // to handle incoming requests, like webserver applications)
        };

        let fut_recv = async { // put sender `tx` into the first async block, retunrns a future
            let mut container: Vec<String> = Vec::new();

            while let Some(received_mess) = rx.recv().await {
                println!("The 2nd async block received something: '{}'", received_mess);
                container.push(received_mess);
            }

            return container // return the `container`, so that we can write `let all_received = fut_recv.await;`
        };

        let (_, all_received) = trpl::join(fut_send, fut_recv).await;

        println!("\nAll received items: {:?}", all_received);
        // All received items: ["hi", "from", "the", "1st", "async block"]
    })
}
/*
=== wait 500ms for the sender ===
The 2nd async block received something: 'hi'
=== wait 500ms for the sender ===
The 2nd async block received something: 'from'
=== wait 500ms for the sender ===
The 2nd async block received something: 'the'
=== wait 500ms for the sender ===
The 2nd async block received something: '1st'
=== wait 500ms for the sender ===
The 2nd async block received something: 'async block'

All received items: ["hi", "from", "the", "1st", "async block"]
*/

// =====================================================================================
// 5. MPSC with `trpl::channel` and async blocks
//    `trpl::join!()` macro
// =====================================================================================
/*
We can also build the multiple-producer and single-consumer workflow
using different async blocks instead of spawning different threads.

tx -> one async block
tx_1 -> another async block

rx -> another async block

--------------------------------------------

NOTE: Here we will use `trpl::join!()` (instead of `trpl::join()`).
      `trpl::join!()` is a macro that performs the awaiting internally.
=> don't need to call `.await` right after
*/

fn demo_trpl_mpsc() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel(); // initilize tx and rx

        let tx1 = tx.clone(); // clone tx to get another tx1

        ////////////////////////////////

        let tx1_fut = async move { // send messages from tx1
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        ////////////////////////////////

        let rx_fut = async { // rx receives values from both `tx` and `tx1`
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        ////////////////////////////////

        let tx_fut = async move { // send values from tx
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        ////////////////////////////////

        trpl::join!(tx1_fut, tx_fut, rx_fut); // join all async block
        // here we use macro `trpl::join!`, so we don't need to call `.await` right after
    })
}
/*
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    // demo_trpl_channel();
    // demo_trpl_channel_no_concurrency();
    // demo_trpl_channel_spawn_thread();
    // demo_trpl_channel_async_blocks();
    demo_trpl_mpsc();
}
