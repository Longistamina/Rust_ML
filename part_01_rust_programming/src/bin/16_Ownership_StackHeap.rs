/*
Rules of ownership:
+ Each value in Rust has an owner.
+ There can only be one owner at a time.
+ When the owner goes out of scope, the value will be dropped.
*/

// -------------------------------------------------- //
// ----------------- Variable Scope ----------------- //
// -------------------------------------------------- //
/*
Remind again, A scope is the range within a program for which an item is valid
*/

fn _demo_scope() {
    let s = "hello";
    
    {
        let s = "aloha"; // variable s="aloha" is only valid in this scope
        println!("Value of s is: {s}");
    }
    // Outside the scope, variable s="aloha" is not valid anymore
    // Here, only s="hello" is valid

    println!("Value of s is: {s}")
}

// ---------------------------------------------------------------- //
// ----------------- Stack and Heap - String Type ----------------- //
// ---------------------------------------------------------------- //
/*
Here, String Type will be used to illustrate Ownership, stack and heap
(Move to the bottom part of the file to understand about Stack and Heap)

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
fn _demo_StringType() {
  let s0 = "To day is so cool!"; // This s0 is a string literal (&str).
  println!("The value of s0 is: {s0}"); // s0 is stored on the stack, immutable, fixed size


  let s1 = String::from("hello"); // create a String object "from" literal string "hello"
  println!("The value of s1 is: {s1}");   // this s1 String will be stored on the heap

  let mut s2 = String::from("Good morning"); // a mutable String
  s2.push_str(", how are you?"); // push_str() appends a literal to a String
  println!("The value of s2 is: {s2}")
}

// Why can String be mutated but literals cannot? 
// The difference is in how these two types deal with memory (see Memory Allocation)

////////////////////
///    main()    ///
////////////////////

fn main() {
    // _demo_scope();
    _demo_StringType();
}

/*
Think of your program's memory like a workspace with two main areas:
the STACK and the HEAP.


──────────────────────────────────────
 1. THE STACK  (simple, fast, neat)
──────────────────────────────────────

Imagine a stack of plates 🍽️

  - You can only add or remove from the top
  - Last In, First Out (LIFO)

In Rust:
  - Stores : local variables, function calls, return addresses
  - Managed: automatically
  - Speed  : very fast
  - Lifetime: cleaned up the moment a function ends

Example:

  fn greet() {
      let age: i32 = 25;   // stored on the stack
  }                        // age is dropped automatically when greet() returns

👉 Key idea: Temporary, short-lived data
👉 In Rust, ALL primitives (i32, bool, f64, char...) go on the stack by default


──────────────────────────────────────
 2. THE HEAP  (flexible, slower, vast)
──────────────────────────────────────

Imagine a big storage room 📦

  - You can place things anywhere inside
  - You are responsible for remembering (and removing) them

In Rust:
  - Stores : dynamically allocated data — Box<T>, Vec<T>, String, etc.
  - Managed: automatically via Ownership + Drop (no GC, no manual free!)
  - Speed  : slower than the stack (allocation overhead)
  - Lifetime: persists until the owner goes out of scope, then auto-dropped

Example:

  let num = Box::new(10);
  //  ^^^               ← "num" pointer lives on the STACK
  //          ^^^^^^^   ← the value (10) lives on the HEAP

  // No "delete" needed! Rust drops it automatically when num goes out of scope.

👉 Key idea: Long-lived, flexible, dynamically sized data


──────────────────────────────────────
 2b. RUST'S MOST COMMON HEAP TYPES
──────────────────────────────────────

  Type       │  What it is                          │  Stack holds
  ───────────┼──────────────────────────────────────┼─────────────────
  Box<T>     │  Single heap value                   │  pointer
  Vec<T>     │  Growable array                      │  ptr + len + cap
  String     │  Growable UTF-8 text                 │  ptr + len + cap

  Example:

  let v: Vec<i32> = vec![1, 2, 3];
  //  ↑ the Vec struct (ptr, len, cap) → STACK
  //                   [1, 2, 3] data → HEAP

  let s: String = String::from("hello");
  //  ↑ the String struct (ptr, len, cap) → STACK
  //                        "hello" data → HEAP

  let s2: &str = "hello";   // ← string SLICE, points to read-only memory
                             //   NOT heap — this is baked into the binary!


──────────────────────────────────────
 3. STACK vs HEAP  (side by side)
──────────────────────────────────────

  Feature        │  Stack                  │  Heap
  ───────────────┼─────────────────────────┼──────────────────────────
  Speed          │  ⚡ Very fast           │  🐢 Slower
  Size           │  Small (MBs)            │  Large (GBs)
  Management     │  Automatic              │  Automatic via Ownership!
  Lifetime       │  Until scope ends       │  Until owner is dropped
  Structure      │  Ordered (LIFO)         │  Unordered, fragmented
  Stores         │  Primitives, refs       │  Box, Vec, String, etc.
  Main risk      │  Stack Overflow         │  No risk! Rust prevents leaks*

  * Rust's ownership model makes memory leaks nearly impossible.
    (Technically possible with Rc cycles, but very rare.)


──────────────────────────────────────
 4. EASY WAY TO REMEMBER  (Rust style)
──────────────────────────────────────

  STACK  =  short-term memory  →  let x: i32 = 5;
  HEAP   =  long-term storage  →  let x: Box<i32> = Box::new(5);

  The type tells you where it lives:
    - Known, fixed size at compile time?  → STACK  (i32, bool, [i32; 3])
    - Unknown or dynamic size?            → HEAP   (Vec, String, Box)


──────────────────────────────────────
 5. VISUAL
──────────────────────────────────────

  STACK                         HEAP
  (grows downward)              (spread freely)
  ┌──────────────────┐
  │ fn inner()       │  ← top  [ String data: "hello" ]
  │ fn outer()       │
  │ fn main()        │  ← base [ Vec data: 1, 2, 3    ]
  └──────────────────┘
   ptr │ len │ cap              [ Box data: 10         ]
   ────┼─────┼────
    ╰──┴─────┴──────────────────╯
       Stack holds the metadata,
       Heap holds the actual data


──────────────────────────────────────
 FINAL RULE OF THUMB  (Rust style)
──────────────────────────────────────

  → Small, fixed-size, temporary?      → STACK  (Rust does this by default)
  → Large, dynamic, or long-lived?     → HEAP   (use Box, Vec, String, etc.)

  The best part about Rust:
  You get the control of manual memory (like C++)
  with the safety of a garbage collector (like Java) —
  because Ownership handles it all at compile time. ✅
*/