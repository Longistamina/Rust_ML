// ==============
// Setup (inside chap_09_tap_pipe):
//     cargo add tap@1
// Run:
//     cargo run --bin 01_tap_pipe
// Also try release mode to compare demo_debug_tap:
//     cargo run --release --bin 01_tap_pipe
//
// tap is an extension-trait crate: importing its traits adds methods to values.
// use tap::prelude::* imports the traits used throughout this guide.
// For smaller imports, use tap::{Tap, Pipe}, etc.
//
// Essential signatures (simplified):
//     tap(self, closure: &Self -> ())             -> Self
//     tap_mut(self, closure: &mut Self -> ())     -> Self
//     pipe(self, closure: Self -> R)              -> R
//     pipe_ref(&self, closure: &Self -> R)        -> R
//     pipe_ref_mut(&mut self, closure: &mut Self -> R) -> R
// These are ordinary Rust method calls, not shell pipes or concurrent tasks.
//
// Official docs (tap 1.0.1):
// https://docs.rs/tap/latest/tap/
// https://docs.rs/tap/latest/tap/trait.Tap.html
// https://docs.rs/tap/latest/tap/trait.Pipe.html

use tap::prelude::*;

fn main() {
    println!();

    demo_tap();
    println!("\n============================\n");

    demo_tap_mut();
    println!("\n============================\n");

    demo_pipe();
    println!("\n============================\n");

    demo_tap_vs_pipe();
    println!("\n============================\n");

    demo_borrowed_pipes();
    println!("\n============================\n");

    demo_borrowed_views();
    println!("\n============================\n");

    demo_option();
    println!("\n============================\n");

    demo_result();
    println!("\n============================\n");

    demo_debug_tap();
    println!("\n============================\n");

    demo_conversions();
    println!("\n============================\n");

    demo_practical_pipeline();
}

// 01. tap: inspect a value and return that value
// ---------------------------------------------
fn demo_tap() {
    println!("01. tap: inspect, then keep going");

    let name = String::from("Rust")
        .tap(|text| println!("Before formatting: {text}"));
    // text: &String; the closure only borrows the String.
    // println! returns (), as required by tap's closure.
    // The entire tap expression returns the owned String, so name is String.
    println!("Returned value: {name}");

    // Equivalent ordinary Rust:
    let plain = String::from("Rust");
    println!("Before formatting: {plain}");
    println!("Returned value: {plain}");

    // tap takes self by value: calling name.tap(...) would move name into it.
    // It returns that ownership to the caller; it does not clone the value.
}

// 02. tap_mut: insert in-place operations into a chain
// --------------------------------------------------
fn demo_tap_mut() {
    println!("02. tap_mut: mutate, then return the collection");

    let numbers = vec![3, 1, 3, 2]
        .tap_mut(|values| values.sort_unstable())
        .tap_mut(|values| values.dedup())
        .tap(|values| println!("Sorted and deduplicated: {values:?}"));
    // Each mutation closure receives &mut Vec<i32> and returns ().
    // Each tap_mut returns the Vec itself, including the mutations.
    println!("Final Vec: {numbers:?}"); // [1, 2, 3]

    // Without tap_mut, use a mutable binding and call the methods separately.
    // The final binding above need not be mut: mutation happened in the chain.
    let greeting = String::from("hello")
        .tap_mut(|text| text.push_str(", Rust!"));
    println!("{greeting}");
}

// 03. pipe: pass a value to a function and return the function's output
// -------------------------------------------------------------------
fn demo_pipe() {
    println!("03. pipe: transform a value");

    fn double(value: i32) -> i32 {
        value * 2
    }

    let answer = 10
        .pipe(double)                    // double(10) -> 20
        .pipe(|value| value + 5)         // 20 + 5 -> 25
        .pipe(|value| format!("Answer: {value}")); // i32 -> String
    println!("{answer}");

    // pipe can change the type at every step. It takes self by value.
    // For non-Copy inputs such as String, that transfers ownership.
    fn surround(text: String, left: &str, right: &str) -> String {
        format!("{left}{text}{right}")
    }

    let label = String::from("Rust")
        .pipe(|text| surround(text, "[", "]"));
    println!("{label}"); // [Rust]
    // Functions with extra arguments need a closure to supply those arguments.
}

// 04. The critical distinction: which value leaves the expression?
// ----------------------------------------------------------------
fn demo_tap_vs_pipe() {
    println!("04. tap keeps the input; pipe returns the output");

    let values = vec![10, 20, 30]
        .tap(|values| println!("Length observed: {}", values.len()));
    // values is still Vec<i32>; tap's closure receives &Vec<i32>.

    let length = values.pipe(|values| values.len());
    // length is usize; the closure receives the owned Vec and returns its length.
    // The Vec was moved into the closure and is dropped when the closure ends.
    println!("Length returned: {length}");

    // WRONG (does not compile): vec![1, 2].tap(|v| v.len())
    // tap's closure must return (), but len() returns usize.
    // Use pipe to return the length, or print it inside tap to keep the Vec.
    // Prefer named intermediate variables whenever they make a chain clearer.
}

// 05. pipe_ref and pipe_ref_mut: borrow an existing value
// ------------------------------------------------------
fn demo_borrowed_pipes() {
    println!("05. Borrow without moving the owner");

    let mut numbers = vec![3, 1, 2];

    let total: i32 = numbers.pipe_ref(|values| values.iter().sum());
    // closure input: &Vec<i32>; output: i32. numbers remains available.
    println!("total={total}, original={numbers:?}");

    let length = numbers.pipe_ref_mut(|values| {
        values.sort_unstable();
        values.push(4);
        values.len()
    });
    // closure input: &mut Vec<i32>; output: usize.
    // This returns the closure's result, unlike tap_mut, which returns Self.
    println!("length={length}, changed={numbers:?}");

    let text = String::from("  hello  ");
    let trimmed: &str = text.pipe_ref(|value| value.trim());
    println!("Borrowed substring: {trimmed:?}, owner: {text:?}");
    // A borrowed pipe can return a borrow tied to the original owner.
    // WRONG: String::from("hello").pipe(|value| value.as_str())
    // The owned String would be dropped inside the closure: dangling reference.
}

// 06. View adapters: String as str, Vec<T> as [T]
// ----------------------------------------------
fn demo_borrowed_views() {
    println!("06. Borrow a view of a value");

    let text = String::from("hello")
        .tap_ref(|view: &str| println!("AsRef<str>: {view}"))
        .tap_borrow(|view: &str| println!("Borrow<str>: {view}"))
        .tap_deref(|view: &str| println!("Deref<Target=str>: {view}"));
    // These taps still take/return String; only the closure's borrowed view differs.
    // An explicit parameter type selects the desired view when inference needs help.

    let length = text.pipe_as_ref(|view: &str| view.len());
    let words = text.pipe_borrow(|view: &str| view.split_whitespace().count());
    let upper = text.pipe_deref(|view: &str| view.to_uppercase());
    println!("length={length}, words={words}, upper={upper}, owner={text}");
    // These pipes borrow text and return their closures' outputs.

    let mut values = vec![3, 1, 2];
    values.pipe_as_mut(|view: &mut [i32]| view.sort_unstable());
    println!("Sorted through a slice: {values:?}");
    // Mutable counterparts also exist: tap_ref_mut/tap_borrow_mut/tap_deref_mut
    // and pipe_as_mut/pipe_borrow_mut/pipe_deref_mut.
    // Borrow carries stronger semantic requirements than AsRef (e.g. matching
    // equality/hash behavior); Deref follows a type's dereference implementation.
}

// 07. Option-specific taps: inspect Some, mutate Some, react to None
// ----------------------------------------------------------------
fn demo_option() {
    println!("07. Conditional tapping of Option");

    let present = Some(String::from("rust"))
        .tap_some(|text| println!("Found: {text}"))
        .tap_some_mut(|text| text.make_ascii_uppercase())
        .tap_none(|| println!("This closure is skipped"));
    println!("Returned Option: {present:?}"); // Some("RUST")
    // tap_some receives &String; tap_some_mut receives &mut String.
    // Both return the whole Option<String>, not the inner String.

    println!();

    let missing: Option<String> = None;
    let still_missing = missing
        .tap_some(|text| println!("Skipped: {text}"))
        .tap_none(|| println!("No value found"));
    println!("Still missing: {still_missing:?}");
    // tap_none takes a no-argument closure. It observes absence, not a replacement.
    // Use map to transform Some, or unwrap_or_else to provide a fallback.
}

// 08. Result-specific taps: observe success/error without handling it away
// -----------------------------------------------------------------------
fn demo_result() {
    println!("08. Conditional tapping of Result");

    fn parse_count(text: &str) -> Result<u32, std::num::ParseIntError> {
        let count = text.parse::<u32>()
            .tap_ok(|value| println!("Parsed successfully: {value}"))
            .tap_err(|error| println!("Parsing failed: {error}"))?;
        // tap_ok receives &u32; tap_err receives &ParseIntError.
        // Both return the original Result; ? still propagates Err afterward.
        Ok(count + 1)
    }

    println!("Caller receives: {:?}", parse_count("41")); // Ok(42)

    println!();

    println!("Caller receives: {:?}", parse_count("wrong")); // Err(...)
    // Logging an error is not recovery. Use or_else/match to recover;
    // use map_err to change the error type.
    // tap_ok_mut and tap_err_mut also exist to mutate the selected payload.
}

// 09. Debug-only tapping
// ----------------------
fn demo_debug_tap() {
    println!("09. tap_dbg: conditional on debug_assertions");

    let numbers = vec![1, 2, 3]
        .tap_dbg(|values| println!("Debug observation: {values:?}"));
    println!("Always printed: {numbers:?}");
    // Normally the closure runs with cargo run, and is skipped with --release.
    // The exact switch is cfg(debug_assertions), which profiles can customize.
    // tap_dbg does not automatically format anything: supply your own closure.
    // Other tap families have _dbg counterparts too.
    // Keep required mutations/validation out of debug-only closures!
}

// 10. Bonus: explicit conversions inside a chain
// ----------------------------------------------
fn demo_conversions() {
    println!("10. conv and try_conv");

    let upper = "hello"
        .conv::<String>()
        .tap_mut(|text| text.make_ascii_uppercase());
    println!("{upper}");
    // conv::<T>() calls Into<T>; the target type is written at the method call.
    // This avoids an ambiguous .into() in the middle of some chains.

    let small = 200_u16.try_conv::<u8>();
    println!("Fits in u8: {small:?}"); // Ok(200)

    println!();

    let too_large = 300_u16.try_conv::<u8>();
    println!("Does not fit in u8: {too_large:?}"); // Err(...)
    // try_conv::<T>() calls TryInto<T> and returns Result<T, _>.
}

// 11. Practical pipeline: tokenize, inspect, normalize, sort, then format
// ----------------------------------------------------------------------
fn demo_practical_pipeline() {
    println!("11. Combining tap, tap_mut, and pipe");

    let summary = " rust, cargo, rust, clap "
        .split(',')
        .map(str::trim)
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect::<Vec<String>>()
        .tap(|words| println!("Collected: {words:?}"))
        .tap_mut(|words| words.sort_unstable())
        .tap_mut(|words| words.dedup())
        .tap(|words| println!("Unique: {words:?}"))
        .pipe(|words| format!("{} tools: {}", words.len(), words.join(", ")));
    println!("{summary}"); // 3 tools: cargo, clap, rust

    // split/map/filter/collect/sort_unstable/dedup/join are standard Rust APIs.
    // tap/tap_mut/pipe are the crate's additions connecting the steps.
    // Iterator::inspect observes each item lazily while an iterator is consumed.
    // tap here observes the entire collected Vec immediately at that step.
    // For plain Option/Result inspection, std also offers inspect/inspect_err;
    // tap adds general value taps, mutable taps, view adapters, and piping.
}
