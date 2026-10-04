/*
A union stores several possible field interpretations in the same memory region—similar to a C union.
```
union Number {
    integer: u32,
    float: f32,
}
```

Only one representation is meaningfully stored at a time.
When you read a field, Rust cannot prove that
the bits currently form a valid value of that field’s type, so reading it is unsafe.

You will mostly encounter unions in low-level code or C interoperability.
*/

fn main() {}
