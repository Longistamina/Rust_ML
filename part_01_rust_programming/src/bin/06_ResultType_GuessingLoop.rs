use std::io::{self, Write};
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    println!("This the Guessing Game!");

    println!("===============================================================================");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    let mut guess_string = String::new();
    let mut guess_numeric: u32;
    let mut guess_old: u32 = Default::default();

    // --- LOOP 1: Initial Valid Input ---
    loop { // This loop requires the player to re-input the guess in the right format (a number)
        print!("Please enter your guess (should be a number): ");
        io::stdout().flush().expect("Failed to flush stdout");

        guess_string.clear(); // clear old string input
        io::stdin()
            .read_line(&mut guess_string)
            .expect("Error: Failed to read input!");

        guess_numeric = match guess_string.trim().parse() { // handling invlaid input
            Ok(num) => num,
            Err(_) => continue // continue the loop
        // .parse() also returns a Result type, returning one of the two enums: "Ok(value)" and "Err(error)"
        // Therefore, we can use "match" expression to handle these enums, instead of calling ".expect()" to crash the program
        // OK(value) and Err(error) are like two sealed boxes containing different information.
        // If the parse() worked sucessfully, we got the box Ok(value),
        //                                    and we use "Ok(num) => num" is like to assign name "num" to that value, then return it (guess_numeric will take it)
        // If the parse() failed, we got the box Err(error),
        //                        we can assign a temporary name for the "error" and return it if we need it for other thing,
        //                        but here we use "_" is like a placeholder just for the sake of the syntax, we don't need that "error" information

       };

       break;
    }

    // --- LOOP 2: Game Logic and Re-guessing ---
    loop { // Create an infinite loop for the player to guess again

        match guess_old.cmp(&guess_numeric) {
            Ordering::Equal => (), // Do nothing, like "pass" in Python
            _ => match guess_numeric.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => { // If the player's guess is correct, announce the victory and break the loop
                    println!("You win!!!");
                    break;
                }
            }
        }

        guess_old = guess_numeric;

        print!("Please guess again ('q' to exit): ");
        io::stdout().flush().expect("Failed to flush stdout");

        guess_string.clear(); // Clear the old string input

        io::stdin()
            .read_line(&mut guess_string)
            .expect("Error: Failed to read input!");

        let guess_string = guess_string.trim(); // remove the "\n" character

        match guess_string.cmp("q") {
            Ordering::Equal => break,
            _ => { // "_" stands for all other cases
                guess_numeric = match guess_string.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!(">>>Please enter a number or 'q'!");
                        continue;
                    }
                }
            }
        }
    }

}

// loop {}: create infinite loop
// break: break the loop
// continue: continue the loop
// "()" or "{}": can use this as "do-nothing function", work like "pass" in Python
