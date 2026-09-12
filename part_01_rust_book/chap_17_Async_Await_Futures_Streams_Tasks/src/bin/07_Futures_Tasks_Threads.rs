/*
================================================================================
PUTTING IT TOGETHER: FUTURES, TASKS, AND THREADS
================================================================================

This final section answers one main question:
"Should we use threads or async?"

The answer is:
"It depends on the work, and many real programs use both."
*/

// ==============================================================================
// 1. FUTURES, TASKS, AND THREADS
// ==============================================================================

/*
Future
------
A Future is the smallest unit of asynchronous work.
It represents one asynchronous computation that may:
+ make progress
+ become Pending
+ resume later
+ eventually produce an Output

One Future may contain and await many other Futures.
Therefore, a Future can form a tree:
outer Future
├── network Future
├── timer Future
└── another async block
    ├── channel Future
    └── file I/O Future

Task
----
A Task is a runtime-managed unit of asynchronous execution.
A task may contain and manage multiple Futures.
We can create one with something like:
```
trpl::spawn_task(async {
    // asynchronous work
});
```

Tasks are managed by the async runtime, not directly by the operating system.
Tasks are usually much lighter than operating-system threads.

Thread
------
A Thread is an operating-system-managed unit of execution.
We can create one with:
```
std::thread::spawn(|| {
    // synchronous work
});
```

Threads can genuinely execute in parallel
when multiple CPU cores are available.

However, each thread usually requires more memory and operating-system
resources than an async task.
*/

// ==============================================================================
// 2. THE MANAGEMENT HIERARCHY
// ==============================================================================
/*
The relationship is approximately:

Operating system
└── manages Threads
    └── async Runtime runs on one or more Threads
        └── runtime Executor manages Tasks
            └── Tasks contain and poll Futures

Compact version:
+ OS manages threads
+ Runtime manages tasks
+ Tasks manage futures

A Future is the smallest unit of async concurrency.
A Task is a scheduled group of asynchronous work.
A Thread is an OS-managed execution resource.
*/

// ==============================================================================
// 3. CONCURRENCY BOUNDARIES
// ==============================================================================
/*
Threads form boundaries around synchronous operations.
Concurrency can happen between different threads.

Tasks form boundaries around asynchronous operations.
Concurrency can happen:
+ between tasks
+ between Futures inside one task

For example:
```
let task = async {
    let fut1 = async_operation_1();
    let fut2 = async_operation_2();

    trpl::join(fut1, fut2).await;
};
```
Here, fut1 and fut2 execute concurrently inside the same task.
*/

// ==============================================================================
// 4. WHEN TO USE THREADS
// ==============================================================================
/*
Threads are generally suitable for CPU-bound work.
CPU-bound work spends most of its time performing computations.
Examples:
+ video encoding
+ image processing
+ scientific computations
+ matrix calculations
+ processing independent chunks of a large dataset

If separate parts can be calculated independently,
multiple threads may run them in parallel on multiple CPU cores.

CPU-bound work:
    `use threads / parallel-processing libraries`
*/

// ==============================================================================
// 5. WHEN TO USE ASYNC
// ==============================================================================
/*
Async is generally suitable for I/O-bound work.
I/O-bound work spends much of its time waiting.

Examples:
+ network requests
+ database queries
+ file I/O
+ channel messages
+ timers
+ user-interface events
+ handling many network connections

While one Future is waiting, the runtime can execute another Future.
I/O-bound work:
    `use async Futures and Tasks`
*/

// ==============================================================================
// 6. THREADS AND ASYNC ARE COMPLEMENTARY
// ==============================================================================
/*
Threads and async are not enemies.

Many real applications use both:
+ Threads provide CPU parallelism.
+ Async tasks provide efficient I/O concurrency.

For example:
Video encoding:
    CPU-bound
    -> perform it on dedicated worker threads
Updating the user interface when encoding finishes:
    event/message-based
    -> use an async channel


Another example:
Web server:
    async tasks handle thousands of connections
Expensive image processing requested by a client:
    worker threads perform the CPU-heavy calculation
*/

// ==============================================================================
// 7. COMBINING A THREAD WITH ASYNC MESSAGE RECEIVING
// ==============================================================================

use std::{thread, time::Duration};

fn demo_thread_and_async() {
    let (tx, mut rx) = trpl::channel();

    // This part uses a separate operating-system thread.
    //
    // `move` transfers ownership of `tx` into the thread.
    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();

            // This blocks only this spawned thread.
            thread::sleep(Duration::from_secs(1));
        }

        // `tx` is dropped when the thread finishes.
        //
        // The channel then closes because no Sender remains.
    });

    // This part executes asynchronously.
    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }

        // `rx.recv().await` returns None after `tx` is dropped
        // and all queued messages have been received.
    });
}

/*
Execution
---------

Spawned thread                 Async task
--------------                 ----------

send 1
sleep for 1 second       ->    receive 1 and print it
                               await next message

send 2
sleep for 1 second       ->    receive 2 and print it
                               await next message

...

send 10                  ->    receive 10 and print it
thread ends
tx is dropped            ->    recv returns None
                               receiver loop ends


The thread performs blocking operations:
thread::sleep(...)

The async receiver does not block its runtime thread while waiting:
rx.recv().await

Therefore, synchronous threaded code and asynchronous code can communicate
through a channel.
*/

// ==============================================================================
// 8. ASYNC RUNTIMES MAY USE MULTIPLE THREADS
// ==============================================================================
/*
Using async does not necessarily mean using only one thread.
Many async runtimes are multithreaded.

They may distribute async tasks across several worker threads:
Runtime
├── worker thread 1
│   ├── task A
│   └── task B
│
├── worker thread 2
│   ├── task C
│   └── task D
│
└── worker thread 3
    └── task E

Some runtimes use work stealing.
Work stealing means:
"If one runtime thread has too many tasks and another thread is relatively idle,
the runtime may move tasks to the idle thread."
This balances the workload automatically.

Therefore, a real async program may contain all three levels:
+ multiple operating-system threads
+ multiple async tasks on those threads
+ multiple Futures inside those tasks
*/

// ==============================================================================
// 9. IMPORTANT PRACTICAL RULE
// ==============================================================================
/*
Do not perform long blocking work directly inside an async task.
For example:
```
async {
    std::thread::sleep(Duration::from_secs(10));
}
```
`thread::sleep` blocks the entire runtime worker thread.
Other async tasks assigned to that thread cannot progress during that time.

For an asynchronous delay, use:
```
async {
    trpl::sleep(Duration::from_secs(10)).await;
}
```
This pauses only the current async task and lets the runtime execute other tasks.

For genuinely CPU-heavy or unavoidable blocking work, move it to:
+ a dedicated thread
+ a thread pool
+ a runtime facility such as spawn_blocking
*/

// ==============================================================================
// 10. DECISION GUIDE
// ==============================================================================
/*
CPU-bound and parallelizable work:
=> Prefer threads or a parallel-processing library.
=> Examples:
    encoding video
    matrix computations
    processing large independent data chunks

I/O-bound and highly concurrent work:
=> Prefer async tasks and Futures.
=> Examples:
    network requests
    database connections
    message handling
    timers

Both CPU work and I/O waiting:
=> Combine threads and async.
=> Examples:
    async web server + threaded image processing
    threaded video encoding + async UI notifications
*/

// ==============================================================================
// 11. FINAL CHAPTER SUMMARY
// ==============================================================================
/*
async fn / async block
    creates a lazy Future

Future
    represents one asynchronous computation

.await
    waits for a Future while allowing the current task to pause

Runtime
    drives Futures by polling them

Task
    is a runtime-managed unit containing asynchronous work

Thread
    is an operating-system-managed unit of execution

Stream
    produces multiple asynchronous values over time

join / join!
    wait for multiple Futures to complete

select
    races Futures and returns the first one completed

Channel
    sends values between concurrent tasks or threads

Pin
    prevents a potentially self-referential Future from moving

Unpin
    marks types that remain safe to move


Final rule of thumb:

CPU-bound work
    -> threads and parallelism

I/O-bound work
    -> async tasks and concurrency

Mixed workload
    -> combine threads and async
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_thread_and_async();
}
