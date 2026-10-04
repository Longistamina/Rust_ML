/*
Declarative macros use patterns and substitutions.
Procedural macros use Rust code to inspect input tokens and produce output tokens.

```
use proc_macro::TokenStream;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    // inspect input and generate output
}
```

A `TokenStream` is the macro’s representation of Rust source.
Procedural macros must live in a special crate with `proc-macro = true` in its manifest.

We will go through 3 different kinds of procedural macros:
+ custom derives
+ attribute-like macros
+ function-like macros
*/
