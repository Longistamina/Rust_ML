// ========================================================
// Return a closure with `impl Fn(...) -> ...`
// ========================================================
/*
Closures are represented by traits, which means you can’t return closures directly.
In most cases where you might want to return a trait,
you can instead use the concrete type that implements the trait as the return value of the function.
*/

fn plus_one() -> impl Fn(i32) -> i32 { // return "something" that implements trait `Fn` and return i32 (that is closure)
    |x| x + 1
}

// ========================================================
// Return a closure that captures value with move
// ========================================================

fn plus_value(value: i32) -> impl Fn(i32) -> i32 {
    move |x| x + value
}
/*
We have to use keyword `move` here to makes closure `|x| x + value`
take the owner ship of the `value`, so that the closure can
keep using it after the `plus_value` finishes
*/

// ======================================================================
// Store different closures in a vector with `Box<dyn Fn(...) -> ...>`
// ======================================================================
/*
Suppose you want to store different closures in a vector like this:
```
let handlers = vec![plus_one(), plus_value(123)];
```

Though both of them promise to return the same thing `Fn(i32) -> i32`,
but each function's `impl Fn` hides a different concrete type,
thus they cannot be stored in the same vector.

Solution: hide them behind a smart pointer like `Box`
          `Box<dyn Fn(...) -> ...>`

The box holds a closure, and `dyn Fn` says callers can use it
through the `Fn(...) -> ...` interface without needing to know its exact closure type.
*/

fn multiply_two() -> Box<dyn Fn(f32) -> f32> {
    Box::new(|x| x * 2.0)
}

fn multiply_value(value: f32) -> Box<dyn Fn(f32) -> f32> {
    Box::new(move |x| x * value)
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    let add_one = plus_one(); // add_one = |x| x + 1
    let answer = add_one(4); // or can write `plus_one()(4)`
    println!("plus_one = {}", answer);

    let add_seven = plus_value(7);
    let answer = add_seven(5); // or can write `plus_value(7)(5)`
    println!("plus_value = {answer}");

    println!();

    let handlers = vec![multiply_two(), multiply_value(2.6), multiply_value(-8.2)];
    for handler in handlers {
        let answer = handler(3.24);
        println!("handler answer = {}", answer)
    }
}
/*
plus_one = 5
plus_value = 12

handler answer = 6.48
handler answer = 8.424
handler answer = -26.567999
*/
