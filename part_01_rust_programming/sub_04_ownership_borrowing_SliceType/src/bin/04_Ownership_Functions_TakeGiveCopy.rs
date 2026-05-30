fn main() {

    // ------------------------------------------------------------------------------- //
    // ------------------------------ takes_ownership() ------------------------------ //
    // ------------------------------------------------------------------------------- //

    let s = String::from("hello"); // "s" comes into scope

    fn takes_ownership(some_string: String) {
        println!("some_string = {some_string}")
    }

    takes_ownership(s); // value of "s" is MOVED into the scope of takes_ownership function
                                    // so it is no longer valid here
                                    // memory of "s" is all freed.

    // println!("some_string again = {s}") // This will cause error since "s" has been moved to takes_ownership, no longer valid

    println!("===============================================================================");

    // -------------------------------------------------------------------------- //
    // ------------------------------ makes_copy() ------------------------------ //
    // -------------------------------------------------------------------------- //

    let x = 5; // "x" comes into scope

    fn makes_copy(some_integer: i32) {
        println!("some_integer = {some_integer}")
    }

    makes_copy(x); // Here, "x" is NOT MOVED as "s",
                                // this is because i32 has a Copy trait

    println!("some_integer (again) = {x}"); // x is still valid here thanks to the Copy trait, no error

    println!("===============================================================================");

    // ------------------------------------------------------------------------------- //
    // ------------------------------ gives_ownership() ------------------------------ //
    // ------------------------------------------------------------------------------- //

    fn gives_ownership() -> String {
        let s = String::from("This string will be yours"); // declare a String "s"

        s // "s" goes out of scope, function returns the value of "s" (give ownership),
    }

    let s1 = gives_ownership(); // s1 receives ownership
    println!("s1 = {s1}");

    println!("===============================================================================");

    // ------------------------------------------------------------------------------------ //
    // ------------------------------ takes_then_givesback() ------------------------------ //
    // ------------------------------------------------------------------------------------ //

    let s2 = String::from("I will take the ownership of s2, and give it to s3");

    fn takes_then_givesback(some_string: String) -> String {

        // some_string takes the ownership from the input,
        // then goes into scope

        some_string // some_string goes out of scope, its ownership as well as value is given back
    }

    let s3 = takes_then_givesback(s2);

    // println!("s2 = {s2}"); // Again, s2 has been moved to takes_then_givesback(), its ownership now belongs to s3
    println!("s3 = s2 = {s3}");

}
