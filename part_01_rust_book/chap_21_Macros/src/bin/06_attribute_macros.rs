//! Attribute-like procedural macro: an attribute plus the item it decorates.
//!
//! This file is a teaching example, not a standalone compilable program.
//! Procedural macro definitions belong in a crate whose Cargo.toml has
//! `[lib] proc-macro = true`. The example use below belongs in a different,
//! consuming crate; a procedural macro cannot be invoked in its own crate.

/*
Imagine a web framework lets an application write this in its binary crate:
```
    #[route(GET, "/")]
    fn index() {
        // Handler body
    }
```

The `route` attribute receives two separate token streams:
1. `attr` contains the attribute arguments: `GET, "/"`.
2. `item` contains the annotated item: the entire `fn index() { ... }`.

The macro can parse both streams, use the method and path to build routing
metadata, and return generated Rust code. For example, it could return the
original `index` function plus a registration item that associates GET `/`
with that function. The compiler then compiles the returned tokens as if they
had been written in the consuming crate.

The original function is not automatically preserved: if the macro wants it
to remain, its output must include it (possibly transformed).
*/

// This is the procedural-macro crate's implementation-side signature.
// `proc_macro::TokenStream` is Rust's compiler-facing representation of tokens.
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // `attr` is the text-like token input from inside #[route(...)] .
    // A real macro would parse it, for example into a method and path.
    let _route_arguments = attr;

    // `item` is the function or other Rust item carrying #[route]. A real
    // macro would inspect or modify it and usually include it in its output.
    // Returning it unchanged makes this a pass-through attribute for now.
    item
}

/*
Contrast with `#[derive(...)]`: a derive macro is attached to a struct/enum/
union and normally appends generated items, such as a trait implementation.
An attribute-like macro receives a general item and can replace or augment it.
*/
