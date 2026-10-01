/*
An associated type is a type placeholder declared inside a trait.

The trait’s methods can use that placeholder,
and each implementation chooses what it stands for.

An example of associated types is the trait `Iterator`
*/

// =====================================================
// Define `Iterator` trait with associated type `Item`
// =====================================================

trait Iterator {
    type Item; // `type Item;` says, “An iterator has some item type, but the trait doesn’t pick it.”

    fn next(&mut self) -> Option<Self::Item>; // `Self::Item` means “the Item type associated with whichever type implements this trait.”
}

// =====================================================
// Define `Counter` struct
// =====================================================

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0 as u32}
    }
}

// ==================================================================================
// Implement method `Iterator::next()` for `Counter` with specified associated type
// ==================================================================================

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    let mut counter = Counter::new();
    while let Some(value) = counter.next() {
        println!("Got value = {}", value)
    }
}

// ==================================================================================
// How about using trait as parameter: `trait Iterator<T>`
// ==================================================================================

// The above logic could also be implemented like this
#[allow(dead_code)]
trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}
/*
impl Iterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        // ...
    }
}
*/


/*
The problem of this approach is that we have to annotate the types repeatedly,
like `let item: &mut dyn Iterator2<u32> = /* ... */;`

Using `type Item = u32` inside the trait definitiong removes this burden
*/

// ====================================================================
// When to use which?
// ====================================================================
/*
Use a generic trait parameter when callers may choose among different types for the same implementing type

Use an associated type when the implementation itself determines the type
*/
