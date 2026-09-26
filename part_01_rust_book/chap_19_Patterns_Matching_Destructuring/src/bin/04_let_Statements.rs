/*
let PATTERN = EXPRESSION;

Every time you’ve used a let statement like this you’ve been using patterns,
although you might not have realized it!
*/

fn main() {
    println!();

    let x = 5;
    println!("x = {}", x);

    println!("\n==============================================================\n");

    let (a, b, c) = (22, 48, 960);
    println!("a = {a}");
    println!("b = {b}");
    println!("c = {c}");

    println!("\n==============================================================\n");

    let (m, _, k) = (1.5, 3.9, 4.2); // ignore the 2nd value
    println!("m = {m}");
    println!("k = {k}")
}

/*
x = 5

==============================================================

a = 22
b = 48
c = 960

==============================================================

m = 1.5
k = 4.2
*/
