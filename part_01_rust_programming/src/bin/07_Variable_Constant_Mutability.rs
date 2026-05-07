fn main() {
    // --------------------------------------------------- //
    // ---------------- Immutable variable ----------------//
    // --------------------------------------------------- //

    let x = 5; // immutable variable
    println!("The value of x is: {x}");

    // x = 6; // this will cause a compile-time error because x is immutable

    println!("===============================================================================");

    // ------------------------------------------------- //
    // ---------------- Mutable variable ----------------//
    // ------------------------------------------------- //

    let mut y = 10; // mutable variable
    println!("The value of y is: {y}");

    y = 15; // this is allowed because y is mutable
    println!("The value of y is now: {y}");

    println!("===============================================================================");

    // ------------------------------------------ //
    // ---------------- Constant ----------------//
    // ---------------------------------------- //

    const MAX_POINTS: u32 = 100_000; // constant variable
    const PI: f64 = 3.14159; // another constant variable
    println!("The maximum points are: {MAX_POINTS}");
    println!("The value of PI is: {PI}");

    // To declare a constant, use the `const` keyword, followed by the name of the constant
    // Constants are always immutable, so the expression assigned to a constant must be a constant expression, not a value that could be computed at runtime.
    // Cannot use the `mut` keyword with constants, as they are inherently immutable.
    // Constants are conventionally written in uppercase with underscores separating words.

    // When declaring a constant, you must also specify its type, as constants do not have a default type.
    // This is because the value of a constant must be known at compile time, and the compiler needs to know the type to determine how much memory to allocate for it.

    // Operations that can be used when declaring constants: https://doc.rust-lang.org/reference/const_eval.html

}
