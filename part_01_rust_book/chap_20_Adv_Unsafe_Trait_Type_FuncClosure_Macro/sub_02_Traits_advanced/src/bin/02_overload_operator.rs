/*
Rust lets a parameter have a default type.
If an implementation does not provide that parameter,
Rust will use the default.
*/

// =====================================================
// Example with trait `Add` and `+` parameter
// =====================================================

use std::ops::Add;
/*
Let's look inside operator `+` as an example.
When we run `let x = y + z`, the sum is performed
by trait `Add` under the hood, it looks like this:
```
trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

`Rhs` means “right-hand side”: the type of the value after +.
`Rhs = Self` means that, by default, the right side has the same type as the implementing type.
`Output` is the type returned by the addition.
*/

// ================================================================================
// Define struct `Point` and implement trait `Add` for it to add another `Point`
// (add 2 SAME types together)
// ================================================================================

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Add for Point { // Because `Rhs` defaults to `Self`, `impl Add for Point` is effectively `impl Add<Point> for Point`.
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn demo_add_same_types() {
    let point1 = Point {x: 25.2, y: 6.8, z: 7.27};
    let point2 = Point {x: -15.4, y: 2.15, z: 256.};

    let point3 = point1 + point2; // NOTE: `point1` and `point2` will be consumed
    println!("point1 + point2 = {:?}", point3)
}
// point1 + point2 = Point { x: 9.800001, y: 8.950001, z: 263.27 }

// ==============================================================================================
// Define struct `Milimeters` and implement trait `Add` for it to add another struct `Meters`
// (add 2 DIFFERENT types together)
// ==============================================================================================

struct Milimeters(u32);
impl Clone for Milimeters {
    fn clone(&self) -> Milimeters {
        Milimeters(self.0)
    }
}

struct Meters(u32);
impl Clone for Meters {
    fn clone(&self) -> Meters {
        Meters(self.0)
    }
}

// Here, we implement clone trait for both of them so that we can call `meters.clone()`
// This help us retains a copy of the original value, since `Add::add()` consume the `self` and `Rhs`

impl Add<Meters> for Milimeters { // now `Rhs = Meters`, not as Self or `Milimeters` anymore
    type Output = Milimeters; // Tell Rust the output of this add should be `Milimeters` type

    fn add(self, other: Meters) -> Milimeters { // Instruct Rust how to add Milimeters and Meters
        Milimeters(self.0 + (other.0 * 1000)) // `self.0` to access the value
    }
}

fn demo_add_different_types() {
    let milimeters = Milimeters(469);
    let meters = Meters(3);

    let result = milimeters.clone() + meters.clone();

    println!("{}mm + {}m = {}mm", milimeters.0, meters.0, result.0)
}
// 469mm + 3m = 3469mm

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_add_same_types();
    demo_add_different_types();
}
