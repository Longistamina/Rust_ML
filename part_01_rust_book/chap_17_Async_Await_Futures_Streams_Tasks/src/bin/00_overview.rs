/*
Chapter 17: Async, Await, Futures, and Streams
==============================================

Main idea
---------

Asynchronous programming allows a program to work on other tasks while one task is waiting for something to finish.

For example, when a program waits for:

- data from a network
- a file to be read
- a database response
- a timer to finish

the CPU does not need to remain idle. The current task can pause, allowing another task to run.
When the awaited operation is ready, the paused task can resume.

Concurrency vs. parallelism
---------------------------

`Concurrency` means making progress on multiple tasks by switching between them.
A single CPU core can run concurrently:
    Task A runs
    Task A waits
    Task B runs
    Task B waits
    Task A resumes
=> The tasks do not necessarily execute at exactly the same instant.

`Parallelism` means multiple tasks are literally executing at the same time, usually on different CPU cores.
Rust async primarily provides concurrency. However, an async runtime may also use multiple threads,
so async programs can sometimes execute in parallel as well.

CPU-bound vs. I/O-bound work
----------------------------

CPU-bound work keeps the processor busy performing calculations.
Examples:
- video rendering
- scientific computation
- image processing
- model training

I/O-bound work spends much of its time waiting for external data or devices.
Examples:
- downloading a file
- reading from disk
- querying a database
- waiting for user input

Async programming is especially useful for I/O-bound work
because the program can perform other tasks during the waiting time.

Blocking code
-------------

In ordinary blocking code, a slow operation prevents the current thread from continuing:

    Task A starts a network request
    Task A waits for the response
    The entire thread remains blocked
    Task A continues when the response arrives

One possible solution is to create a separate operating-system thread for every task.
However, threads consume system resources, so creating thousands of them can become expensive.

Async code
----------

In async code, only the current task pauses:

    Task A starts a network request
    Task A pauses
    The async runtime runs Task B
    The network response arrives
    The runtime resumes Task A

Many async tasks can therefore share a relatively small number of operating-system threads.

Important Rust concepts
-----------------------

async fn:
An async function does not immediately execute all its work and return the final result.
Calling it produces a Future representing work that may finish later.

Future:
A Future represents an eventual value. It may currently be incomplete,
but it can later become ready and produce its result.

.await:
The .await operator waits for a Future to become ready.
While waiting, the current async task may yield control so another task can run.

Async runtime:
Rust's standard library defines the Future abstraction,
but it does not provide a complete async runtime.

A runtime such as Tokio is responsible for:
- running async tasks
- checking whether futures can make progress
- pausing tasks that are waiting
- waking tasks when their resources become ready
- scheduling runnable tasks

Stream:
A Future eventually produces one result.
A Stream produces multiple results over time.
It is similar to an Iterator, but its next value may not be available immediately.

Compact mental model
--------------------

    async fn  = a function whose execution can pause
    Future    = a value representing unfinished work
    .await    = pause here until the Future is ready
    runtime   = coordinate and schedule async tasks
    Stream    = asynchronously produce multiple values

Most important takeaway
-----------------------

Async does not automatically make CPU-heavy computation faster.

Its main benefit is efficiently handling many tasks that spend time waiting, especially network and other I/O operations.

In one sentence:
Rust async lets a task pause when it cannot currently make progress,
allowing the runtime to execute other tasks and return to the paused task when it is ready.
*/
fn main() {

}
