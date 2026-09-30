/*
Raw pointers: `*const T` and `*mut T`

A normal reference, such as `&i32` or `&mut i32`, carries strong compiler guarantees:
- it points to valid memory
- it is not null
- aliasing rules are obeyed (&mut is exclusive)
- it behaves within Rust’s lifetime system

A raw pointer gives up those guarantees.
In Rust, a raw pointer is a low-level primitive type that holds a direct memory address.
Represented as `*const T` (immutable) and `*mut T` (mutable),
raw pointers bypass Rust’s strict compile-time safety checks and borrow checker.
(allow a mutable reference to exist along side another mutable/immutable reference)

Why must use unsafe Rust to dereference raw pointers?
A raw pointer could be null, dangling, incorrectly aligned,
or pointing to the wrong kind of value. Rust cannot know.
*/

fn main() {
    println!();

    let mut num = 5;

    let r1 = &raw const num; // create a raw constant pointer
    let r2 = &raw mut num; // create a raw mutable pointer

    let address = 0x012345usize; // create merely a pointer-shaped value, not magically create valid memory at that address
    let r = address as *const i32;

    unsafe { // open `unsafe` block to allow dereferencing raw pointers
        println!("{}", *r1); // must use `*r1` because it is the true value, while `r1` is just an adress
        println!("{}", *r2);
        println!("{}", *r) // runtime error: misaligned pointer dereference: address must be a multiple of 0x4 but is 0x12345
    }
}

/*                        THIS WILL NOT WORK
fn main() {
    println!();

    let mut num = 5;

    let r1 = &num;
    let r2 = &mut num;

    println!("{}", r1);
    println!("{}", r2);
}
*/
