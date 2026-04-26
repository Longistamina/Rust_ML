/*
In the case of a string literal (&str), we know the contents at compile time, 
so the text is hardcoded directly into the final executable.
This is why string literals are fast and efficient.
But these properties only come from the string literal’s immutability.

However, for String Type, we do not know the size of the contents in advance,
we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile,
and whose size might change while running the program.

So with the String Type, to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.

This means:
+ First, at runtime, an allocator will have to request an amount of memory to store the String with unknown size.
+ Second, When we are done with this String, we have to find a way to return back that memory (to avoid memory leak)

About the first part, when we do something like ```let s1 = String::from("hello")```,
the programming language will request the memory it needs (to store the mutable variable on the heap)

About the second part:
+ In languages with using garbage collector (GC), the GC will keep track the variables' memory,
  and it will help us clean up the memory part that isn't used anymore (so we don't need to think about it).
+ However, in languages without GC, we have to do it manually. 
  So it's our responsiblity to identify when the memory is not used anymore and free it.
  If we forget, memory will be wasted.
  If we do it too early, some variables will be invalid (because their memories have been freed up)

########################################################################################################################

Rust take a different path, that is Ownership and scope.

Rules of ownership:
+ Each value in Rust has an owner.
+ There can only be one owner at a time.
+ When the owner goes out of scope, the value will be dropped.

So when a variable is created, it will "owns" a memory space to store its value.
But when the variable goes out of the scope, that memory is automatically returned back, 
to avoid memory waste (or memory leak)
*/

// ---------------------------------------------------------------------- //
// ----------------- Variable Scope - Memory Allocation ----------------- //
// ---------------------------------------------------------------------- //
/*
Remind again, A scope is the range within a program for which an item is valid
*/

fn _demo_scope() {
    let s = "hello";
    
    {
        let s = "aloha"; // An amount of memory will be allocated to store this s="aloha" variable (on the stack since it's immutable)
        println!("Value of s is: {s}"); // This variable is only valid in this scope
    }
    // Outside the scope, variable s="aloha" is not valid anymore
    // So, the memory it owns is freed up! No memory waste! No memory leak!
    // Here, only s="hello" is valid

    println!("Value of s is: {s}")
}

//

fn main() {
    _demo_scope();
}