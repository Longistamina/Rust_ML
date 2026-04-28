// ------------------------------------------------------------------------------------------------------------ //
// ----------------- Mutable and Immutable - Stack and Heap - String literals and String Type ----------------- //
// ------------------------------------------------------------------------------------------------------------ //
/*
Most of the types we have learned like int, float, bool, ... are of a known size
-> can be stored on the stack and popped off the stack when their scope is over
-> can be quickly and trivially copied to make a new, independent instance 
   if another part of code needs to use the same value in a different scope (referencing)

Even string literals like `let s = "Hello";` (&str) also have fixed size, immutable

String Type is one of types that are stored on the heap
(because their size is not fixed, grownable and mutable)
-> Use them to see how Rust knows when to clean up data, 
   without garbage collector like other languages (see Memory Allocation)

For example, when we want to take user string inputs, apparently their size are not known
-> cannot use string litterals, because they are fixed in size, immutable
-> MUST use String Type, because they are grownable in size, and mutable in value.
*/

#[allow(non_snake_case)] // Disable the non_snake_case warning
fn demo_StringType() {
  let s0 = "To day is so cool!"; // This s0 is a string literal (&str).
  println!("The value of s0 is: {s0}"); // s0 is stored on the stack, immutable, fixed size


  let s1 = String::from("hello"); // create a String object "from" literal string "hello"
  println!("The value of s1 is: {s1}");   // this s1 String will be stored on the heap

  let mut s2 = String::from("Good morning"); // a mutable String
  s2.push_str(", how are you?"); // push_str() appends a literal to a String
  println!("The value of s2 is: {s2}")
}

// Why can String be mutated but literals cannot? 
// The difference is in how these two types deal with memory (see OwnerShip and Memory Allocation)

////////////////////
///    main()    ///
////////////////////

fn main() {
    demo_StringType();
}