/*
A `Stream` is an asynchronous `Iterator`: instead of producing all its items immediately,
it may need to wait before producing each next item.

------------------------
Stream vs Future
------------------------

A `Future` produces at most one final value:
```
let response = trpl::get(url).await;

Future<Output = T>

Pending
Pending
Ready(T)       ← finished forever
```

A `Stream` can produce many values over time:
```
Stream<Item = T>

Pending
Ready(Some(T)) ← produce first value T
Pending
Ready(Some(T)) ← produce second value T
Ready(Some(T))
Pending
Ready(None)    ← stream finished forever
```

So, when we call `rx.recv()` in an async message passing channel,
it will produce a `stream`.

-------------------------------------
Stream and `.next()` and Iterator
-------------------------------------

Calling `stream.nex()` does not return the next item (Option<T>),
instead it returns `Future<Output = Option<Item>>`.

To get the Option<T>, you must use `stream.next().await`
so that it will wait until either:
+ `Some(item)`: another item becomes available
+ `None`: the stream ended

This is the core difference between an Iterator and a Stream.
Since Iterator is synchronous, you don't need to wait,
just `iterator.next()`, because an iterator already has all its data.

-------------------------------------------------------------------
`trpl::stream_from_iter()`: create a stream from a know iterator
--------------------------------------------------------------------

This wraps the synchronous iterator with the Stream interface.

---------------------------------------------------------------------------
`trpl::StreamExt`: allows calling `stream.next()` and `stream.try_next()`
---------------------------------------------------------------------------
The `stream.next()` and `stream.try_next()` methods are supplied by
`StreamExt` extension trait, which is not in scope by default
=> You have to import it.

`stream.next()`
-> Return `Option<T>`: Some(item) or None
-> The inputs should look like: `vec![1, 2, "3", "four", "six"]`

`stream.try_next()`
-> Return `Result<Option<T>, E>`: Ok(Some(item)) or Err(error)
-> The inputs should look like: `vec![Ok(1), Ok(2), Ok("3"), Err("some_error"), Ok("six")]
-> It will stop when encounter the first `Err(error)`
*/

use trpl;
use trpl::StreamExt; // allow `stream.next()` and `stream.try_next()`

fn main() {
    println!();

    trpl::block_on(async {
        println!("\n===================== next() ============================\n");

        let values = vec![1, 2, 3, 4, 5];
        let iter = values.iter().map(|n| n * 2);

        let mut stream = trpl::stream_from_iter(iter);
        // like iterator, to use `next()`, we must make stream mutable,
        // because calling `next()` modify the stream internal state

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
        /*
        The value was: 2
        The value was: 4
        The value was: 6
        The value was: 8
        The value was: 10
        */

        println!("\n=================== try_next() ==========================\n");

        let str_nums = vec!["1", "2", "three", "four", "5", "6"];
        let iter_nums = str_nums.iter().map(|x| x.parse::<f32>());
        // `.iter().map(|x| x.parse::<f32>()` will produce something like `vec![Ok(1.0), Ok(2.0), Err(_), Err(_), Ok(5.0), Ok(6.0)]
        // => use `try_next()`

        let mut stream = trpl::stream_from_iter(iter_nums);

        while let Ok(item) = stream.try_next().await {
            println!("The value was: {:?}", item);
        }
        /*
        The value was: Some(1.0)
        The value was: Some(2.0)

        ---------------------------

        "three" -> parse into `Err(_)` -> break the `while let ...` loop
        */
    })
}
