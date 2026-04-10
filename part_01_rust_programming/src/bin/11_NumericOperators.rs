fn main() {
    // addition
    let sum = 5 + 10;
    println!("Sum = 5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Subtraction = 95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("Multiplication = 4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("Division_quotient = 56.7 / 32.2 = {quotient}");

    let truncated = -5 / 3; // Results in -1
    println!("Division_truncated = -5 / 3 = {truncated}");

    let quotient = -5.0 / 3.0; // Not truncated anymore
    println!("Division_quotient = -5.0 / 3.0 = {quotient}");

    // remainder
    let remainder = 43 % 5;
    println!("Remainder = -5.0 / 3.0 = {remainder}");

    // exponent (use .pow(), .powi(), .powf(), .exp())
    let integer_base: i32 = 2;
    let result = integer_base.pow(3);
    println!("Exponent_integer = 2^3 = {result}");

    let float_base: f64 = 3.9;
    let result = float_base.powi(5);
    println!("Exponent_float = 3.9^5 = {result}");

    let float_base: f64 = 2.5;
    let result = float_base.powf(-2.0);
    println!("Exponent_float = 2.5^(-2.0) = {result}");    

    let result = 1.0_f64.exp(); // natural exponentioal
    println!("Exponent_natural = 1^e = {result}")
}