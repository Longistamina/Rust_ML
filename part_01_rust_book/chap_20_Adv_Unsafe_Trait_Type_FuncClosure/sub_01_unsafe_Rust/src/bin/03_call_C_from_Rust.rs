/*
Rust cannot inspect or verify C code, so foreign functions are unsafe by default.
*/

unsafe extern "C" { //
    fn abs(input: i32) -> i32;
}
/*
`extern "C"` means “use the C calling convention,”
which specifies how arguments, return values, and calls work at the binary level.
*/

fn main() {
    unsafe {
        println!("{}", abs(-3));
    }
}

/*
If a particular imported function is truly safe for every allowed Rust input,
you can make that promise explicitly:
```
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}
```
*/
