/*
 * ``Vec<T>`` is vector.
 * Vectors allow you to store more than one value in a single data structure
 * that puts all the values next to each other in memory.
 *
 * They are useful when you have a list of items,
 * such as the lines of text in a file or the prices of items in a shopping cart.
 */

fn main() {
    println!();

    // --------------------------------------------------------------------------------- //
    // -------------------------------- Create a vector -------------------------------- //
    // --------------------------------------------------------------------------------- //

    let v_empty: Vec<i32> = Vec::new(); // Use ``Vec::new()`` to construct a new empty vector with type i32
    println!("v_empty = {v_empty:?}");

    let v1 = vec![1, 2, 3]; // Use ``vec![...]`` macro to create a vector, let Rust infer the dtype (here is i32 as default)
    println!("v1 = {v1:?}");

    println!("\n===============================================================================\n");

    // ----------------------------------------------------------------------------------- //
    // -------------------------------- Updating a vector -------------------------------- //
    // ----------------------------------------------------------------------------------- //

    let mut v2 = Vec::new(); // Create a mutable empty vector first
    v2.push(3);
    v2.push(5);
    v2.push(7);
    v2.push(9);

    println!("v2 updated = {v2:?}");

    println!("\n===============================================================================\n");


}
