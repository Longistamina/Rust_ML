fn main() {
    // ---------------------------------------------- //
    // ---------------- Scalar type ----------------- //
    // ---------------------------------------------- //
    // Scalar type basically means a single value
    // Rust's 4 primary scalar types: integers, floating-point numbers, Booleans, and characters

    ////////////////////
    //    Integer    ///
    ////////////////////
    // Integer are number without fractional component

    // Signed integer: i8, i16, ..., i128, isize (architecture-dependent)
    // stores value from -2^(n-1) to 2^(n-1)-1, it can store both positive and negative integer numbers
    // For example, i8 stores value from -2^7 to 2^7-1 -> from -128 to 127

    let x: i8 = 3;
    // let x = 3_i8;
    println!("x-i8 = {x}");

    let y: i16 = -15;
    // let y = -15_i16;
    println!("y-i16 = {y}");

    // Unsigned integer: u8, u16, ..., u128, usize (architecture-dependent)
    // stores value from 0 to 2^n-1, it can store only positive numbers
    // For example, u8 stores value from 0 to 2^8-1 -> from 0 to 255

    let x: u8 = 15;
    // let x = 15_u8;
    println!("x-u8 = {x}");

    let y: u32 = 1_234; // equals to 1234
    // let y = 1_234_u32;
    println!("y-u32 = {y}");

    /*
    let z: u16 = -23
    => Raise ERROR!!!!
     */

    // NOTE: the default integer type is i32
    // NOTE: if you try to store the value exceed the range of the type -> Overflow

    println!("===============================================================================");

    //////////////////////////
    //    Floating-Point    //
    //////////////////////////
    // Floating-Point types are numbers with fractional component
    // Rust has two primitive types for FP: f32 and f64 (default is f64)
    // Both are signed type (can store both positive and negative number)
    // FP is represented according to IEEE-754 standard

    let x = 2.3; // f64
    // let x = 2.3_f64;
    println!("x-f64: {x}");

    let y: f32 = -1_530.26;
    // let y = -1_530.26_f32;
    println!("y-f32: {y}");

    println!("===============================================================================");

    ///////////////////
    //    Boolean    //
    ///////////////////
    // Boolean type in Rust has two possible values: true and false
    // Boolean type can be used for control flow (if condition), indexing, ....

    let t = true;
    println!("True: {t}");

    let f: bool = false; // with explicit type annotation
    println!("False: {f}");

    println!("===============================================================================");

    ////////////////
    //    Char    //
    ////////////////
    // The char type represents a SINGLE character
    // char literals are specified using single quotation marks ''
    // (mean while String is something like a vector containing many char, so it's not scalar)
    // (String literals are represented by double quotation marks "")

    let c = 'c'; // Must use '', not ""
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Char type examples: {c}, {z}, {heart_eyed_cat}");

    println!("===============================================================================");

    // ------------------------------------------------- //
    // ---------------- Compound Types ----------------- //
    // ------------------------------------------------- //
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    /////////////////
    //    Tuple    //
    /////////////////
    // Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
    // Use () and "," to define a tuple.
    // Each position in the tuple has a type, they don’t have to be the same.

    let tup: (i32, f64, u8) = (-500, 6.4, 1);
    println!("Tuple tup = {tup:?}");

    // Destructure a tuple to get its value
    let (x, y, z) = tup;
    println!("x_tup = {x}");
    println!("y_tup = {y}");
    println!("z_tup = {z}");

    // Use indexing to access its value
    let first_element_tup = tup.0;
    let second_element_tup = tup.1;
    println!("first_element_tup = {first_element_tup}");
    println!("second_element_tup = {second_element_tup}");

    println!("===============================================================================");

    /////////////////
    //    Array    //
    /////////////////
    // Unlike a tuple, every element of an ARRAY must have the SAME TYPE.
    // Array in rust has fixed length
    // Use [] and "," to define an array

    let a = [1, 2, 3, 4, 5];
    println!("Array a = {a:#?}"); // print each element of the array on a separate line

    let months = ["January", "February", "March", "April", "May", "June", "July",
                              "August", "September", "October", "November", "December"];
    println!("Array months = {months:?}"); //  print out all the elements of the array on one single line

    // define an array with duplicate values
    let a_duplicate = [5.2; 9];
    println!("Array a_duplicate = {a_duplicate:?}");

    // define an array with explicit type
    let a_explicit: [f32; 4] = [-1.5, -2.9, 0.2, 6.0];
    println!("Array a_explicit = {a_explicit:?}");

    // Array element accessing
    let first_element_array = a_explicit[0];
    let second_element_array = a_explicit[1];
    let last_element_array = a_explicit[a_explicit.len()-1];
    println!("first_element_array = {first_element_array}");
    println!("second_element_array = {second_element_array}");
    println!("last_element_array = {last_element_array}");

    // NOTE: tuple does not support indexing like array since its elements' datatypes are not the same
}
