/*
 * Slices let you reference a contiguous sequence of elements in a collection (vector, strings, hash maps).
 * A slice is a kind of reference, so it does not have ownership.
 */

fn main() {
    // ---------------------------------------------------------------- //
    // ------------------------- Slice Type --------------------------- //
    // ---------------------------------------------------------------- //

    let s = String::from("hello_bonjour_nihao");

    let eng = &s[0..5]; // start_index..end_index-1, The "_" is not included in the slice
    let fr = &s[6..13];
    let ch = &s[14..];

    println!("English: {}", eng);
    println!("French: {}", fr);
    println!("Chinese: {}", ch);

    let i = 10;
    println!("First to {i}th-1: {}", &s[..10]); // equivalent to &s[0..10]
    println!("{i}th to end: {}", &s[i..]); // equivalent to &s[10..s.len()]

    println!("s[..] = {}", &s[..]); // equivalent to &s[0..s.len()], the whole string itselfs


    // ------------------------------------------------------------------------------------ //
    // ------------------------- &str over &String for function --------------------------- //
    // ------------------------------------------------------------------------------------ //
    /*
     * Should PREFER "fn first_word(s: &str) -> &str {}" over "fn first_word(s: &String) -> &str {}"
     * Using &str makes your function more flexible and idiomatic because it allows the function to accept both String and &str types.
     * If you define a function with &String, you can only pass a String object to it.
     * However, if you define it with &str, you can pass:
     * - A String (it will automatically turn into a slice)
     * - A string literal ("hello")
     * - A slice of a string (&s[0..5])
     */

     fn first_word(s: &str) -> &str { // Accepts both String and &str types
         for (i, &item) in s.as_bytes().iter().enumerate() {
             if item == b' ' {
                 return &s[..i];
             }
         }
         s

         // s.as_bytes() converts the &str to an array of bytes
         // .iter() is a method that returns each element in a collection
         // .enumerate()  wraps the result of iter and returns each element as part of a tuple (index, &item)
         // &item is a reference to the byte value
         // item == b' ' checks if the byte is a space character, then returns the value before the function ends
         // If no space is found, the entire string is returned
     }

     let my_string = String::from("Hello World");
     let my_literal = "hello world";

     // Both of these work with &str:
     println!("{}", first_word(&my_string)); // Coerced from &String to &str
     println!("{}", first_word(my_literal)); // Already a &str
     println!("{}", first_word(&my_string[..3])); // Only pass the first 6 elements of the string to the function
}
