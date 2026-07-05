use crate::garden::vegetables::Asparagus; // Create shortcut for ``Asparagus`` struct

pub mod garden; // tells the compiler to include the code it finds in ``src/garden.rs``

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}")
}
