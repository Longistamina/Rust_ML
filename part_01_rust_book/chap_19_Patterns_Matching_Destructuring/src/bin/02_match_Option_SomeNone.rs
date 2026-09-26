/*
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

Literally says, if the value matches a pattern,
then executes the corresponding expression.

This file shows an example with Option<T>
*/

fn main() {
    println!();

    let nums: Vec<Option<i32>> = vec![Some(3), None, Some(89), None, Some(1_000_000), None, None];

    for num in nums {
        match num {
            Some(value) => println!("This is number {}", value),
            None => println!("This is None")
        }
    }
}

/*
This is number 3
This is None
This is number 89
This is None
This is number 1000000
This is None
This is None
*/
