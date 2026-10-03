/*
Rust generally needs to know a value’s size at compile time
so it can lay out storage for it.

Some types don’t have one fixed size known at compile time.
These are called `dynamically sized types` (DSTs).
*/

// ============================================
// Example with str
// ============================================

fn example_str() {
    // A str is a DST: its size depends on how many bytes of text it contains.
    // That means you can’t store a bare str as an ordinary local variable:

    // let s: str = "Hello"; // does not work

    // Instead, you use it behind a pointer, most commonly as a string slice:
    let s: &str = "Hello";
    println!("s = {}", s)
}

// ============================================
// Example with `dyn SomeTrait` objects
// ============================================
/*
Trait objects work on the same broad principle.

A trait such as `dyn SomeTrait` doesn’t specify one concrete value size,
so it is used behind a pointer:
```
&dyn SomeTrait
Box<dyn SomeTrait>
```
*/

// =========================================================================
// `Sized` and `?Sized`: marks types whose size is known at compile time
// =========================================================================
/*
The `Sized` trait marks types whose size is known at compile time.
Rust implicitly gives generic type parameters a `Sized` bound

This one `fn generic<T>(value: T) {}`
is actually this one:
```
fn generic<T: Sized>(value: T) {}
```

If you want a generic function to accept a potentially unsized type,
you can relax that bound with `?Sized`:
```
fn use_value<T: ?Sized>(value: &T) {}
```

Two details matter here:
- `?Sized` means “T might be Sized, or it might not be.”
- The parameter is `&T`, rather than T, because a possibly unsized value needs to be behind a pointer.
*/

fn main() {
    println!();
    example_str();
}
