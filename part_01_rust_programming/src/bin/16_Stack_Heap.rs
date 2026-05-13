/*
Think of your program's memory like a workspace with two main areas:
the STACK and the HEAP.
*/

fn main() {

    /*
    ──────────────────────────────────────────────────────────────────────────────────────
    1. THE STACK  (simple, fast, neat)
    ──────────────────────────────────────────────────────────────────────────────────────

    Imagine a stack of plates 🍽️

    - You can only add or remove from the top
    - Last In, First Out (LIFO)

    In Rust:
    - Stores : local variables, function calls, return addresses
    - Managed: automatically
    - Speed  : very fast
    - Lifetime: cleaned up the moment a function ends
    */

    let x: u8 = 3; // fixed size, on the stack
    let y: u8 = 5; // fixed size, on the stack

    fn add_numbers(num1: u8, num2: u8) -> u8 { // fixed size, on the stack
        num1 + num2
    }

    let z = add_numbers(x, y); // fixed size, on the stack
    println!("z = x + y = {x} + {y} = {z}"); // fixed size, on the stack

    /*                          LAST IN FIRST OUT
    ┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐
    │ stacking_1  ┆ stacking_2  ┆ stacking_3  ┆ stacking_4  ┆ stacking_5  │
    ╞═════════════╪═════════════╪═════════════╪═════════════╪═════════════╡
    │     ...     ┆     ...     ┆     ...     ┆     ...     ┆  println!() │
    │     ...     ┆     ...     ┆     ...     ┆      z      ┆      z      │
    │     ...     ┆     ...     ┆ add_numbers ┆ add_numbers ┆ add_numbers │
    │     ...     ┆      y      ┆      y      ┆      y      ┆      y      │
    │      x      ┆      x      ┆      x      ┆      x      ┆      x      │
    └─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘

    ┌──────────────┬──────────────┬──────────────┬──────────────┬──────────────┐
    │ unstacking_1 ┆ unstacking_2 ┆ unstacking_3 ┆ unstacking_4 ┆ unstacking_5 │
    ╞══════════════╪══════════════╪══════════════╪══════════════╪══════════════╡
    │  println!()  ┆     ...      ┆     ...      ┆     ...      ┆     ...      │
    │      z       ┆      z       ┆     ...      ┆     ...      ┆     ...      │
    │ add_numbers  ┆ add_numbers  ┆ add_numbers  ┆     ...      ┆     ...      │
    │      y       ┆      y       ┆      y       ┆      y       ┆     ...      │
    │      x       ┆      x       ┆      x       ┆      x       ┆      x       │
    └──────────────┴──────────────┴──────────────┴──────────────┴──────────────┘
    */

     println!("===============================================================================");

    /*
    ───────────────────────────────────────────────────────────────────────────────────
    2. THE HEAP  (flexible, slower, vast)
    ───────────────────────────────────────────────────────────────────────────────────

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
    */

    let mut s1 = String::from("Hello"); // mutable, growable, changeable -> ptr on stack, data on heap
    s1.push_str(", I am Rust");
    println!("s1 = {s1}");

    let number: i8 = -3; // fixed size, on stack
    println!("number = {number}");

    /*
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
    ------------------------------------------------------------------------------------------------
                                           LAST IN FIRST OUT
    ┌──────────────────┬──────────────────┬──────────────────┬──────────────────┬──────────────────┐
    │    stacking_1    ┆    stacking_2    ┆    stacking_3    ┆    stacking_4    ┆    stacking_5    │
    ╞══════════════════╪══════════════════╪══════════════════╪══════════════════╪══════════════════╡
    │       ...        ┆       ...        ┆       ...        ┆       ...        ┆ println!(number) │
    │       ...        ┆       ...        ┆       ...        ┆      number      ┆      number      │
    │       ...        ┆       ...        ┆   println!(s1)   ┆   println!(s1)   ┆   println!(s1)   │
    │       ...        ┆  s1.push_str()   ┆  s1.push_str()   ┆  s1.push_str()   ┆  s1.push_str()   │
    │      s1_ptr      ┆      s1_ptr      ┆      s1_ptr      ┆      s1_ptr      ┆      s1_ptr      │
    └──────────────────┴──────────────────┴──────────────────┴──────────────────┴──────────────────┘

    ┌──────────────────┬──────────────────┬──────────────────┬──────────────────┬──────────────────┐
    │   unstacking_1   ┆   unstacking_2   ┆   unstacking_3   ┆   unstacking_4   ┆   unstacking_5   │
    ╞══════════════════╪══════════════════╪══════════════════╪══════════════════╪══════════════════╡
    │ println!(number) ┆       ...        ┆       ...        ┆       ...        ┆       ...        │
    │      number      ┆      number      ┆       ...        ┆       ...        ┆       ...        │
    │   println!(s1)   ┆   println!(s1)   ┆   println!(s1)   ┆       ...        ┆       ...        │
    │  s1.push_str()   ┆  s1.push_str()   ┆  s1.push_str()   ┆  s1.push_str()   ┆       ...        │
    │      s1_ptr      ┆      s1_ptr      ┆      s1_ptr      ┆      s1_ptr      ┆      s1_ptr      │
    └──────────────────┴──────────────────┴──────────────────┴──────────────────┴──────────────────┘

    */
    println!("===============================================================================");


    /*
        ──────────────────────────────────────
        2b. RUST'S MOST COMMON HEAP TYPES
        ──────────────────────────────────────

        Type       │  What it is                          │  Stack holds
        ───────────┼──────────────────────────────────────┼─────────────────
        Box<T>     │  Single heap value                   │  pointer
        Vec<T>     │  Growable array                      │  ptr + len + cap
        String     │  Growable UTF-8 text                 │  ptr + len + cap



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
 FINAL RULE OF THUMB  (Rust style)
──────────────────────────────────────

  → Small, fixed-size, temporary?      → STACK  (Rust does this by default)
  → Large, dynamic, or long-lived?     → HEAP   (use Box, Vec, String, etc.)

  The best part about Rust:
  You get the control of manual memory (like C++)
  with the safety of a garbage collector (like Java) —
  because Ownership handles it all at compile time. ✅
*/
}
