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

/*
So to sum thing up, when we declare a variable, the language will request an amount of memory to store that variable,
or we can also say that the variable owns that amount of memory.

If the variable is immutable -> the memory is allocated on the stack
If the variable is mutable -> the memory is allocated on the heap

Then the variable goes out of the scope {}
-> Rust will automatically call a function named ```drop``` at the end of the scope {}
-> this ```drop``` function will eliminate the variable and fee up space
   (in Python, we have a similar function named "del", in C++ is Resource Acquisition Is Initialization (RAII))
*/


// ------------------------------------------------------------------------------------- //
// ----------------------- Data Interaction between Immutables ------------------------- //
// ------------------------------------------------------------------------------------- //
/* In Rust, multiple variables can interact with the same data in different ways */

fn _demo_immutable_interact() {
    let x = 5; // bind value 5 to x
    let y = x; // make a copy of the value in x, then bind it to y

    println!("x = {x}");
    println!("y = {y}")

    /*
        x <- 5   bind value 5 to x
             |
             v
        y <- 5   make a copy of the value in x, then bind it to y

    So here, we have two copies of value 5, each binds to x and y respectively.
    Since they are immutable, they are stored on the stack.
    And since their size is fixed, the program can make 2 (or more) copies of them without caring too much about memory waste.
    */
}


// ----------------------------------------------------------------------------------- //
// ----------------------- Data Interaction between Mutables ------------------------- //
// ----------------------------------------------------------------------------------- //
/* In Rust, multiple variables can interact with the same data in different ways */

/*
A String Type in Rust is made up of 3 parts:
+ a pointer (on the stack): shows the way to the memory that holds the contents of the string (on the heap)
+ a length (on the stack): how much memory, in bytes, the contents of the String are currently using
+ a capacity (on the stack): the total amount of memory, in bytes, that the String has received from the allocator
(We will deal with length and capacity later)

So when we create a string using ```s1 = String::from("hello")```,
its memory layout is being shown as below.

All the mutable types in Rust that need a pointer to point to the memory that holds actual data will have similar layouts like this.
The actual data in the heap is the thing that is mutable, growable, changeable...

    ========================= RUST Mutable MEMORY LAYOUT =========================

        STACK (Variable Metadata)                       HEAP (Actual Data)
    -------------------------------                 -----------------------
    Name: s1 (The Owner)                            Index | Value
    +----------+----------+                         +-----+-------+
    | ptr      |  *-------------------------------->|  0  |   h   |
    | len      |    5     |                         |  1  |   e   |
    | capacity |    5     |                         |  2  |   l   |
    +----------+----------+                         |  3  |   l   |
                                                    |  4  |   o   |
                                                    +-----+-------+
*/

//////////////////////////////////////////////////
//    borrowing - referencing - shallow copy    //
//////////////////////////////////////////////////

fn _demo_mutable_borrow() {
    let s1 = String::from("hello");
    let s2 = &s1; // s2 borrow the value of s1, the "&" sign is for referencing, borrowing
                           // By doing this, s1 is still remains
                           // Because we only copy the stack data (ptr, len, capacity) of s1, then bind to s2, not moving s1 to s2
                           // the actual on the heap is not copied -> 2 pointers of s1 and s2 point to the same heap
                           // -> s2 is a "shallow copy"

    println!("s1 = {s1}");
    println!("s2 = s1_borrowed = {s2}");

    /*
    ========================= RUST Borrowing, Referencing LAYOUT =========================
        STACK (Variable Metadata)                       HEAP (Actual Data)
    -------------------------------                 -----------------------
    Name: s1 (The Owner)                            Index | Value
    +----------+----------+                         +-----+-------+
    | ptr      |  *-------------------------------->|  0  |   h   |
    | len      |    5     |                         |  1  |   e   |
    | capacity |    5     |                         |  2  |   l   |
    +----------+----------+                         |  3  |   l   |
                                                    |  4  |   o   |
    Name: s2 (The Borrower)                         +-----+-------+
    +----------+----------+                            ^
    | ptr      |  *------------------------------------+
    | len      |    5     |
    | capacity |    5     |
    +----------+----------+
     */
}

////////////////////////////////////
//    clone - make a deep copy    //
////////////////////////////////////


fn _demo_mutable_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // The clone() will make a copy of all data on the stack and the heap of s1, and bind it to s2
    // s2 is a "deep copy"

    println!("s1 = {s1}");
    println!("s2 = s1_cloned = {s2}");

    /*
    ========================= RUST Mutable Cloning LAYOUT =========================
        STACK (Variable Metadata)                       HEAP (Actual Data)
    -------------------------------                 -----------------------
    Name: s1                                        Index | Value
    +----------+----------+                         +-----+-------+
    | ptr      |  *-------------------------------->|  0  |   h   |
    | len      |    5     |                         |  1  |   e   |
    | capacity |    5     |                         |  2  |   l   |
    +----------+----------+                         |  3  |   l   |
                                                    |  4  |   o   |
                                                    +-----+-------+

    Name: s2                                        Index | Value
    +----------+----------+                         +-----+-------+
    | ptr      |  *-------------------------------->|  0  |   h   |
    | len      |    5     |                         |  1  |   e   |
    | capacity |    5     |                         |  2  |   l   |
    +----------+----------+                         |  3  |   l   |
                                                    |  4  |   o   |
                                                    +-----+-------+
     */
}

////////////////////////////////////////
//    move one variable to another    //
////////////////////////////////////////

fn _demo_mutable_moved() {
    let s1 = String::from("hello");
    let s2 = s1; // here s1 has been moved to s2, its memory is no longer valid

    // println!("s1 = {s1}"); // this will raise error because s1 is no longer valid, it has been moved to s2
    println!("s2 = s1_moved = {s2}")
}


// ----------------------------------------------------------------------- //
// ----------------------- Reassignment and Drop ------------------------- //
// ----------------------------------------------------------------------- //
/*
As we learned before, we can reassign a new value to a mutable variable,
the memory of the old value will be freed, to make space for the new value
(done by function ```drop```).

This is how reassign works under the hood
*/

#[allow(unused_assignments)]
fn _demo_reassignment_drop() {
    let mut s = String::from("hello"); // create a mutable String whose contents are "hello"
    s = String::from("bonjour"); // reassign the mutable variable with new contents
                                 // everything on the stack and heap related the old contents are all gone (make space for this new one)
                                 // this all done by ```drop``` function

    println!("s_reassigned = {s}") // "bonjour"
}


// ------------------------------------------------------------ //
// ----------------------- CONCLUSION ------------------------- //
// ------------------------------------------------------------ //

/*
======= Stack data copied only =======
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = &s1; // borrowing, referencing

======= both Stack and Heap data copied =======
    let s1 = String::from("hello");
    let s2 = s1.clone();

======= Moved one variable to another =======
    let s1 = String::from("hello");
    let s2 = s1;

======= Reassignment - Drop old value =======
    let mut s1 = String::from("hello");
    s1 = String::from("aloha");

*/

///////////////////////
//       main()      //
///////////////////////

fn main() {
    _demo_scope();
    println!("===============================================================================");

    _demo_immutable_interact();
    println!("===============================================================================");

    _demo_mutable_borrow();
    println!("===============================================================================");

    _demo_mutable_moved();
    println!("===============================================================================");

    _demo_mutable_clone();
    println!("===============================================================================");

    _demo_reassignment_drop();
}
