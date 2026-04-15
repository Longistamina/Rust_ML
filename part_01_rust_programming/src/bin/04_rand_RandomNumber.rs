#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rand = "0.8.5"
//! ```

use rand::prelude::*; // must use rand = "0.10.1" in Cargo.toml for this to work
                      // "prelude" is a common module of many Rust library, storing frequently used modules
                      // ::* means import everything from prelude
fn main() {
    println!("Let's generate a random number!");

    let mut rng = rand::rng(); // Create a random generator (must be "mut")
    
    let random_int = rng.random_range(1..=100);
    println!("The generated random INTEGER number is: {random_int}");
    // 1. rand::rng() provides a high-quality random number generator
    //    that is local to the current thread and automatically seeded by the OS.
    // 2. random_range() is a method from the RngExt trait 
    //    used to generate a value within a specific bound.
    // 3. 1..=100 is an inclusive range syntax, meaning it covers all numbers 
    //    from 1 up to and including 100.

    let random_float = rng.random_range(1.0..=100.0); // generate floating-point numbers
    println!("The generated random FLOATING number is: {random_float}")

}