/*
We have `if let` syntax to shorten some pattern matching boilerplates.

But what if we have multiple values and need to loop through them
while also matching them?

=> use `while let`
*/

// ==============================================================
// 1. without using `while let`: the clunky `match` block
// ==============================================================

fn demo_clunky_while_match() {
    let mut stack = vec![1, 2, 3];
    loop {
        match stack.pop() {
            Some(top) => {println!("Popped value: {}", top);},
            None => {break;} // You must explicitly handle the failure to break the loop
        }
    }
}

// ==============================================================
// 2. Using `while let`
// ==============================================================

fn demo_while_let() {
    let mut stack = vec![4, 5, 6];
    while let Some(top) = stack.pop() {
        println!("Popped value: {}", top);
    }
    // when `stack.pop()` returns None, the while automatically breaks
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_clunky_while_match();

    println!("\n===================================================\n");

    demo_while_let();
}
