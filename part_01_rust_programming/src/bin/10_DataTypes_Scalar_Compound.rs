fn main() {
    // ---------------------------------------------- //
    // ---------------- Scalar type ----------------- //
    // ---------------------------------------------- //
    // Scalar type basically means a single value
    // Rust's 4 primary scalar types: integers, floating-point numbers, Booleans, and characters

    /////////////
    // Integer //
    /////////////
    // Integer are number without fractional component

    // Signed integer: i8, i16, ..., i128, isize (architecture-dependent)
    // store value from -2^(n-1) to 2^(n-1)-1, meaning it can store negative integer numbers
    let x: i8 = 3;
    let y: i16 = -15;

    println!("x-i8 = {x}");
    println!("y-i16 = {y}");
}