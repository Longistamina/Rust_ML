use std::io;
// io stands for Input/Output, a library to obtain user input and then print the results as output
// std stands for Standard, is the standard library (where io library and many others come from)

// By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. 
// This set is called the "prelude"
// If something you want is not in the "prelude" (like std::io), you have to bring it in

fn main() {
    println!("Print the information!");

    println!("Please input your information.");

    let mut information = String::new(); // use "let mut" to declare a MUTABLE variable to store user input
                                           // String::new() is a function that returns a new instance of a String
                                           // the "::" indicates that "new()" is an association function of String
                                           // so, this String::new() returns an empty string which the variable "information" is bound to

    io::stdin() // or can use std::io::stdin() if we had not imported std::io before
        .read_line(&mut information) 
        .expect("Failed to read line");

    // .read_line(&mut information) call out read_line() from stdin() to take the user input, stored into "information"
    // that's why "information" must be mutable so that the method .read_line() can change its content to take in user input
    
    // the "&" indicates this argument (the user input) is a reference, letting other parts of the code access this piece of data without copying this data into memory multiple times
    // reference is also immutable by default, that's why we write "&mut information", not "&information"

    // .read_line() returns not only the user input, but also a Result value (an enumeration)
    // "Result" has 2 states: "Ok(value)" (successful operation) or "Err(error)" (failed operation)
    // "Result" has the .expect("message") method to handle "Err" cases -> it will cause the program to crash and display the "message"
    // If the state is "Ok", then "Result" will return the user input it is holding so that you can use it
    // (If we not call .expet("message") out, Rust will display a warning during compiling)
    // NOTE: more about Ok(value) and Err(error) in file 06_loop_break_continue_pass_ErrorHandling.rs
    
    // one-liner: io::stdin().read_line(&mut information).expect("Failed to read line");

    println!("Your given information: {information}"); // The {} is a placeholder, to print out the value of the "information" variable
}

/*
╰─ $ cargo run -p part_01_rust_programming --bin 02_io_InputOutput
   Compiling part_01_rust_programming v0.1.0 (/home/longdpt/Documents/Long_AISDL/Rust_ML/part_01_rust_programming)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/02_io_InputOutput`
Print the information!
Please input your information.
Rust is so cool!!!
Your given information: Rust is so cool!!!
*/