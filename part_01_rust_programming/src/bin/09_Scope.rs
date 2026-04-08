fn main () {
    // Scope defines the region of code where a variable or item is valid, accessible, and authorized to exist
    // Scope are defined by curly braces {}
    
    let x = 10;

    {
        let x = x*3;
        println!("The value of x in the inner scope is: {x}"); // 30
    }

    let x = x + 2;

    println!("The value of x in the main scope is: {x}"); // 12
}