fn main() {
    // ------------------------------------------ //
    // ---------------- Assigning ----------------//
    // ------------------------------------------ //

    let mut x = 5; // mutable variable
    x = x + 1; // reassigning x to a new value
    println!("The value of x is: {x}"); // this will print 6

    // ------------------------------------------ //
    // ---------------- Shadowing ----------------//
    // ------------------------------------------ //

    let y = 10; // immutable variable
    let y = y + 1; // shadowing y with a new variable that has the same name
    println!("The value of y is: {y}"); // this will print 11

    // const Z: u32 = 20; // constant variable
    // let Z = Z + 1; // shadowing Z with a new variable that has the same name
    // println!("The value of Z is: {Z}"); // this will raise error because constant cannot be shadowed
}