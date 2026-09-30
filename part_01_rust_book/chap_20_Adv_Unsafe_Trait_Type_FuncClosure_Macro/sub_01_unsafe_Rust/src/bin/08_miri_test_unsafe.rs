/*
The compiler prevents many problems statically.

`Miri` runs Rust code in an interpreter
that detects many forms of undefined behavior at runtime,
including invalid pointer use.

```
rustup +nightly component add miri
cargo +nightly miri test
```
*/

fn main() {}
