/*
The book `Design Patterns: Elements of Reusable Object-Oriented Software`
defines OOP in this way:
```
Object-oriented programs are made up of objects.
An object packages both data and the procedures that operate on that data.
The procedures are typically called methods or operations.
```

Based on this definition, Rust is truly object-oriented:
+ `Struct` and `Enum` have the data
+ `impl` blocks provide methods on structs and enums

Even though structs and enums with methods aren’t called objects,
they provide the same functionality.

Main characteristics of OOP:
+ Encapsulation: hide implementation details
+ Inheritance: type system and trait code sharing
+ Polymorphism: generics
*/

// =====================================================================================
// 1. Encapsulation: hide implementation details
// =====================================================================================
/*
Encapsulation means hiding the implementation details of an object,
so that codes using the object cannot access that implementation.

Therefore, the only way to interact with an object is through its public API.

This enables the programmer to change and refactor an object’s internals
without needing to change the code that uses the object.
*/

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value); // add new value to the list
        self.update_average(); // update new average value after adding
    }

    pub fn remove(&mut self) -> Option<i32> { // remove and return the last element of the vector
        let removed = self.list.pop();
        match removed {
            Some(value) => {
                self.update_average(); // Only update average after popping if there is still any element to pop
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&mut self) -> f64 { // return `object.average` when user calls `object.average()`
        self.average
    }

    fn update_average(&mut self) { // This method is not public, it is encapsulated (private)
        let total = self.list.iter().sum::<i32>();
        self.average = total as f64 / self.list.len() as f64
    }
}
/*
In this `struct AveragedCollection`,
the users can only access methods like `add`, `remove` and `average`
=> They are the only way to access and modify data.

Meanwhile, the `update_average` method is kept encapsulated.

The `self.list` and `self.average` fields are also kept private,
so that there is no way for external code
to add or remove items to or from them directly.
*/

fn demo_encapsulation() {
    let vector = vec![1, 3, 7, 82, 9, 10];
    let average = vector.iter().sum::<i32>() as f64 / vector.len() as f64;

    let mut avg_vector = AveragedCollection {
        list: vector,
        average: average
    };

    println!("Average before removing: {}", avg_vector.average());

    avg_vector.remove();

    println!("Average after removing: {}", avg_vector.average())
}

// =====================================================================================
// 2. Inheritance: type system and trait for code sharing
// =====================================================================================
/*
Inheritance is a mechanism whereby an object
can inherit elements from another object’s definition,
thus gaining the parent object’s data and behavior
without you having to define them again.

If a language must have inheritance to be object oriented,
then Rust is not such a language.

Rust does not have inheritance mechanism,
but it offers limited similar functionalities:
+ type system
+ trait for code sharing

About `Type system`, for example when you define an object as `f32`,
the object will gain all the characteristics and methods defined on `f32`
like `pow`, `sin`, `acos`, ...

About `Trait`, you can use trait to define shared behaviours.
For example, in Rust, both `f32` and `i64` implement `add` methods.
*/

// =====================================================================================
// 3. Polymorphism: generics
// =====================================================================================
/*
Polymorphism is a concept that refers to code
that can work with data of multiple types.
For inheritance, those types are generally subclasses of a bigger abstract type
(like Shape is a bigger concept than Square, Rectangle, Circle, ...)

Rust uses generics to abstract over different possible types
and trait bounds to impose constraints on what those types must provide.
This is sometimes called bounded parametric polymorphism.
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();
    demo_encapsulation();
}
