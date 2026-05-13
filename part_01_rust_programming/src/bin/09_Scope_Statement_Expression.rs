
const SCALE: f32 = 2.5; // local scope constant
static FREQ: f32 = 3.; // local scope static

// Or can use ``static mut FREQ`` but must use ``unsafe`` later

fn main () {
    // ------------------------------ //
    // ----------- Scope ------------ //
    // ------------------------------ //
    // Scope defines the region of code where a variable or item is valid, accessible, and authorized to exist
    // Scope are defined by curly braces {}

    let x = 10;

    {
        let x = x*3;
        println!("The value of x in the inner scope is: {x}"); // 30
    }

    let x = x + 2;

    println!("The value of x in the main scope is: {x}"); // 12

    println!("===============================================================================");

    // ----------------------------------------------------- //
    // ----------- Local Scope and Global Scope ------------ //
    // ----------------------------------------------------- //
    // Local Scope: refers to variables bound within a block, they will be dropped when scope ends
    // Global Scope: refers to items that are available program-wide (can be via ``constant`` or ``static``)

    let input_global: f32 = 33.;

    {
        let input_local: f32 = 22.;

        let result = input_local * SCALE + FREQ;
        println!("local = {result}");

        let result = input_global * SCALE + FREQ;
        println!("global (inner) = {result}")
    }

    let result = input_global * SCALE;
    println!("global (outer) = {result}");

    println!("===============================================================================");

    // ---------------------------------- //
    // ----------- Statement ------------ //
    // ---------------------------------- //
    // Statements in Rust are instructions that perform action, DO NOT return value

    let y = 6; // This is a statement, it just assigns value 6 to y, does not return any value
    println!("The value of y is: {y}");

    // let z = (let y = 7)
    // This will cause error, because (let y = 7) does not return any value
    // so we cannot assign (let y = 7) to z

    println!("===============================================================================");

    // ----------------------------------- //
    // ----------- Expression ------------ //
    // ----------------------------------- //
    // Expressions in Rust are instructions that perform action, then RETURN value

    let z = {
        let x: i32 = 3;
        x.pow(2)

        // This is the example of an expression
        // The x is defined, then powered, then returned
        // z will take the returned value from x

        // DON'T WRITE: let x = x.pow(2)
        // DON'T WRITE: x.pow(2);
        // The semicolon and the (let ... = ...) will make this become a statement, cannot return value
    };

    println!("The value of z = x^2 = 3^2 = {z}");

    // So when we write: let y = 6;
    // the 6 here is also an expression, it returns value 6 to y
}
