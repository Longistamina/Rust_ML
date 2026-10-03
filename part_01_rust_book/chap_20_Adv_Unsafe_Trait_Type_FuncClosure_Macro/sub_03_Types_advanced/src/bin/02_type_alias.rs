/*
A type alias gives a type a shorter or more meaningful name

Aliases are useful when a type is long or repeated often. For example:
```
type Thunk = Box<dyn Fn() + Send + 'static>;
```

----------------------------------------------------------

The standard library uses the same idea for I/O results.
`std::io::Result<T>` is an alias for `std::result::Result<T, std::io::Error>`

It saves repetition while remaining the same Result type.
So the usual Result methods and the ? operator work with it.
*/

type Kilometers = u32; // give a type a more meaningful name
type Thunk = Box<dyn Fn() + Send + 'static>; // create short alias for long type

fn main() {
    println!();

    let distance: Kilometers = 5;
    println!("distance = {}km", distance);

    let f: Thunk = Box::new(|| println!("hi"));
    f();
}
