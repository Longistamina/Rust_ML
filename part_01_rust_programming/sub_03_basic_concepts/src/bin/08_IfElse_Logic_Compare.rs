// In Rust (and many other languages), if-else are used for Flow Control (branching the program).

use rand::prelude::*;

fn main() {
    let mut rng = rand::rng(); // 1. Make the variable mutable

    demo_if();
    println!("===============================================================================");

    demo_if_else(&mut rng); // 2. Pass a mutable reference (borrowing)
    println!("===============================================================================");

    demo_if_elseif_else(&mut rng); // 3. Use it again
    println!("===============================================================================");

    demo_if_nested(&mut rng);
    println!("===============================================================================");

    demo_let_if(&mut rng);
}

// ----------------------------------------- //
// ------------------ if ------------------- //
// ----------------------------------------- //

fn demo_if() {

    let number = 2;

    if number != 0 {
        println!("This number is something other than 0.")
    }
}

// ----------------------------------------- //
// --------------- if - else --------------- //
// ----------------------------------------- //

fn demo_if_else(rng: &mut ThreadRng) {

    let random_number = rng.random_range(1..=10);

    if random_number > 5 {
        println!("TRUE: random_number {random_number} is greater than 5.")
    } else {
        println!("FALSE: random_number {random_number} is not greater than 5")
    }
}

// --------------------------------------------------- //
// --------------- if - else if - else --------------- //
// --------------------------------------------------- //

fn demo_if_elseif_else(rng: &mut ThreadRng) {

    let random_score = rng.random_range(0.0..=10.0); // generate floating-point numbers

    if (9.0 < random_score) & (random_score <= 10.0) {
        println!("The score is EXCELLENT!")
    } else if random_score.gt(&8.0) & random_score.le(&9.0) { // (8.0 < random_score) & (random_score <= 9)
        println!("The score is VERY GOOD!")
    } else if (6.5 < random_score) & (random_score <= 8.0) {
        println!("The score is GOOD!")
    } else if random_score.gt(&5.0) & random_score.le(&6.5) { // (5.0 < random_score) & (random_score <= 6.5)
        println!("The score is AVERAGE!")
    } else {
        println!("The score is BAD!")
    }

    // NOTE: Using many if-else can clutter your code
    //       In such case, consider using "match" expression
}

// ----------------------------------------- //
// -------------- Nested if ---------------- //
// ----------------------------------------- //

fn demo_if_nested(rng: &mut ThreadRng) {
    let has_id = rng.random_bool(0.5); // 50% chance of being true
    let is_admin = rng.random_bool(0.3); // 30% chance of being true

    print!("Access Check: ");

    if has_id {
        // This code only runs if has_id is true
        if is_admin {
            println!("Welcome, Administrator. You have full access.");
        } else {
            println!("Welcome, User. You have limited access.");
        }
    } else {
        println!("Access Denied. No ID found.");
    }
}

// -------------------------------------------------------------------- //
// --------------- let var = if cond {expr} else {expr} --------------- //
// -------------------------------------------------------------------- //

fn demo_let_if(rng: &mut ThreadRng) {

    let random_number = rng.random_range(1..=100);

    let condition = (random_number % 2).eq(&0); // true or false

    let result = if condition {"EVEN"} else {"ODD"};
    // if condition == true, then result = "EVEN",
    // else, result = "ODD"

    println!("{random_number} is {result}!")
}

/*
######### COMPARING OPERATORS #########

x != y || x.ne(&y) || not equal
x == y || x.eq(&y) || equal
x >= y || x.ge(&y) || greater or equal
x <= y || x.le(&y) || less or equal
x > y  || x.gt(&y) || greater than
x < y  || x.lt(&y) || less than

Again, here we write "&y", not "y".
This is because we want to use y as reference only (or "borrow" it).

######### LOGICAL OPERATORS #########

expr & expr -> Bitwise AND
var &= expr -> Bitwise AND and assignment
expr && expr -> Short-circuiting logical AND (If the first part is false, stop the checking)

expr | expr -> 	Bitwise OR
var |= expr -> Bitwise OR and assignment
expr || expr -> Short-circuiting logical OR (If the first part is false, stop the checking)

expr ^ expr -> Bitwise exclusive OR
var ^= expr -> Bitwise exclusive OR and assignment

!expr -> NOT logic, negate the truth value of the expression
*/
