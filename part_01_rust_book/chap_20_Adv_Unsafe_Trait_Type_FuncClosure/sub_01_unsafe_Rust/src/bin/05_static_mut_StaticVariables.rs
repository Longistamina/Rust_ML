/*
Example of a static variable:
`static HELLO_WORLD: &str = "Hello, world!";`

Unlike a `const`, a `static` has one fixed memory location for the program’s lifetime.
Therefore, a mutable global like this is dangerous.
```
static mut COUNTER: u32 = 0;
```

Why? Because if two threads read/write it at the same time,
you can get a data race, which is undefined behavior in Rust.
That is why access requires unsafe code.

In real Rust, prefer safe shared-state tools:
- `AtomicU32` for a simple counter
- `Mutex<T>` for protected shared mutable state
- `channels` for sending ownership between threads
`static mut` is largely a last resort for tightly controlled low-level code.
*/

static HELLO_WORLD: &str = "Hello, world!"; // static constant
static mut COUNTER: u32 = 0; // static mut: unsafe

/// SAFETY: Calling this from more than a single thread at a time is undefined behavior,
/// so you *must* guarantee you only call it from a single thread at a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!();

    println!("static const = {}", HELLO_WORLD);

    unsafe { // open unsafe block to modify `static mut` variable
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("static mut COUNTER: {}", *(&raw const COUNTER));
    }
}
