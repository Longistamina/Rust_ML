/*
We have learned that we can use Trait to define shared behaviours for different types.
For example: types `String`, `Vec`, `Array`, ... all have `len()` or `count()`,
but they behave differently.
=> Use Trait to define that shared behaviours like `len()` or `count()`.

(In other language with inheritance, the idea is creating a general class named "Collection",
then define the method `len()` or `count()` inside it. Later on, other classed like `String`,
`Vec` or `Array` will inherit this "Collection" class and gain the `len()` method.)

--------------------------------------------------------------------------------------

In this example, we implement a small part of a GUI project.
The idea is that we will define a shared Trait named `Draw`,
so that any GUI objects added by the users can share and call
this `Draw` trait to display the object on the screen.

--------------------------------------------------------------------------------------

Finally, we will discuss a little bit about `static dispatch` and `dynamic dispatch`
*/

// =====================================================================
// 1. Defining a Trait for Common Behavior (example: Draw trait)
// =====================================================================

pub trait Draw {
    fn draw(&self);
}
// Define a trait named `Draw` that will have one method named `draw`.

// ================================================================================
// 2. Defining `Screen` struct to store GUI components as `Vec<Box<dyn Draw>>`
// ================================================================================
/*
As said, a GUI object can have different components (button, icon, ...)
to be drawn on the screen.

So, let's define a `Screen` struct to store these components.
*/

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}
/*
Our `components` here is a vector containing many different component.

Since these components are about to be drawn on the screen,
they should be bounded to trait `Draw`
=> `Vec<Box<dyn Draw>>`

Each component is a `Trait object`, which is `Box<dyn Draw>`.
Here, we must wrap the trait object inside a pointer `Box<T>`
because Trait objects in Rust have an unknown size!

For example, the fields of `Button` struct could be different in size
from the fields of `Textbox` struct => the data structures' sizes is different
(different height, different width, different color, ...)
=> wrap in a pointer so that they are all allocated on the heap,
   only the fixed-size pointer is on the stack.

`dyn` means "dynamically sized"

`dyn Draw` means "any type that implements Draw trait, no need to be the same type"

This method of using trait objects allows one Screen instance to hold a Vec<T>
that contains a Box<Button> as well as a Box<TextField>.
*/

// ==============================================================================================
// 3. Define `run()` method for `Screen`, loop through `components` and call `component.draw()`
// ==============================================================================================
/*
This `Screen` struct should implement method `run`,
so that when we call `screen.run()`, it will loop through
all its components and call `component.draw()` one by one
to display all the components on the screen.
*/

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
            println!()
        }
    }
}

// ==============================================================================================
// 4. Why not ``pub struct Screen<T: Draw> {pub components: Vec<T>,}``?
// ==============================================================================================
/*
If we define our `Screen` struct like this,

```
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

Rust will restrict us to a `Screen` instance
that has a list of components all of type `Button` or all of type `TextField`.

If you’ll only ever have homogeneous collections,
using generics and trait bounds is preferable
because the definitions will be monomorphized at compile time to use the concrete types.
*/

// ==============================================================================================
// 5. Implement trait `Draw` for screen components `Button` and `TextBox`
// ==============================================================================================
/*
Now we will define two different screen components as different types (structs),
then implement trait `Draw` differently for each of them.
*/

//--------//
// Button //
//--------//

pub struct Button {
    height: u32,
    width: u32,
    label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawn button with:");
        println!("+ height: {}", self.height);
        println!("+ width: {}", self.width);
        println!("+ label: {}", self.label);
    }
}

//---------//
// TextBox //
//---------//

pub struct TextBox {
    height: u32,
    width: u32,
    bgcolor: String
}

impl Draw for TextBox {
    fn draw(&self) {
        println!("Drawn textbox with:");
        println!("+ height: {}", self.height);
        println!("+ width: {}", self.width);
        println!("+ bgcolor: {}", self.bgcolor);
    }
}

// ==============================================================================================
// 6. Demo everything
// ==============================================================================================

#[allow(dead_code)]
fn demo_screen() {
    let screen = Screen { // define `screen` as an instance of struct `Screen`
        components: vec![
            Box::new(Button { // First component, wrapped in Box<T> -> Box<dyn Draw>
                height: 10,
                width: 25,
                label: "Play".to_string()
            }),

            Box::new(TextBox { // Second component, wrapped in Box<T> -> Box<dyn Draw>
                height: 35,
                width: 65,
                bgcolor: "Green".to_string()
            })
        ]
    };

    screen.run(); // run the screen, this will loop through each component and call `component.draw()`
}
/*
Drawn button with:
+ height: 10
+ width: 25
+ label: Play

Drawn textbox with:
+ height: 35
+ width: 65
+ bgcolor: Green
*/

// ==============================================================================================
// 7. Refactor with type hint: 2 methods
// ==============================================================================================

#[allow(dead_code)]
fn demo_screen_refactor_typehint() {
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { // First component, wrapped in Box<T> -> Box<dyn Draw>
            height: 22,
            width: 50,
            label: "Pause".to_string()
        }),

        Box::new(TextBox { // Second component, wrapped in Box<T> -> Box<dyn Draw>
            height: 40,
            width: 100,
            bgcolor: "Brown".to_string()
        })
    ];

    let screen = Screen {components}; // define a `screen` with known `components` list

    screen.run();
}
/*
Method 1:
```
let components: Vec<Box<dyn Draw>> = vec![
    Box::new(SelectBox { /* .. */ }),
    Box::new(Button { /* .. */ }),
];
```

Method 2:
```
let components = vec![
    Box::new(SelectBox { /* .. */ }) as Box<dyn Draw>,
    Box::new(Button { /* .. */ }),
];
```

-------------------------------------------------------

Why do we need type hint here after refactoring?

In the previous section, when we write
```let screen = Screen {components: vec![Box::new(...), Box::new(...)]}```,
since the `Screen.components` is already typed with `Vec<Box<dyn Draw>>`,
Rust knows the type of the vector `vec![]` inside the screen.

However, when we refactor the definition of `components` out of the `Screen {}`,
that type information does not attach to the external `components` vector anymore.
Consequently, Rust has to infer the type of that `components` on its own.

By default, `vec![]` requires elements' type to be homogeneous.
Hence, when we write ```let components = vec![Box::new(Button), Box::new(TextBox)]```,
Rust will infer that this `components` vector contains all elements of type `Box<Button>`,
which will make the compiler panick because the second element is `Box<TextBox>`, not `Box<Button>`

Therefore, we have to give it the type hint using either 2 mentioned methods
so that Rust becomes aware that the type of this vector is `Vec<Box<dyn Draw>>`,
which allows its elements to be from different types (Button or TextBox) with different size.
*/

// ==============================================================================================
// 8. `static dispatch` and `dynamic dispatch`
// ==============================================================================================
/*
This code is static dispatch,
all the elements of the `Vec<T>` must have the same type <T>,
hence have the same size.
```
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}
```

This code is dynamic dispatch,
the elements of the `Vec<Box<dyn Draw>>`
can be of different types (that implement `Draw` trait),
hence have different sizes.
```
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}
```

`static dispatch` is more optimized, runtime is faster,
but more rigid in Type and Size.

`dynamic dispatch` is more flexible in Type and Size,
but the trade-off is slower runtime,
because runtime must use the pointer to lookup to the object
to know which right specified `draw` method to call.
This lookup adds some overhead.
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    // demo_screen();
    demo_screen_refactor_typehint();
}
