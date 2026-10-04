/*
`Fn(...) -> ...` is a trait implemented by closures and function pointers.
The uppercase `Fn` is a trait.

Other variants: `FnMut`, `FnOnce`
*/

fn add_two(x: i32) -> i32 {
    x + 2
}

fn do_triple<F: Fn(i32) -> i32>(f: F, arg: i32) -> i32 { // Not only accepts function pointers, but also closures
    f(arg) + f(arg) + f(arg)                             // Basically accepts anything that implements `Fn` trait
}

fn main() {
    println!();

    let answer_fn = do_triple(add_two, 3); // use function pointer as input for `do_triple`
    println!("answer_fn = {answer_fn}");

    let answer_closure = do_triple(|x| x * 4, 2); // use closure as input for `do_triple`
    println!("answer_closure = {answer_closure}")
}
