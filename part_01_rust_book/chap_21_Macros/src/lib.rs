//! This file defines the *procedural derive macro* `#[derive(HelloMacro)]`.
//!
//! It belongs in the root of a crate whose `Cargo.toml` contains:
//!
//! ```toml
//! [lib]
//! proc-macro = true
//!
//! [dependencies]
//! syn = "2.0"
//! quote = "1.0"
//! ```
//!
//! This crate provides the macro, but does not define the `HelloMacro` trait.
//! The generated `impl HelloMacro for ...` is added to the code using the
//! derive, so that consuming crate must define or import the trait.

// `TokenStream` is the compiler's token-level representation of Rust source.
// A procedural macro receives input tokens and returns the tokens to append.
use proc_macro::TokenStream;

// `quote!` helps construct Rust code as tokens without manually assembling
// punctuation and identifiers.
use quote::quote;

// This attribute registers the public function below as a derive macro named
// `HelloMacro`, used by callers like `#[derive(HelloMacro)]`.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the annotated item (for example, `struct Pancakes;`) into a
    // structured syntax tree. `DeriveInput` stores details such as its name,
    // fields, generics, and whether it is a struct, enum, or union.
    //
    // `unwrap()` keeps this teaching example short. A production macro should
    // report a useful compiler error instead of panicking if parsing fails.
    let ast = syn::parse(input).unwrap();

    // Generate the implementation using the parsed item.
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // `ident` is the annotated type's identifier. For `struct Pancakes;`,
    // `name` represents `Pancakes`.
    let name = &ast.ident;

    // Describe the Rust code we want the compiler to add. In `quote!`,
    // `#name` interpolates the Rust variable `name` into the generated tokens.
    // Thus, deriving on `Pancakes` produces `impl HelloMacro for Pancakes`.
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // `stringify!` turns the type-name tokens into a string literal
                // at compile time, so the message can include "Pancakes".
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // `quote!` returns its own token-stream type; convert it into the
    // `proc_macro::TokenStream` required by the procedural-macro interface.
    generated.into()
}
