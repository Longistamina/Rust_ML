#![allow(non_snake_case, dead_code)]

/*
This script shows all known matching syntaxes.

1. Matching Literals
2. Matching Named Variables
3. Matching Multiple Patterns
4. Matching Ranges with `..=` or `..`
5. Destructuring: Structs, Structs and Tuples, Enums, Nested Structs and Nested Enums
6. Ignoring Values with `_`: entire value, parts of value with nested `_`
7. Ignoring variable with `_variable_name`
8. Ignore remaining parts with `..`
9. Match Guards for Conditioning
10. Using `@` Bindings
*/

// ====================================================
// 1. Matching Literals
// ====================================================

fn demo_matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
// one

// ====================================================
// 2. Matching Named Variables
// ====================================================

fn demo_matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // this `y` will shadow the outside `y = 10`, it takes the value 5 of `x = Some(5)`
        _ => println!("Default case, x = {x:?}"),
    } // `y = 5` dropped here

    println!("at the end: x = {x:?}, y = {y}"); // back to `y = 10`
}
/*
Matched, y = 5
at the end: x = Some(5), y = 10

----------------------------------

Inside the match scope {},
x = Some(5) matches Some(y)
=> y takes the value 5

When goes out of scope,
`y = 5` is dropped, get back to `y = 10`
*/

// ====================================================
// 3. Matching Multiple Patterns
// ====================================================

fn demo_matching_multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
// one or two

// ====================================================
// 4. Matching Ranges with `..=` or `..`
// ====================================================

fn demo_matching_ranges() {
    let x = 5;
    match x {
        1..=5 => println!("x is in [1, 5]"), // 5 is inclusive with `1..=5`
        _ => println!("x is something else"),
    }

    let y = 10;
    match y {
        7..10 => println!("y is in [7, 10)"), // 10 is exclusive with `7..10`
        _ => println!("y is something else"),
    }

    let z = 'c';
    match z {
        'a'..='j' => println!("z is early ASCII letter"),
        'k'..='z' => println!("z is late ASCII letter"),
        _ => println!("z is something else"),
    }
}
/*
x is in [1, 5]
y is something else
z is early ASCII letter
*/

// ===========================================================================================
// 5. Destructuring: Structs, Enums, Nested Structs and Nested Enums, Structs and Tuples
// ===========================================================================================

// -------------------- //
// Destructring Structs //
// -------------------- //

struct Point {
    d1: i32,
    d2: i32
}

fn demo_destructuring_Struct() {
    let point = Point{d1: 0, d2: 7};

    let Point { d1: x, d2: y } = point; // destructuring, x will take 0, y will take 7
    println!("Coordinates (x, y) = ({}, {})", x, y);

    match point {
        Point { d1: x, d2: 0 } => println!("On the x axis at {x}"),
        Point { d1: 0, d2: y } => println!("On the y axis at {y}"),
        Point { d1: x, d2: y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
/*
Coordinates (x, y) = (0, 7)
On the y axis at 7
*/

// --------------------------- //
// Destructring Structs-Tuples //
// --------------------------- //

fn demo_destructuring_structs_tuples() {
    let ((feet, inches), Point { d1: x, d2: y }) = ((3, 10), Point { d1: 3, d2: -10 });

    println!("feet = {feet}");
    println!("inches = {inches}");
    println!("x = {x}");
    println!("y = {y}");
}
/*
feet = 3
inches = 10
x = 3
y = -10
*/

// ------------------ //
// Destructring Enums //
// ------------------ //

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn demo_destructuring_Enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }
}
// Change color to red 0, green 160, and blue 255

// --------------------------------- //
// Destructring Nested Structs-Enums //
// --------------------------------- //

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn demo_destructuring_nested_structs_enums() {
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}
// Change color to hue 0, saturation 160, value 255

// ============================================================================
// 6. Ignoring Values with `_`: entire value, parts of value with nested `_`
// ============================================================================

// ------------------- //
// Ignore entire value //
// ------------------- //

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn demo_ingore_entire_value() {
    foo(3, 4) // only print 4
}
// This code only uses the y parameter: 4

// ------------------------------------- //
// Ignore parts of value with nested `_` //
// ------------------------------------- //

fn demo_ignore_parts_of_value() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
}
// Some numbers: 2, 8, 32

// ====================================================
// 7. Ignoring variable with `_variable_name`
// ====================================================

fn demo_ignore_variable() {
    let _x = 3; // will not raise warning
    let y = 5;
    println!("y = {y} (x is ignored)");
}
// y = 5 (x is ignored)

/* WARNING
let s = Some(String::from("Hello!"));

if let Some(_) = s { // `if let some(_s) = s` will cause error because s is moved to _s. Should use only `_` here
    println!("found a string");
}

println!("{s:?}");
*/

// ====================================================
// 8. Ignore remaining parts with `..`
// ====================================================

struct Shape {
    x: i32,
    y: i32,
    z: i32
}

fn demo_ignore_remaining() {
    let origin = Shape { x: 0, y: 0, z: 0 };
    match origin {
        Shape { x, .. } => println!("x is {x}"), // ignore the `y` and `z`
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => { // ignoring the middle parts
            println!("First and Last: {first}, {last}");
        }
    }
}
/*
x is 0
First and Last: 2, 32
*/

// ====================================================
// 9. Match Guards for Conditioning
// ====================================================

fn demo_match_guards_conditioning() {
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"), // num = Some(4) matches Some(x), then checks if (x % 2 == 0)
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    println!();

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"), // `y` here is the outside `y = 10` since there's nothing shadowing it.
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    println!();

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
/*
The number 4 is even

Default case, x = Some(5)
at the end: x = Some(5), y = 10

no
*/

// ====================================================
// 10. Using `@` Bindings
// ====================================================
/*
The at operator `@` lets us create a variable that holds a value
at the same time we’re testing that value for a pattern match.
*/

enum Notice {
    Hello { id: i32 },
}

fn demo_at_operator_bindings() {
    let msg = Notice::Hello { id: 5 };

    match msg {
        Notice::Hello { id: captured @ 3..=7 } => {
            println!("Found an id in range: {captured}")
            // By specifying `captured @` before the range `3..=7`
            // we’re capturing whatever value matched the range in a variable named `captured`
            // while also testing that the value matched the range pattern.
        }
        Notice::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Notice::Hello { id } => println!("Found some other id: {id}"),
    }
}
// Found an id in range: 5

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_matching_literals();

    println!("\n====================================================\n");

    demo_matching_named_variables();

    println!("\n====================================================\n");

    demo_matching_multiple_patterns();

    println!("\n====================================================\n");

    demo_matching_ranges();

    println!("\n====================================================\n");

    demo_destructuring_Struct();

    println!("----------------------");

    demo_destructuring_structs_tuples();

    println!("----------------------");

    demo_destructuring_Enum();

    println!("----------------------");

    demo_destructuring_nested_structs_enums();

    println!("\n====================================================\n");

    demo_ingore_entire_value();

    println!("----------------------");

    demo_ignore_parts_of_value();

    println!("\n====================================================\n");

    demo_ignore_remaining();

    println!("\n====================================================\n");

    demo_match_guards_conditioning();

    println!("\n====================================================\n");

    demo_at_operator_bindings();
}
