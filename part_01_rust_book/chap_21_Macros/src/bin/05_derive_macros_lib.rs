//! Consumer-side example for the derive macro defined in `src/lib.rs`.
//!
//! Cargo compiles `src/lib.rs` as the `proc-macro` library and this file as a
//! separate binary crate. Procedural macros cannot be used inside the crate
//! that defines them, but this binary is a different crate, so it can import
//! and use the derive macro.
//!
//! Before compiling, replace `your_package_name` below with the library crate
//! name from `Cargo.toml`. It is usually the `[package]` name with `-` changed
//! to `_` (unless `[lib] name = "..."` overrides the library name).

// Import the procedural derive macro exported by src/lib.rs.
// This puts `HelloMacro` in the macro namespace, where `#[derive(...)]` looks.
// use your_package_name::HelloMacro;
use chap_21_Macros::HelloMacro;

// This is the trait that the generated implementation implements.
// The derive macro crate cannot export this ordinary trait, so the consuming
// crate defines it here. The trait and derive macro can share the name because
// Rust keeps type names and macro names in separate namespaces.
trait HelloMacro {
    // An associated function; the generated impl will provide its body.
    fn hello_macro();
}

// Ask the procedural macro from lib.rs to inspect this struct and append code.
// The derive receives the struct's tokens; it extracts the name `Pancakes` and
// generates an implementation equivalent in spirit to:
//
// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is {}!", stringify!(Pancakes));
//     }
// }
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    println!();

    // This call works because the derive has supplied the trait implementation
    // during compilation. It is an ordinary function call at runtime.
    Pancakes::hello_macro();
}

// Expected output:
// Hello, Macro! My name is Pancakes!
