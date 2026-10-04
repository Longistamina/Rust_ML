/*
Unsafe functions follow extra rules that Rust cannot check.

To create an unsafe function, use `unsafe` keyword, for example:
```
unsafe fn dangerous() {
...
}
```

To execute unsafe functions, we need to call it inside an unsafe block.
For example:
```
unsafe {
    dangerous();
}
```

In modern Rust, even inside unsafe fn,
individual unsafe operations should still sit in their own unsafe `{} blocks`.
This makes the genuinely risky lines easy to audit.
*/

// ==========================================================================
// Example: Split one mutable slice into two mutable slices
// ==========================================================================
/*
Suppose you want to do something like this:
```
let mut data = [1, 2, 3, 4, 5, 6];
let (left, right) = data.split_at_mut(3);

// left:  [1, 2, 3]
// right: [4, 5, 6]
Suppose you want to do something like this:
```

A naive implementation like this will fail:
```
fn split_at_mut(values: &mut [i32], mid: usize)
    -> (&mut [i32], &mut [i32])
{
    (&mut values[..mid], &mut values[mid..])
}
```
=> Why fails? Because the borrow checker sees two mutable borrows from values and refuses.

This is how we should do it (with raw pointers).
*/

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize)
    -> (&mut [i32], &mut [i32]) {

        let len = values.len();
        let ptr = values.as_mut_ptr(); // Returns an unsafe mutable pointer to the slice’s buffer.

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid), // Forms a slice from a pointer and a length, returns as a mutable
                slice::from_raw_parts_mut(ptr.add(mid), len - mid)
            )
        }
    }
/*
Step by step:
1. ptr points at the first slice element.
2. The assertion proves mid is in bounds.
3. The first slice covers elements 0..mid.
4. ptr.add(mid) points at element mid.
5. The second slice covers mid..len.
6. Those regions touch but do not overlap.

Therefore two `&mut` slices are valid.
The unsafe code is small; the public function remains safe because the function itself enforces its crucial precondition.
This is the ideal unsafe-Rust pattern: prove an invariant once internally, then offer everyone else a safe function.
*/

fn main() {
    println!();

    let mut data = [1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut data, 4);

    println!("left = {:?}", left);
    println!("rightt = {:?}", right);
}
/*
left = [1, 2, 3, 4]
rightt = [5, 6]
*/
