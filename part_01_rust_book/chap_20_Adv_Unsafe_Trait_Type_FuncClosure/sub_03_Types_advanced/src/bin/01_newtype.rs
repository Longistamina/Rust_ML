/*
`newtype` pattern: wrap an existing type into a one-field tuple struct

For example:
```
struct Milimeters(u32);
struct Choice(bool);
*/

use std::ops::Add;

#[derive(Debug)]
struct Position(f32); // newtype pattern

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0)
    }
}

impl Clone for Position {
    fn clone(&self) -> Position {
        Position(self.0)
    }
}

fn main() {
    println!();

    let pos1 = Position(25.2);
    let pos2 = Position(-12.8);

    let pos = pos1.clone() + pos2.clone();

    println!("pos = {:?} + {:?} = {:?}", pos1, pos2, pos)
}
