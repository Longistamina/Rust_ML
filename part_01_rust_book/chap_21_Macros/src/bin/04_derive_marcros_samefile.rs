// A single-file alternative for demonstrating the idea behind a derive macro.
//
// A real #[derive(HelloMacro)] procedural macro must live in a separate
// proc-macro crate. This declarative macro keeps the example in one file and
// generates the HelloMacro implementation for the type passed to it.

pub trait HelloMacro {
    fn hello_macro();
}

macro_rules! impl_hello_macro {
    ($type:ty) => {
        impl HelloMacro for $type {
            fn hello_macro() {
                println!(
                    "Hello, Macro! My name is {}!",
                    stringify!($type)
                );
            }
        }
    };
}

struct Pancakes;

impl_hello_macro!(Pancakes);

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    Pancakes::hello_macro();
}
// Hello, Macro! My name is Pancakes!
