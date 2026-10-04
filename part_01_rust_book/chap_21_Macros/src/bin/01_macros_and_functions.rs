/*
`macros` are a way of writing code that writes other code,
which is known as metaprogramming. In Rust, macros generate
Rust codes during compilationg.

We have already met several macros: `println!`, `vec!`, and `#[derive(...)]`
All of these macros expand to produce more code than the code you’ve written manually.

-----------------------------------------------------------------------------------------

A function has a fixed number and type of parameters.

Meanwhile, macros can take a variable number of parameters.
For example with `println!`, we can write
    `println!("hello")` -> 1 argument
    `println!("hello {}", name)` -> 2 arguments

-----------------------------------------------------------------------------------------

Macros generate the expanded code before the compiler interprets its meaning.
So a macro can, for example, implement a trait on a given type.

A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.

-----------------------------------------------------------------------------------------

The downside to implementing a macro instead of a function is that
macro definitions are more complex than function definitions
because you’re writing Rust code that writes Rust code.

Due to this indirection, macro definitions are generally more difficult to read,
understand, and maintain than function definitions.

-----------------------------------------------------------------------------------------

Another important difference between macros and functions is that
you must define macros or bring them into scope before you call them in a file.

Meanwhile, a function can be defined anywhere and called anywhere.
*/

fn main() {
    println!("\nHi, I am the macro println!")
}
