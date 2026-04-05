use std::io::{self, Write}; // import Write to enable stdout().flush()
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("This the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    print!("Please enter your guess: "); // print!() will not display the text, but store in a buffer, only waits for \n then then display
    io::stdout().flush().expect("Failed to flush stdout"); // manually flush the buffer, to force the text to be displayed

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Error: Failed to read input!");

    loop { // Create an infinite loop for the player to guess again

        let guess_numeric: u32 = guess.trim().parse().expect("Error: Please input a guessing number!");

        match guess_numeric.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { // If the player's guess is correct, announce the victory and break the loop
                println!("You win!!!");
                break;
            }
        }

        print!("Please guess again ('q' to exit): ");
        io::stdout().flush().expect("Failed to flush stdout");

        guess.clear(); // Clear the old string input

        io::stdin()
            .read_line(&mut guess)
            .expect("Error: Failed to read input!");

        let guess = guess.trim(); // remove the "\n" character

        match guess.cmp("q") {
            Ordering::Equal => break,
            _ => continue // "_" stands for all other cases
        }
    }
    
}