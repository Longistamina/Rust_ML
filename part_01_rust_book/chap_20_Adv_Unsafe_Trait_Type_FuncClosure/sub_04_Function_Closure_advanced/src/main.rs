/*
This part explains advanced features of functions and closures,
including:
+ Function pointers
+ Return closures

Some distinction:
`fn(...) -> ...` is a function pointer: it points to an ordinary named function.
`Fn(...) -> ...` is a trait implemented by closures and function pointers.

The lowercase `fn` is a type.
The uppercase `Fn` is a trait.
*/
fn main() {
    println!("Hello, world!");
}
