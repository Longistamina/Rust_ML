/*
A trait becomes unsafe when implementing it requires upholding a rule that the compiler cannot verify:
```
unsafe trait Foo {}

unsafe impl Foo for i32 {}
```

The most important real examples are `Send` and `Sync`:
- `Send`: a value may move to another thread.
- `Sync`: shared references to a value may be used across threads.

Usually Rust derives these automatically.
But if your custom type includes raw pointers,
you might need `unsafe impl Send` or `unsafe impl Sync`.
Doing so means you have manually verified it truly remains thread-safe.
*/

fn main() {}
