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


    // -------------------------------------------------------------------------- //
    // ------------------------- Error with reference --------------------------- //
    // -------------------------------------------------------------------------- //

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {word}");
}
