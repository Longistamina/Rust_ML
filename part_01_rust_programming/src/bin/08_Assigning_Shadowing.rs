fn main() {
    // ------------------------------------------ //
    // ---------------- Assigning ----------------//
    // ------------------------------------------ //

    let mut x = 5; // mutable variable
    x += 1; // reassigning x to a new value
    println!("The value of x is: {x}"); // this will print 6

    println!("===============================================================================");

    // ------------------------------------------ //
    // ---------------- Shadowing ----------------//
    // ------------------------------------------ //
    // Shadowing means redeclare a variable of the same name but with a new value
    // Shadowing helps transform the variable, while still keeps it immutable, even change the type

    let y = 10; // immutable variable
    println!("The value of y is: {y}");

    let y = y + 14; // shadowing y with a new variable that has the same name
    println!("The value of y after shadowing is: {y}"); // this will print 11

    // const Z: u32 = 20; // constant variable
    // let Z = Z + 1; // shadowing Z with a new variable that has the same name
    // println!("The value of Z is: {Z}"); // this will raise error because constant cannot be shadowed

    println!("===============================================================================");

    // -------------------------------------------- //
    // ---------- Assigning vs Shadowing ---------- //
    // -------------------------------------------- //

    let space = "   "; // string
    let space = space.len(); // numeric
    println!("The lenght of the space character: {space}");

    /*
    let mut space = "   ";
    space = space.len()
    => Raise error!!!!!
     */
}
