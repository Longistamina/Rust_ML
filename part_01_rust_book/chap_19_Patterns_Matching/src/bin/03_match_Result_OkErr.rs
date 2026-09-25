/*
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

Literally says, if the value matches a pattern,
then executes the corresponding expression.

This file shows an example with Result<T, Error>
*/

fn main() {
    println!();

    let nums_str = vec!["32", "one", "46"];

    for num in nums_str {
        let result = num.parse::<i32>();

        match result {
            Ok(value) => println!("Parsed sucessfully: {}", value),
            Err(error) => println!("Parsed fail: {}", error)
        }
    }
}

/*
Parsed sucessfully: 32
Parsed fail: invalid digit found in string
Parsed sucessfully: 46
*/
