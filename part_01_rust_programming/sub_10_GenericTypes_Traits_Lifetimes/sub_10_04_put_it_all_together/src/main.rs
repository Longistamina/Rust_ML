/*
 * This file puts all the concepts of Generics, Traits and Lifetimes together into an example
 */

// 1. Import the Display trait from the standard library
use std::fmt::Display;

// 2. The function (from the book)
fn longest_with_an_announcement<'a, T>( // 'a is generic lifetime parameter, T is generic type parameter
    x: &'a str, // x is a reference parameter with lifetime 'a
    y: &'a str, // y is also a reference parameter with lifetime 'a
    ann: T,
) -> &'a str
where
    T: Display, // use ``where`` clause to specify that type T is bound to trait Display
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 3. A main function to test it
fn main() {
    let string1 = String::from("long string is long");
    let string2 = "xyz";

    // T can be ANY type that implements Display.
    // Let's use an i32 (a number) just to prove it doesn't have to be a string!
    let announcement = 404;

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        announcement
    );

    println!("The longest string is: {result}");

    /*
     * Both string1 and string2 live until the very end of the main function.
     * Therefore, their overlapping lifetime ('a) also lasts until the end of main.
     * => This means it is 100% safe for result to hold a reference to string1 and print it on the final line.
     */
}
