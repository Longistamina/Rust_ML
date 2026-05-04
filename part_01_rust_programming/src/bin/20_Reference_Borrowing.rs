/*
 * As mentioned earlier, when passing a mutable (like String Type) into a function,
 * it will be moved to the function's scope, and its memory is no longer valid in the main() scope.
 * -> Cannot reuse it anymore

* => Solution: Use references (&), or borrow it instead of moving it.
*/


#[allow(clippy::ptr_arg)]
fn _immutable_reference(s: &String) -> usize { // The & indicates "s" parameter here is a reference to a String
    s.len() // return the length of the string as an i32
}

#[allow(clippy::ptr_arg)]
fn _mutable_reference(s: &mut String) { // The &mut indicates "s" parameter here is a mutable reference to a String
    s.push_str(", world"); // convert the string to uppercase
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    // ----------------------------------------------------------------------------- //
    // ---------------------------- Immutable Reference ---------------------------- //
    // ----------------------------------------------------------------------------- //

    let s0 = String::from("hello!");
    let len = _immutable_reference(&s0);
    println!("The length of s0 is {len}");
    /*
    ============================= RUST Reference LAYOUT =============================

          STACK (The Reference)            STACK (The Owner)            HEAP (Data)
       ---------------------------      -----------------------      ---------------
       Name: s (&str)                   Name: s0 (String)            Index | Value
       +-------+-------+                +----------+-------+         +-----+-------+
       | ptr   |   *---|--------------->| ptr      |   *---|-------->|  0  |   h   |
       | len   |   5   |                | len      |   5   |         |  1  |   e   |
       +-------+-------+                | capacity |   5   |         |  2  |   l   |
                                        +----------+-------+         |  3  |   l   |
                                                                     |  4  |   o   |
                                                                     +-----+-------+
    Here, s0 is not moved into the function.
    Instead, a reference (ptr) to s0 is passed into the function.
    That reference helps the "s" parameter borrow the value of s0 without taking ownership.

    The return value is the length of s0, not changing anything
    -> not violate immutability.
    ================================================================================
     */

     // Rust allows multiple immutable references to the same data at the same time.
     // Because immutable variables have no pointer

     let s1 = &s0;
     let s2 = &s0;
     println!("Multiple mutable references: s1 = {s1}, s2 = {s2}");

     // NOTE: immutable references cannot be modified
     //
     // #[allow(clippy::ptr_arg)]
     // fn _change_immutable_reference(s: &String) {
     //     s.push_str(", world");
     // }
     // _change_immutable_reference(&s0);
     // println!("The change of s0 is {s0}");


     // --------------------------------------------------------------------------- //
     // ---------------------------- Mutable Reference ---------------------------- //
     // --------------------------------------------------------------------------- //

     let mut s0 = String::from("hello");
     _mutable_reference(&mut s0);
     println!("The mutable reference of s0 is {s0}");
     // Now, since s0 is a mutable reference, it can be modified by the function.


     // ------------------------------------------------------------------------------------ //
     // ---------------------------- Multiple Mutable Reference ---------------------------- //
     // ------------------------------------------------------------------------------------ //
     /*
      * * Rust does not allow multiple mutable references to the same data at the same time.
      * This is to prevent data races and ensure memory safety.
      *
      * A data race is similar to a race condition and happens when these three behaviors occur:
      *    + Two or more pointers access the same data at the same time.
      *    + At least one of the pointers is being used to write to the data.
      *    + There’s no mechanism being used to synchronize access to the data.
      *
      * => Solution: borrowing once at a time, combine with scope {}
      */

     let mut s0 = String::from("Rust is cool");

     // let s1 = &mut s0;
     // let mut s2 = &mut s0;
     // println!("Multiple mutable references: s1 = {s1}, s2 = {s2}");
     // -> causes compiler panic

     {
         let s1 = &mut s0;
         println!("First borrow: s1 = {s1}");
     } // here, s1 goes out of scope and is dropped, no more race conditions

     let s2 = &mut s0;
     println!("Second borrow: s2 = {s2}");


     // ------------------------------------------------------------------------------------ //
     // -------------------- Mutable and Immutable references - Lifetime ------------------- //
     // ------------------------------------------------------------------------------------ //
     /*
      * Rust does not allow mutable and immutable references to the same value at the same time,
      * must end the life-time of the immutable reference before the mutable reference can be used.
      */

     let mut s0 = String::from("Hello, World!");

     let s1 = &s0;

     // let s2 = &mut s0; // This will not work, because s0 is already borrowed as immutable

     println!("First borrow: s1 = {s1}");

     let s2 = &mut s0; // This instead will work, because s1 is already used and ended its life-time

     s2.insert_str(0, "asf");
     println!("After push_str: s2 = {s2}");


     // ------------------------------------------------------------------------------------ //
     // -------------------------------- Dangling Reference -------------------------------- //
     // ------------------------------------------------------------------------------------ //
     /*
      * A dangling reference is a reference that points to memory that has been freed or is no longer valid.
      * Meaning the memory on the heap is gone, but the pointer on the stack is still there, pointing to it.
      *
      * Rust never allows this dangling reference.
      * => The compiler will catch any attempt to use a dangling reference and prevent it from compiling.
      */

     //  fn dangle() -> &String {
     //      let s = String::from("dangle"); // Create a new String on the heap
     //      &s // reference to the string
     //  } // but "s" goes out of scope here, so it's memory is freed
     //    // so the "&s" reference is referencing to nothing
     //    // => compiler panic

     // let dangle = dangle();
     // println!("Dangling reference: dangle = {dangle}");

}
