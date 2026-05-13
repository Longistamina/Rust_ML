/*
* Methods are similar to functions:
* We declare them with the fn keyword and a name, they can have parameters and a return value
*
* Unlike functions, methods are defined within the context of a struct (or an enum or a trait object),
* and their first parameter is always self, which represents the instance of the struct the method is being called on.
*
* Use ``impl`` to define a method of a struct,
* and use ``&self`` or ``&mut self``
* (``&self`` is short-hand for ``self: &Self``)
*
* Methods of a struct can have multiple parameters rather than just ``&self``
* When defining methods, can split into multiple ``impl`` blocks
*
* Fields' names and Methods' names can be overlapped.
* instance.something() -> Rust understands this as a method.
* intance.something -> Rust understands this as a field
*
* (Often, but not always, when we give a method the same name as a field,
* we want it to only return the value in the field and do nothing else.
* Methods like this are called getters, using ``pub fn`` to create.
* Getters are useful because you can make the field private,
* but the method public and thus enable read-only access to that field)
*
* A method that does not have ``&self`` parameter is called associated function.
* Can only be called by ``Struct::associated_function()`` (like String::from()), not from instance.
* Associated functions are usually used for constructors (returning a new instance of that struct)
* When defining associated function, should use ``Self`` if we want it to be a constructor
*/

use::std::f64::consts::PI;

// ---------------------------------------------------------------------- //
// -------------------- Struct with a single method --------------------- //
// ---------------------------------------------------------------------- //

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

impl Rectangle {
    fn area(&self) -> u32 { // Define method ``area`` for struct ``Rectangle``
        self.width * self.length
    }
}

// ----------------------------------------------------------------------------------------------- //
// ------------------------- Multi Methods - Multi impl blocks - Getters ------------------------- //
// ----------------------------------------------------------------------------------------------- //

struct Parallelogram {
    base: u32, // Private, read-only, access via getter ``pub``
    width: u32 //
}

impl Parallelogram { // First ``impl`` block
    fn area(&self) -> u32 {
        self.base * self.width
    }

    fn perimeter(&self) -> u32 {
        2 * (self.base + self.width)
    }
}

impl Parallelogram { // Second ``impl`` block

    pub fn base(&self) -> u32 { // A getter that grants read-only access to ``self.base``
        self.base
    }

    pub fn width(&self) -> u32 { // A getter that grants read-only access to ``self.width``
        self.width
    }
}

// ----------------------------------------------------------------------------------------- //
// ---------------------------------- Associated function ---------------------------------- //
// ----------------------------------------------------------------------------------------- //

struct Elipse {
    a: i32,
    b: i32
}

impl Elipse {
    fn circle(size: i32) -> Self { // An associated function that create a ``circle`` with struct ``Elipse``
        Self { // ``Self`` here refers to the thing returned after this impl, that is ``Elipse``
            a: size,
            b: size
        }
        // Construct an ``Elipse`` instance with a=b=size and return it (as a circle)
    }

    fn is_valid(dimension: i32) -> bool { // An asssociated function that check whether the dimension is valid (>0) or not
        dimension.ge(&0)
    }

    fn area(&self) -> f64 {
        let a = self.a as f64;
        let b = self.b as f64;
        PI * a * b
    }

}

////////////
// main() //
////////////

fn main() {
    println!();

    let rect1 = Rectangle{
        width: 25,
        length: 42
    };
    println!("rect1 area = {}", rect1.area());

    let rect2 = Rectangle{
        width: 44,
        length: 53
    };
    println!("rect2 area = {}", rect2.area());

    println!("\n======================================================\n");

    let parellogram = Parallelogram {
        width: 35,
        base: 22
    };

    let base =  parellogram.base(); // use getter to get the ``self.base`` field
    let width = parellogram.width();
    println!("Parellogram base = {}", base);
    println!("Parellogram width = {}", width);
    // println!("Parellogram valid (base, width) = {}, {}", parellogram.is_valid(base), parellogram.is_valid(width));
    println!("Parellogram perimeter = {}", parellogram.perimeter());
    println!("Parellogram area = {}", parellogram.area());

    println!("\n======================================================\n");

    let circle = Elipse::circle(5); // Use associated function to construct a circle

    let a = circle.a; // ``self.a`` and ``self.b`` have fixed size -> have copy trait
    let b = circle.b;

    let (valid_a, valid_b) = (Elipse::is_valid(a), Elipse::is_valid(b));

    println!("circle a == b: {}", a.eq(&b));
    println!("circle radius = a = b = {}", a);
    println!("circle check (a, b) = ({}, {})", valid_a, valid_b);
    println!("circle area = {}", circle.area())

}
