#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rand = "0.8.5"
//! ```

use rand::Rng; // must use rand = "0.8.5" in Cargo.toml for this to work
               // Rng here is a trait, it defines methods that random number generators implement, and this trait must be in scope for us to use those methods

fn main() {
    println!("Let's generate a random number!");
    
    let random_number = rand::thread_rng().gen_range(1..=100); // must use rand = "0.8.5" in Cargo.toml for this to work
                                   // rand::thread_rng() function gives a particular random number generator, on local thread, seeded by operating system
                                   // gen_range() is a method bound to the generator created by thread_rng()
                                   // 1..=100 means from 1 to 100 (the 100 is included), start..=end

    println!("The generated random number is: {random_number}")
}