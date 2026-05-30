fn main() {
    println!("\nHello, world!");
}

// fn main() {}: defines a function named "main", actually this is the main function that will wrap "smaller" functions, and run (like def main() in Python)
// {}: the function body is wrapped in this brackets {}
// println!(): The function name is "println", the "!" is called Rust macros (explained later)
// "\nHello, world!": The string that needs to be printed out
// "\n" is newline character
// The semi-colon ";": denotes that the expression println!() is over, and the next one is ready to begin
