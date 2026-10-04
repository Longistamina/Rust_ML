/*
A declarative macro matches the `shape of Rust` code
and substitutes code for the matching parts.

This is somewhat like `match`, except it matches tokens and syntax rather than runtime values.

To create a declarative macro, we use `macro_rules!`

This file shows the example of macro `vec![]`
(I rename it as `list![]`)
*/

#[macro_export] // makes a declarative macro (macro_rules!) available outside of the module or crate where it was defined
macro_rules! list {
    ( $($x:expr),* ) => { // replacement-code body
        { // generated Rust block expression
            let mut list_temp = Vec::new();
            $(
                list_temp.push($x);
            )*
            list_temp
        }
    }
}
/*
`$x:expr` -> capture a Rust expression, and call it $x
`$($x:expr)` -> group this pattern because it can repeat
`$($x:expr),` -> the comma "," means the expressions/patterns are separated by commas
`$($x:expr),*` -> the "*" means zero or more occurences, meaning that this patterns could be absent (zero) or occur many times
`( $( $x:expr ),* )` -> the outer "()" marks the boundary of the pattern

---------------------------------

`$( list_temp.push($x))*`
-> this means for captured expression `$x`,
   repeat the `push()` once
*/

fn main() {
    println!();

    let list = list![1, 3, 5, 7];
    println!("list = {:?}", list)
}
/*
Rust will replace `let list = list![1, 3, 5, 7]` with
```
let list = {
    let mut list_temp = Vec::new();
    list_temp.push(1);
    list_temp.push(3);
    list_temp.push(5);
    list_temp.push(7);
}
```

That's why in the macro definition, we have to open the {} two times
(...) => {
    {
        ...
    }
}
*/
