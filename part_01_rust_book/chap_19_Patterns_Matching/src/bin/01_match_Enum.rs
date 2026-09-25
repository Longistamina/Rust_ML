/*
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

Literally says, if the value matches a pattern,
then executes the corresponding expression.

This file shows an example with Enum
*/

enum Num {
    Int(i32),
    Float(f64),
    None,
}

fn main() {
    println!();

    let nums: Vec<Num> = vec![
        Num::Int(3),
        Num::None,
        Num::Float(28.0),
        Num::Float(15.3),
        Num::None,
    ];

    for num in nums {
        match num {
            Num::Int(value) => println!("This is integer {}", value),
            Num::Float(value) => println!("This is float {}", value),
            Num::None => println!("This is None")
        }
    }
}
/*
This is integer 3
This is None
This is float 28
This is float 15.3
This is None
*/
