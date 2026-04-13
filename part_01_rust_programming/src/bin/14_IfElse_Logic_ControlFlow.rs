use rand::prelude::*;

fn main() {

    // ----------------------------------------- //
    // ------------------ if ------------------- //
    // ----------------------------------------- //

    let number = 2;

    if number != 0 {}


    // ----------------------------------------- //
    // --------------- if - else --------------- //
    // ----------------------------------------- //

    let mut rng = rand::rng();
    let random_number = rng.random_range(1..=10);
    
    if random_number > 5 {
        println!("TRUE: random_number {random_number} is greater than 5.")
    } else {
        println!("FALSE: random_number {random_number} is not greater than 5")
    }


    // --------------------------------------------------- //
    // --------------- if - else if - else --------------- //
    // --------------------------------------------------- //

    let mut rng = rand::rng();
    let random_score = rng.random_range(0.0..=10.0); // generate floating-point numbers

    if (9.0 < random_score) & (random_score <= 10.0) {
        println!("The score is EXCELLENT!")
    } else if random_score.gt(&8.0) & random_score.le(&9.0) {
        println!("The score is VERY GOOD!")
    } else if (6.5 < random_score) & (random_score <= 8.0) {
        println!("The score is GOOD!")
    } else if random_score.gt(&5.0) & random_score.le(&6.5) {
        println!("The score is AVERAGE!")
    } else {
        println!("The score is BAD!")
    }
}