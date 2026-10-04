//! Function-like procedural macro: a macro invocation whose contents are
//! processed by Rust code at compile time.
//!
//! This file illustrates the two sides of a function-like procedural macro.
//! In a real project, the definition belongs in a crate whose Cargo.toml has
//! `[lib] proc-macro = true`; the invocation belongs in a different crate.

/*
A consumer might write:
```
    let query = sql!(SELECT * FROM posts WHERE id = 1);
```

Although it looks like a function call, this does not call an ordinary `sql`
function at runtime. The compiler sends the tokens between the delimiters to
the procedural macro before compiling the program.

Unlike a `macro_rules!` macro, the procedural macro can run Rust code to parse
and validate those tokens as a small SQL language. It then returns Rust tokens.
Because this invocation appears where an expression is expected, its output
must be a valid Rust expression. For example, it might expand conceptually to:
```
    SqlQuery::new("SELECT * FROM posts WHERE id = 1")
```

That generated expression is then compiled and evaluated like ordinary code.
The macro itself does not automatically execute the SQL against a database;
that would be the job of the generated code at runtime.
*/

use proc_macro::TokenStream;

// `#[proc_macro]` registers this public function as a function-like macro named
// `sql`, called with `sql!(...)` by a different crate.
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // `input` contains only the tokens inside the invocation's delimiters:
    // for sql!(SELECT ...), it contains SELECT ... .

    // A real implementation would parse the SQL tokens (often with a parser
    // library), report a compile-time error for invalid SQL, and use `quote!`
    // or another approach to generate Rust tokens for the caller.

    // Placeholder: return the input unchanged. This is not a useful SQL macro,
    // and raw SQL tokens generally are not a valid Rust expression, but it
    // makes clear that the function must return a TokenStream.
    input
}

/*
The `sql` macro is not the same thing as `macro_rules! sql { ... }`:

- `macro_rules!` selects an output using declared token-pattern arms.
- A function-like procedural macro receives tokens and runs Rust code to
  decide how to parse and transform them.
*/
