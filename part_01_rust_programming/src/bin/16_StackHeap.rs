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

fn main() {
  
}