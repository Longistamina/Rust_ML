/*
There are several concepts that must be settled down first.

`future`: a value that may not be ready now but will become ready at some point in the future.
In Rust, any type that implements `Future` trait are all futures.
Each future holds its own information about the progress that has been made and what “ready” means.

`async` is a keyword to blocks and functions to specify that they can be interrupted and resumed.

`await` is a keyword that can be used in an async block or an async function
to `await` a `future` value (that is, wait for it to become ready).

Any point where you await a future within an async block or function
is a potential spot for that block or function to pause and resume.
The process of checking with a future to see if its value is available yet is called polling.

Because Rust provides the `Future` trait, though,
you can also implement it for your own data types when you need to.

---------------------------------------------------------------------

NOTE: `futures` in Rust are LAZY!!!
      They don’t do anything until you ask them to with the `.await` keyword.
      Only when we use `.await`, that lazy `future` will start "waiting" for something.

This is something like `.iter()` creates a lazy representation,
only when we call `.iter().collect()` that it realizes everything.

---------------------------------------------------------------------

At each `await` point, Rust gives a runtime a chance to pause the task
and switch to another one if the future being awaited isn’t ready.

The inverse is also true: Rust only pauses `async` blocks
and hands control back to a runtime at an `await` point.

Everything between await points is synchronous.

For example:
    `thread::sleep(Duration::from_millis(50))` will pause the all the tasks and block the whole thread for 50ms
    `async {trpl::sleep(Duration::from_millis(50)).await}` will pause this async task for 50ms, but does not block other tasks of the thread

---------------------------------------------------------------------

Let's write a little web scrapper.
We’ll pass in two URLs from the command line,
fetch both of them concurrently, and return the result of whichever one finishes first.
*/

// =====================================================================
// 0. `fututres`, `tokio` and `trpl`
// =====================================================================
/*
The `futures` crate is an official home for Rust experimentation for async code,
and it’s actually where the `Future` trait was originally designed.

`tokio` is the most widely used async runtime in Rust today, especially for web applications.

However, we will not use them here, we will use `trpl` (stands for "The Rust Programming Language").
It is a wrapper that re-exports all the types, traits, and functions you’ll need from those 2 crates
=> make it easier for education and demonstrating.
*/

// =====================================================================
// 1. Defining the `page_title()` function with `async` keyword
// =====================================================================
/*
This function will take the URL of the page, make a request to it,
and finally return the text of the `<title>` element (along with the original url)
*/

use trpl::Html;

async fn page_title(url: &str) -> (&str, Option<String>) { // use `async` keyword here to specify that this function can be interrupted and resumed

    let response = trpl::get(url).await;
    // `trpl::get(url)` to get wat ever the `url` passed in, returns a lazy representation
    // `.await` to tell it to wait for a `future` value, no more lazy
    // (this step is asynchronous because it has to wait for the server to send back the first part of its response,
    // which will include HTTP headers, cookies, and so on)

    let response_text = response.text().await;
    // `response.text()` to get the text of the `response`, also returns a lazy representation
    // here also call `.await` to tell it to wait for a `future` value, no more lazy
    // (this step is also asynchronous because it has to wait for the entire `response` to arrive)

    /*
    We have to explicitly `.await` both of these futures, because futures in Rust are lazy:
    they don’t do anything until you ask them to with the `.await` keyword.
    (In fact, Rust will show a compiler warning if you don’t use a future.)
     */

    let response_text_2 = trpl::get(url).await.text().await; // we can also chain both steps like this
    drop(response_text_2);

    let title = Html::parse(&response_text) // parsing raw string `response_text` into an instance of Html type.
        .select_first("title") // a Html instance can have many "title" elements, but here we get only the first one (returns Option<ElementRef>)
        .map(|title| title.inner_html()); // `.map()` here is `Option<ElementRef>.map()`, this lets us work with the item in the Option if it’s present, and do nothing if it isn’t (could use a `match` here)
                                         // call `inner_html()` on the title to get its content, which is a String. When all is said and done, we have an Option<String>.

    (url, title) // return both url and title as (&str, Option<String>)
}

/*
BIG NOTE!!!!!!!!!!!!!!!

Because we use `async` to create `page_title()` function,
later when we call it like `page_title(&input)`,
this call call creates a lazy future representing the function’s entire body.
=> calling it does NOT enter the function body to execute the task

More specifically, when you call `let output = page_title(&input);`,
Rust behaves like this:
```
let output = Future {
    state: NotStarted,
    url: &input,
};
```

Again, it does not enter the function body, that's why the line
```let response = trpl::get(url).await;```
will not be executed, be it has not been yet reached.
*/

// ==============================================================================
// 2. Executing an Async Function with a Runtime (not `async main`)
//    Let's begin the race between 2 URLs
// ==============================================================================
/*
Calling an `async` function does not execute it to completion.
It creates a `Future`, and something must repeatedly run—or “poll”—that future.

If you use async with main like this
```
async fn main() {
    let title = page_title(url).await;
}
```
=> the code will not compile, because Rust requires the program entry point
    to be an ordinary synchronous function.

In other words, the `main` must be synchronous, but `.await` requires an async context
=> this conflict makes the compiler panick.

Therefore, it is NOT ALLOWED to use any `async` codes inside `main()`

======== SOLUTION ========

The solution is to keep main synchronous and `create an async block inside it`.
=> use `trpl::block_on(async {})`
*/

fn main() { // synchronous `main()`
    println!();

    let args: Vec<String> = std::env::args().collect(); // collect environment variable

    trpl::block_on(async { // create an asynchronous block inside the main
        let title_fut_1 = page_title(&args[1]); // save the resulting future value as `title_fut_1`, contain both (url, title)
        let title_fut_2 = page_title(&args[2]);
        /*
        Remind again, because `page_title()` is created with `async` keyword,
        calling it creates a lazy future representing the function’s entire body,
        so Rust will not enter its body to execute the task inside

        => `title_fut_1` and `title_fut_2` here are LAZY!!!
        => they represent pending works.
        */

        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                /*
                `trple::select()` returns a value `trple::Either` to indicate which of the futures finishes first.
                The Either type is somewhat similar to a Result in that it has two cases.
                Unlike Result, though, there is no notion of success or failure baked into Either.
                Instead, it uses `Left` and `Right` to indicate “one or the other”:
                */
                trpl::Either::Left(left) => left, // `title_fut_1` is mapped to `trpl::Either::Left`
                trpl::Either::Right(right) => right, // `title_fut_2` is mapped to `trpl::Either::Right`
                /*
                `trple::select()` only returns either `Left` or `Right`, not both.

                If it returns `Left` -> `title_fut_1` wins -> we got the (url, maybe_title) of the url_1
                If it returns `Rightt` -> `title_fut_2` wins -> we got the (url, maybe_title) of the url_2

                => Only the winner is printed out below
                */
            };

        /*
        `trple::select()` also returns a new LAZY `future`,
        so we must call `trple::select().await` tell it to start "waiting" for something.

        When we call `.await` here,
        it will also initialize `title_fut_1` and `title_fut_2`,
        hence both of them now are not lazy anymore,
        and all the `.await` in the `page_title()` now also woken up
        and start their job of "waiting" for something (for the `response`, and the `.text()`)
        */

        println!("{url} returned first"); // print out the winner
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    }) // Go out of `async` block here
}

/*
// let's use `URL_1 = https://www.google.com/` and `URL_2 = https://brave.com/`

cd /path/to/chap_17_Parallel_Async_Await_Futures_Streams
cargo run --bin 01_Futures_Async_Await_AsyncBlock https://www.google.com/ https://brave.com/
*/

// =============================================================================
// 3. Explain more about ``async {}`` and ``trpl::block_on()``
// =============================================================================
/*
`async {... do something ...}` is a async block,
it does not immediately execute its body.
It creates a Future representing this unfinished computation.

For example:
```
let future = async {
    match page_title(url).await {
        Some(title) => println!("{title}"),
        None => println!("No title"),
    }
};
```

At this point, `future` describes what should happen, but it still needs to be executed,
and you must also set up this:
```
trpl::block_on(future);
```

Or combine them like this:
```
trpl::block_on(async {
    match page_title(url).await {
        Some(title) => println!("{title}"),
        None => println!("No title"),
    }
};

----------------------------------------------------------------------

So what does ``trpl::block_on()`` do here?
It accepts a `future` like the above said `let future = async {... do something ...}`

It then:
1. Sets up the machinery needed to execute the future.
2. Polls the future to let it make progress.
3. Handles pauses caused by `.await`.
4. Wakes and polls the future again when it can continue.
5. Blocks the calling thread until the future finishes.
6. Returns the value produced by the future.

main()
  |
  | creates async block
  v
Future
  |
  | passed to `block_on()`
  v
Runtime repeatedly polls Future
  |
  +-- Ready(value)   → return value
  |
  +-- Pending        → wait and poll again later
```

----------------------------------------------------------------

`block_on()` blocks the thread containing main until the top-level future finishes.
But inside that future, the runtime can manage many async tasks concurrently:

main thread waits inside `block_on()`
                  |
                  v
         async runtime
         ├── download A
         ├── download B
         └── timer C

So when `download A` is waiting for the network, the runtime can work on B or C.
*/

// =============================================================================
// 4. Why doesn’t Rust provide one built-in runtime?
// =============================================================================
/*
Rust defines the common `Future` abstraction and the `async/.await` syntax,
but it does not force every program to use the same executor.
Different environments have different needs.

Web server:
- many concurrent connections
- multiple CPU cores
- plenty of memory

Microcontroller:
- possibly one CPU core
- very little memory
- perhaps no heap allocation

A single mandatory runtime would not suit all of them.
Rust therefore lets libraries such as `Tokio` provide the runtime implementation.
*/

// =============================================================================
// 5. Is `async main()` possible?
// =============================================================================
/*
Yes, but must use other runtime (crate) like `tokio`.
For example:
```
#[tokio::main]
async fn main() {
    // async code
}
```

Although that looks like Rust suddenly permits `async fn main`,
the `#[tokio::main]` macro rewrites it into something conceptually similar to:
```
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async {
        // async code
    });
}
```
*/

// =============================================================================
// 6. The bottom line:
// =============================================================================
/*
`async` creates a future,
`.await` waits for a future inside async code,
and a `runtime` (like tokio) actually drives the future to completion.
*/
