use std::cmp::Ordering; // import Ordering type for enum variants: Less, Greater, Equal
use std::io;
// use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("This is the guessing game!");
    println!("Please guess a random number from 1 to 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read intput");

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); // convert "guess" from String to Integer u32 for comparison
                                                                           // The user input will be "number\n" (for example "5\n")
                                                                           // The "\n" here is due to the Enter
                                                                           // .trim() to remove that "\n", then .parse() to automatically parse into numeric type
                                                                           // since we set "guess: u32 = ...", the .parse() will try to parse the string into u32

    // call out .cmp() method to compare "guess" with "secret_number", returns one of 3 enums: Less, Greater, Equal
    match guess.cmp(&secret_number) { // "match" expression to decide what to do based on the variant of the Ordering
        Ordering::Less => println!("Too small!"), // guess < secret_number
        Ordering::Greater => println!("Too big!"), // guess > secret_number
        Ordering::Equal => println!("You win!!!!"), // guess == secret_number
    }
}