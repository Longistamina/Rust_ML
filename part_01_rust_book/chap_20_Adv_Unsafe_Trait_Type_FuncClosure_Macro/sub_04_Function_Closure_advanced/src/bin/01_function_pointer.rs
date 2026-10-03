/*
`fn(...) -> ...` is a function pointer: it points to an ordinary named function.
You can pass a named function to another function.

`fn` is a type
*/

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32{ // `fn(i32) -> i32` means a pointer to a function that accepts an i32 and returns an i32
    f(arg) + f(arg)
}

fn main() {
    println!();

    let answer = do_twice(add_one, 3); // use `add_one` function as input for `do_twice` function
    println!("answer = {answer}")
}
