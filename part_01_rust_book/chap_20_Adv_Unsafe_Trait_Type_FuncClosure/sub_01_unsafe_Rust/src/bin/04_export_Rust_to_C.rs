#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Hello from Rust");
}
/*
- `extern "C"` makes Rust use C-compatible calling conventions.
- `pub` exposes the symbol.
- `no_mangle` preserves the name call_from_c so C code can find it.
no_mangle is marked unsafe because a duplicate exported symbol can cause linker-level conflicts.
*/

fn main() {}
