/*
All the code we’ve discussed so far has had Rust’s memory safety guarantees enforced at compile time.
However, Rust has a second language hidden inside it that doesn’t enforce these memory safety guarantees:
It’s called `unsafe Rust` and works just like regular Rust but gives us extra superpowers.

Safe Rust rejects code whenever the compiler cannot confidently prove it obeys memory rules.
That is intentional: rejecting a few valid programs is better than accepting a program that corrupts memory.

But some work is inherently beyond those proofs:
- talking to C libraries or operating systems
- interacting with hardware or fixed memory addresses
- implementing a low-level data structure more efficiently
- expressing a fact you know is true but the borrow checker cannot infer

So unsafe is Rust’s explicit boundary between:
- compiler-enforced guarantees, and
- guarantees you must justify yourself.
Keep unsafe blocks tiny, and ideally hide them behind a normal safe API.

Inside `unsafe { ... }`, you may:
1. Dereference a raw pointer.
2. Call an unsafe function/method.
3. Read or write a mutable global (`static mut`).
4. Implement an unsafe trait.
5. Read a `union` field.
*/

fn main() {
    println!("Hello, world!");
}
