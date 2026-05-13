/*
Methods are similar to functions:
We declare them with the fn keyword and a name, they can have parameters and a return value

Unlike functions, methods are defined within the context of a struct (or an enum or a trait object),
and their first parameter is always self, which represents the instance of the struct the method is being called on.

Use ``impl`` to define a method of a struct
*/

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

fn main() {
    let rect1 = Rectangle{
        width: 25,
        length: 42
    };
    println!("rect1 area = {}", rect1.area());

    let rect2 = Rectangle{
        width: 44,
        length: 53
    };
    println!("rect2 area = {}", rect2.area())
}
