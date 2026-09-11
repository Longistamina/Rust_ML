#![allow(dead_code)]

/*
================================================================================
A CLOSER LOOK AT THE TRAITS FOR ASYNC
================================================================================

This section explains the low-level machinery underneath:

+ async
+ .await
+ Future
+ Stream
+ StreamExt
+ Pin
+ Unpin

The main ideas are:

1. A Future is a lazy state machine.
2. A runtime repeatedly polls that Future.
3. Poll::Pending means "I am not ready; check me again later."
4. Poll::Ready(value) means "I have finished and produced value."
5. Futures may contain references to their own internal data.
6. Such self-referential Futures must not move after execution begins.
7. Pin provides this "must not move" guarantee.
8. Unpin marks types that remain safe to move even when pinned.
9. Stream combines:
       Future's asynchronous readiness
   with Iterator's sequence of values.
*/

use std::future::Future;
use std::pin::{pin, Pin};
use std::time::Duration;

use trpl;

// ==============================================================================
// 0. HIGH-LEVEL RELATIONSHIP
// ==============================================================================
/*
Iterator
--------
An Iterator lazily produces zero or more values synchronously.

Its important method is conceptually:
```
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Possible results:
```
Some(item)    -> another item was produced
None          -> the iterator has finished
```

Future
------
A Future lazily produces one final value asynchronously.

Its important method is:
```
trait Future {
    type Output;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
```

Possible results:
```
Poll::Ready(output) -> the Future has finished
Poll::Pending       -> the Future is not ready yet
```

Stream
------
A Stream lazily produces zero or more values asynchronously.

Its important method is conceptually:
```
trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>>;
}
```

Possible results:
```
Poll::Ready(Some(item)) -> another item was produced
Poll::Pending           -> no item is ready yet
Poll::Ready(None)       -> the entire Stream has finished
```

Compact comparison
------------------

Iterator:
    Option<Item>

Future:
    Poll<Output>

Stream:
    Poll<Option<Item>>

Therefore:

Stream = Future-style readiness + Iterator-style sequence
*/

// ==============================================================================
// 1. THE FUTURE TRAIT
// ==============================================================================
/*
The real Future trait is approximately:
```
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
```

The associated type `Output`
----------------------------
`Output` describes the value produced when the Future finishes.

For example:
```
async {
    42
}
```

produces something implementing:
```Future<Output = i32>```

Another example:
```
async {
    println!("Hello");
}
```

produces something implementing:
```Future<Output = ()>```

The relationship is similar to Iterator:

Iterator::Item
    = the type of every value yielded by an Iterator

Future::Output
    = the type of the one final value produced by a Future
*/

// ==============================================================================
// 2. THE POLL ENUM
// ==============================================================================
/*
The Poll enum is approximately:
```
enum Poll<T> {
    Ready(T),
    Pending,
}
```

Poll::Ready(value)
------------------
The Future has completed. The produced value is available:
```Poll::Ready(42)```

After a normal Future returns `Poll::Ready`, it should generally not be polled again.
Some Future implementations may panic if they are polled after completion.

Poll::Pending
-------------
The Future has not completed.

This does NOT mean:
+ the Future failed
+ the Future returned None
+ the Future was cancelled

It means:
"I cannot make progress right now. Please poll me again when I am ready."

Poll compared with Option
-------------------------
Option<T>:
```
Some(value) -> a value exists
None        -> no value exists
```

Poll<T>:
```
Ready(value) -> the asynchronous operation has finished
Pending      -> the operation is unfinished and may become ready later
```

`Pending` is therefore about readiness over time, not the absence of a value.
*/

// ==============================================================================
// 3. `.await` IS BUILT ON TOP OF `Future::poll()`
// ==============================================================================
/*
When we write:
```let output = some_future.await;```

Rust uses machinery based on repeated calls to:
```some_future.poll(...)```

A very simplified and NOT directly compilable representation is:
```
loop {
    match some_future.poll(...) {
        Poll::Ready(output) => break output,

        Poll::Pending => {
            // Pause the current Future.
            // Return control to the async runtime.
            // Let other Futures run.
            // Resume when this Future is woken.
        }
    }
}
```

IMPORTANT:
The runtime does not usually poll a Pending Future continuously in a busy loop.

Instead:
1. The runtime polls the Future.
2. The Future returns Poll::Pending.
3. The Future registers a Waker through Context.
4. The runtime stops polling this Future for now.
5. The underlying event becomes ready.
6. The Waker tells the runtime that the Future can make progress.
7. The runtime schedules and polls the Future again.

Execution model
---------------
Runtime polls Future
        |
        +-- Poll::Ready(value)
        |       |
        |       +-- Future is finished
        |
        +-- Poll::Pending
                |
                +-- Future is paused
                +-- runtime executes other Futures
                +-- Future is polled again after being woken
*/

// ==============================================================================
// 4. THE CONTEXT AND WAKER
// ==============================================================================
/*
The poll method receives:
```cx: &mut Context<'_>```

The Context contains access to a Waker.
The Waker allows a Future to notify the runtime:
"I may be able to make progress now. Please poll me again."

For example, imagine a network Future:
1. Runtime polls the Future.
2. Network data has not arrived.
3. Future registers its Waker.
4. Future returns Poll::Pending.
5. Runtime runs another task.
6. Network data arrives.
7. Network system invokes the Waker.
8. Runtime schedules the Future again.
9. Future is polled again.
10. Future returns Poll::Ready(response).

In normal application code, we rarely interact directly with Context or Waker.
They mainly matter when:
+ implementing Future manually
+ writing async runtimes
+ creating low-level async libraries
*/

// ==============================================================================
// 5. EVERY ASYNC BLOCK HAS ITS OWN ANONYMOUS TYPE
// ==============================================================================
/*
Consider:
```
let fut1 = async {
    println!("Future 1");
};

let fut2 = async {
    println!("Future 2");
};
```

Both Futures have:
```Future<Output = ()>```

However, they do NOT have the same concrete type.
The compiler creates a different anonymous state-machine type for each async block:
```
fut1: AnonymousFutureTypeA
fut2: AnonymousFutureTypeB
```

This is similar to closures:
```
let closure1 = || println!("one");
let closure2 = || println!("two");
```

Even though both closures have the same parameters and return type,
each closure has its own anonymous concrete type.

Therefore, this generally cannot work directly:
```let futures = vec![fut1, fut2];```

A Vec requires every element to have exactly the same concrete type:
```
Vec<T>
    every element must be T
```

But here:
`fut1` is `AnonymousFutureTypeA`
`fut2` is `AnonymousFutureTypeB`
*/

// ==============================================================================
// 6. `join!()` VERSUS `join_all()`
// ==============================================================================
/*
`trpl::join!()`
---------------
Use `join!` when the number of Futures is KNOWN at compile time:
```trpl::join!(fut1, fut2, fut3);```

The macro can accept Futures with different concrete types.
It awaits them internally, so we do not write:
```trpl::join!(fut1, fut2, fut3).await; // incorrect```

Instead:
```trpl::join!(fut1, fut2, fut3);```       // correct


`trpl::join_all()`
------------------
Use `join_all()` when Futures are stored in a collection:
```
let futures = vec![...];

trpl::join_all(futures).await;
```

Collection `vec![...]` elements must have one common type

The number of Futures may be determined at runtime.
Unlike `join!*(`, `join_all()` is a function that returns a Future,
so its result must be awaited.

Compact comparison
------------------

`trpl::join!(fut1, fut2, fut3);`
    + macro
    + fixed number known at compile time
    + accepts different Future types
    + awaits internally

`trpl::join_all(futures).await;`
    + function
    + accepts a collection
    + number may be determined at runtime
    + collection elements must have one common type
    + returns a Future, so `.await` is required
*/

// ==============================================================================
// 7. USING A TRAIT OBJECT TO GIVE FUTURES ONE COMMON TYPE
// ==============================================================================
/*
We can erase the Futures' different concrete types using a trait object:
```
Box<dyn Future<Output = ()>>
```

For example, we might try:
```
let futures: Vec<Box<dyn Future<Output = ()>>> = vec![
    Box::new(fut1),
    Box::new(fut2),
    Box::new(fut3),
];
```

Now every Vec element has the same visible type:
```Box<dyn Future<Output = ())>>```

What `dyn Future` means
-----------------------
`dyn Future<Output = ()>` means:
"Some concrete type implementing Future<Output = ()>,
 but its exact concrete type is erased."

Why use Box?
------------
A trait object such as:
```dyn Future<Output = ()>```

is dynamically sized because Rust does not know the concrete Future's size.
Therefore, it must normally be placed behind a pointer such as Box:
```Box<dyn Future<Output = ()>>```

However, Box by itself is not enough for `join_all()`:
```
let futures: Vec<Box<dyn Future<Output = ()>>> = vec![...];

trpl::join_all(futures).await;
```

This causes an error similar to:
```dyn Future<Output = ()> cannot be unpinned```

To understand the error, we need to understand how async state machines
can become self-referential.
*/

// ==============================================================================
// 8. WHY A FUTURE MAY BECOME SELF-REFERENTIAL
// ==============================================================================
/*
An async block is compiled into a state machine.

For example:
```
async {
    let text = String::from("hello");
    let reference = &text;

    some_async_operation().await;

    println!("{reference}");
}
```

The generated Future must store:
+ `text`
+ `reference`
+ which execution state it is currently in
+ the inner Future being awaited

Conceptually, its internal state might look like:
```
GeneratedFuture {
    text: String,
    reference: pointer to its own `text` field,
    state: Waiting,
}
```

The internal reference points to another field inside the same Future:
```
GeneratedFuture
+-----------------------------------+
| text: "hello"                     |
|       ^                           |
|       |                           |
| reference ------------------------+
| state: Waiting                    |
+-----------------------------------+
```

If the complete Future is moved to another memory location:
```
Old location                     New location
+--------------------+           +--------------------+
| text: invalid      |           | text: "hello"      |
+--------------------+           +--------------------+
         ^
         |
reference may still point here
```

The internal reference could remain pointed at the old location.

That would be invalid and unsafe.

Therefore, after such a Future begins execution and establishes its internal
references, the Future must not move in memory.
*/

// ==============================================================================
// 9. THE PIN TYPE
// ==============================================================================
/*
Pin provides the guarantee:
"The value being pointed to will not move in memory."

The Future trait requires:
```
fn poll(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
) -> Poll<Self::Output>;
```

It does NOT accept an ordinary:
`&mut Self`

Instead, it accepts:
`Pin<&mut Self>`

This means that before a Future is polled, it must be pinned.
Pin allows the Future implementation to rely on a stable memory location.

Important distinction
---------------------
Pin is not another allocation mechanism.
Pin wraps a pointer-like type:
```
Pin<&mut T>
Pin<Box<T>>
```

Pin does not mean that the pointer variable itself cannot move.

For example:
`Pin<Box<MyFuture>>`

The Box pointer can move between variables, but the `MyFuture` value stored in
the heap remains at the same memory address.

What must stay fixed is the pointed-to value, not necessarily the pointer variable.

Conceptual picture
------------------

Pin<Box<Future>>
     |
     +-- Box pointer may move
             |
             +-- Future allocation remains at the same address
*/

// ==============================================================================
// 10. THE UNPIN TRAIT
// ==============================================================================
/*
Most Rust types do not contain references to themselves.

Examples:
```
i32
bool
String
Vec<T>
HashMap<K, V>
```
These values are normally safe to move.

`Unpin` is a marker trait that says:
"This type does not rely on remaining at one fixed memory address."

A type implementing Unpin can safely move even when accessed through Pin.
Most ordinary Rust types automatically implement Unpin.

The names are slightly counterintuitive:
```
Unpin
    = this type does NOT require pinning restrictions
    = safe to move

!Unpin
    = this type MAY rely on a stable memory address
    = must not move after being pinned
```

Important relationship
----------------------

Pin<T>
    establishes or represents a pinning guarantee

Unpin
    says that the type does not care about that guarantee


For an Unpin type:
Pin<&mut T> behaves almost like &mut T because moving T is safe.

For a !Unpin type:
Pin<&mut T> prevents safe code from moving T out of its location.

Many compiler-generated async Futures are !Unpin because their state machines
may become self-referential.
*/

// ==============================================================================
// 11. WHY `Box<dyn Future>` IS NOT ENOUGH
// ==============================================================================
/*
We tried:
```
Vec<Box<dyn Future<Output = ()>>>
```

A Box places the Future on the heap, and moving the Box itself does not normally
move the heap allocation.

However, an ordinary Box does not establish the complete pinning contract.
The Future API needs a pinned pointer before safely polling a potentially
self-referential Future.

Also, Box<F> implements Future only under the necessary Unpin conditions for
the wrapped Future.

But `dyn Future<Output = ()>` does not automatically implement Unpin.

Therefore `Box<dyn Future<Output = ()>>>` cannot be used by `join_all()`
as the required Future type in this example.

We need a type that explicitly says:
"The Future behind this pointer is pinned."

That type is:
`Pin<&mut dyn Future<Output = ()>>`

or:
`Pin<Box<dyn Future<Output = ()>>>`
*/

// ==============================================================================
// 12. FIXING THE FUTURES WITH `pin!`
// ==============================================================================

fn demo_join_all_with_pinned_futures() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        // `pin!` creates the Future and pins it in the current stack frame.
        // The result behaves like:
        // Pin<&mut AnonymousFutureType>
        let tx1_fut = pin!(async move {
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
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
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
        });

        // Each async block originally had a different anonymous concrete type.
        // We erase those differences with:
        // dyn Future<Output = ()>
        // We also guarantee that the Futures cannot move with:
        // Pin<&mut ...>
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![
            tx1_fut,
            tx_fut,
            rx_fut,
        ];

        trpl::join_all(futures).await;
    });
}


/*
Why this works
--------------

Original types:
```
tx1_fut: AnonymousFutureTypeA
tx_fut:  AnonymousFutureTypeB
rx_fut:  AnonymousFutureTypeC
```

After trait-object coercion:
```
tx1_fut: Pin<&mut dyn Future<Output = ()>>
tx_fut:  Pin<&mut dyn Future<Output = ()>>
rx_fut:  Pin<&mut dyn Future<Output = ()>>
```

Now:
1. Every Vec element has one common type.
2. The concrete Future types are hidden behind `dyn Future`.
3. The Futures are pinned and cannot move.
4. `join_all()` can poll all of them.
5. `.await` waits until every Future finishes.


Why `async move` matters for tx and tx1
--------------------------------------

The two sending Futures use:
`async move`

This moves `tx` and `tx1` into their respective Futures.

When both sending Futures finish:
+ tx is dropped
+ tx1 is dropped
+ no senders remain
+ the channel closes
+ rx.recv().await returns None
+ the receiver loop ends
+ rx_fut finishes
+ join_all finishes

Without `move`, the sending Futures could merely borrow the senders.

The original senders might then remain alive in the outer scope, causing
the receiver to wait forever for another message.
*/

// ==============================================================================
// 13. `pin!` VERSUS `Box::pin()`
// ==============================================================================
/*
There are two common ways to pin a Future.

A. Pin on the current stack
---------------------------
```
let future = pin!(async {
    do_something().await;
});
```

Conceptual type:
`Pin<&mut AnonymousFutureType>`

The pinned Future is tied to the current scope.

B. Pin inside a heap-allocated Box
----------------------------------
```
let future = Box::pin(async {
    do_something().await;
});
```

Conceptual type:
`Pin<Box<AnonymousFutureType>>`

This is useful when:
+ the Future must be returned from a function
+ the Future must outlive the current stack scope
+ ownership of the pinned Future must be transferred
+ dynamic storage is needed

For a heterogeneous collection, we may use:
`Vec<Pin<Box<dyn Future<Output = ()>>>>`

Compact comparison
------------------

pin!(future)
    -> usually stack-pinned
    -> Pin<&mut Future>
    -> convenient for local use

Box::pin(future)
    -> heap-pinned
    -> Pin<Box<Future>>
    -> convenient for owned or returned Futures
*/

// ==============================================================================
// 14. WHY NORMAL `.await` DOES NOT REQUIRE WRITING `pin!`
// ==============================================================================
/*
Normally, we simply write:
```
let output = some_future.await;
```

We do not usually write:
```
let future = pin!(some_future);
let output = future.await;
```

This is because `.await` handles the necessary pinning machinery implicitly
when directly awaiting a Future.

The explicit pinning problem appears in the book because we are doing something
more complicated:
1. Create several different anonymous Futures.
2. Erase their concrete types with `dyn Future`.
3. Store them in a Vec.
4. Pass the collection into join_all().
5. Ask join_all() to poll all of them.

Therefore:

Directly awaiting one Future:
    Rust handles pinning for us.

Manipulating heterogeneous Futures through trait objects and collections:
    we may need to express pinning explicitly.
*/

// ==============================================================================
// 15. THE STREAM TRAIT
// ==============================================================================
/*
The Stream trait is approximately:
```
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>>;
}
```

`Item`
------
A Stream produces zero or more values of type `Item`.

For example:
Stream<Item = i32>

may produce:
1
2
3
4
5

`poll_next`
-----------
`poll_next` asks the Stream:
"Is your next item ready?"

Its return type is:
Poll<Option<Self::Item>>

There are three meaningful possibilities:

Poll::Ready(Some(item))
    -> the next item is available

Poll::Pending
    -> the next item is not available yet
    -> poll this Stream again after it is woken

Poll::Ready(None)
    -> the Stream is permanently finished
*/

// ==============================================================================
// 16. HOW STREAM COMBINES ITERATOR AND FUTURE
// ==============================================================================
/*
Iterator gives us:
```
Option<Item>

Some(item)
    -> another item exists

None
    -> iteration has ended
```

Future gives us:
```
Poll<Output>

Ready(output)
    -> asynchronous work has finished

Pending
    -> asynchronous work is not ready
```

Stream combines them:
```
Poll<Option<Item>>
The outer Poll answers:
"Is something ready now?"

The inner Option answers:
"If ready, did we receive an item, or did the Stream finish?"
```

Examples
--------

Poll::Pending
    Nothing is ready yet.
    Do not treat the Stream as finished.


Poll::Ready(Some("hello"))
    A new item is available.
    Process it and later request another item.

Poll::Ready(None)
    The Stream has ended.
    Do not expect more items.
*/

// ==============================================================================
// 17. STREAMEXT AND `.next().await`
// ==============================================================================
/*
In application code, we usually do not call
`Stream::poll_next(...)` directly.

Instead, we import `use trpl::StreamExt;`
and write `stream.next().await`

Conceptually, StreamExt looks like:
```
trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;

    // Other convenience methods...
}
```

The actual ecosystem definition may instead look like:
```
fn next(&mut self) -> Next<'_, Self>
where
    Self: Unpin;
```

`Next<'_, Self>` is a named struct implementing Future.
Therefore `stream.next()` returns a Future, and
`stream.next().await` waits for the Stream's next item.

The relationship is:
```
Stream::poll_next(...)
    -> low-level polling interface

StreamExt::next().await
    -> high-level convenient interface
```

This is similar to:
```
Future::poll(...)
    -> low-level interface

future.await
    -> high-level convenient syntax
```
*/

// ==============================================================================
// 18. WHY STREAM AND STREAMEXT ARE SEPARATE
// ==============================================================================
/*
Stream
------
Defines the minimum foundational behavior:
+ associated type Item
+ poll_next()


StreamExt
---------
Provides convenient higher-level operations:
+ next()
+ map()
+ filter()
+ and many other Stream utilities

StreamExt is implemented automatically for types that implement Stream.

This means that when creating a custom Stream, we generally only implement
the low-level Stream trait.

Users can then import StreamExt and automatically gain all its convenience methods.

Why keep them separate?
-----------------------
The core Stream contract can remain small and stable.

The ecosystem can add or change convenience methods in StreamExt without
changing the foundational Stream trait.
*/

// ==============================================================================
// 19. FUTURE, STREAM, AND ITERATOR SUMMARY
// ==============================================================================
/*
Iterator
--------

Purpose:
    Produce many values synchronously.

Core associated type:
    Item

Core method:
    next()

Return type:
    Option<Item>

Usage:

while let Some(item) = iterator.next() {
    println!("{item}");
}

Future
------

Purpose:
    Produce one value asynchronously.

Core associated type:
    Output

Core method:
    poll()

Return type:
    Poll<Output>

Normal usage:

let output = future.await;

Stream
------

Purpose:
    Produce many values asynchronously over time.

Core associated type:
    Item

Core method:
    poll_next()

Return type:
    Poll<Option<Item>>

Normal usage:

use trpl::StreamExt;

while let Some(item) = stream.next().await {
    println!("{item}");
}
*/

// ==============================================================================
// 20. COMPLETE MENTAL MODEL
// ==============================================================================
/*
ASYNC BLOCK
-----------
```
async {
    ...
}
```

is compiled into an anonymous state-machine type implementing Future.

FUTURE
------
Future means:
"I represent one asynchronous computation that will eventually produce
one Output value."


POLL
----
Poll::Pending:
"I am not ready. Pause me and check again after I am woken."

Poll::Ready(output):
"I have finished. Here is my final output."

CONTEXT AND WAKER
-----------------
Context gives the Future access to a Waker.

The Waker tells the runtime when the Future may be ready to make progress.

AWAIT
-----
`future.await` is high-level syntax that drives the Future through repeated polling while
allowing the surrounding async task to pause when the Future is Pending.

DIFFERENT ASYNC BLOCK TYPES
---------------------------
Every async block has a unique compiler-generated concrete type, even if
multiple blocks have the same Future::Output type.

DYN FUTURE
----------
`dyn Future<Output = T>` erases those different concrete types
so they can be handled through one common trait-object type.

BOX
---
`Box<dyn Future<Output = T>>` provides owned storage
for a dynamically sized Future trait object.

However, Box alone does not express that
a potentially self-referential Future is pinned.

PIN
---
`Pin<P>` guarantees that the pointed-to `!Unpin` value cannot be moved through safe code.
This allows self-referential async state machines to remain valid.

UNPIN
-----
Unpin means:
"This type is safe to move even if it is behind Pin."

Most ordinary Rust types are Unpin.
Compiler-generated async Futures are often !Unpin.

JOIN_ALL
--------
`join_all(collection).await` runs every Future in a collection
and waits for all of them to finish.

For heterogeneous async-block Futures, we may need:
Vec<Pin<&mut dyn Future<Output = T>>>

or `Vec<Pin<Box<dyn Future<Output = T>>>>`

STREAM
------
Stream means:
"I represent an asynchronous sequence that can produce zero or more Item
values over time."

STREAMEXT
---------
StreamExt provides convenient methods such as `.next()`.

`stream.next().await` waits for one additional item or for the end of the Stream.
*/

// ==============================================================================
// 21. SHORTEST POSSIBLE SUMMARY
// ==============================================================================
/*
Future:
    one asynchronous value

Stream:
    many asynchronous values

poll():
    ask a Future whether its output is ready

poll_next():
    ask a Stream whether its next item is ready

Poll::Pending:
    not ready; wake and poll again later

Poll::Ready(value):
    ready with a value

Context/Waker:
    lets the Future notify the runtime when it can make progress

Pin:
    prevents a !Unpin value from moving after it has been pinned

Unpin:
    says a type is safe to move despite pinning

dyn Future:
    hides different concrete Future types behind one common interface

join!:
    join a fixed number of possibly different Future types

join_all():
    join a collection of Futures

StreamExt:
    supplies convenient Stream methods such as `.next()`
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_join_all_with_pinned_futures();
}
