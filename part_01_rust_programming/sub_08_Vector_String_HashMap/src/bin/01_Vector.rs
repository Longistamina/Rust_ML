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

    // ----------------------------------------------------------------------------------------------------- //
    // ---------------------- Access items with indexing ``[]`` and method ``.get()`` ---------------------- //
    // ----------------------------------------------------------------------------------------------------- //

    let v3 = vec![0.1, 1.2, 2.3, 3.4, 4.5];

    ////////////////////////
    //      indexing      //
    ////////////////////////

    let third_element: &f64 = &v3[2]; // 2.3
    println!("third_element = {third_element}");

    println!("fourth_element = {}", &v3[3]); // 3.4

    /////////////////////
    //      .get()     //
    /////////////////////
    /*
     * NOTE: ``.get()`` will return ``Option<&T>`` -> use ``match`` or ``.unwrap_*()`` to get
     * -> Useful to handle out-of-bound index
     */

    println!();

    let second_element: Option<&f64> = v3.get(1);
    match second_element {
        Some(value) => println!("second_element = {value}"),
        None => println!("There is no second_element")
    }

    let out_of_bound: Option<&f64> = v3.get(100);
    match out_of_bound {
        Some(value) => println!("this_element = {value}"),
        None => println!("This element does not exist")
    }

    let fourth_element = v3.get(3).unwrap_or(&0.); // get the value at index 3, if out-of-bound, return 0.
    println!("fourt_element = {fourth_element}");

    println!("\n===============================================================================\n");

    // ----------------------------------------------------------------------------------------------------- //
    // ------------------------------ Remind mutable and immutable reference ------------------------------- //
    // ----------------------------------------------------------------------------------------------------- //

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    /*
     * This code will not run, because though ``v`` is a mutable vector,
     * ``first`` in the other hand is an immutable reference.
     * -> can’t have mutable and immutable references in the same scope.
     *
     * The ``v.push(6)`` adds a new element to the end of the vector
     * -> this might require allocating new memory and copying the old elements to the new space
     * (if there isn’t enough room to put all the elements next to each other where the vector is currently stored)
     * -> the reference to the first element would be pointing to deallocated memory!! (Not allowed)
     */

    // ----------------------------------------------------------------------------------------------------- //
    // -------------------------------------- Iterating over a vector -------------------------------------- //
    // ----------------------------------------------------------------------------------------------------- //

    ////////////////////
    // for i in &v {} //
    ////////////////////

    let v4 = vec![100, 32, 57];

    println!("Loop with immutable reference:");
    for i in &v4 { // immutable reference of each element in v4
        println!("{i}");
    }

    ////////////////////////
    // for i in &mut v {} //
    ////////////////////////

    let mut v5 = vec![100, 32, 57];

    println!("\nLoop with mutable reference:");
    for i in &mut v5 {
        *i += 50 // ``*`` is a dereference operator (talk about it later)
                 // it helps us get the true value of ``i`` = [100, 32, 5] before running ``+= 50`` in each loop
    }

    println!("v5 elements = {v5:?}"); // [150, 82, 107]

    println!("\n===============================================================================\n");

    // ----------------------------------------------------------------------------------------------------- //
    // ------------------------------------------ Vector and Enum ------------------------------------------ //
    // ----------------------------------------------------------------------------------------------------- //
    /*
     * Vectors can only store values that are of the same type.
     * This can be incovenient because sometimes we would need to store a list of items of different types.
     * -> Combine Vector with Enum (because the variants of an enum are defined under the same enum type)
     */

     #[allow(dead_code)]
     #[derive(Debug)]
     enum Data {
         Id(i32),
         Name(String),
         Score(f64)
     }

     let agent0 = vec![
         Data::Id(0),
         Data::Name(String::from("Cressida Bright")),
         Data::Score(257.25)
     ];

     let agent1 = vec![
         Data::Id(1),
         Data::Name(String::from("James Bond")),
         Data::Score(258.92)
     ];

     let agent2 = vec![
         Data::Id(2),
         Data::Name(String::from("Lennex Monroe")),
         Data::Score(256.16)
     ];

     println!("agent0 = {agent0:?}");
     println!("agent1 = {agent1:?}");
     println!("agent2 = {agent2:?}");

     println!("\n===============================================================================\n");

     // ----------------------------------------------------------------------------------------------------- //
     // ----------------------------- Dropping a vector also drops its elements ----------------------------- //
     // ----------------------------------------------------------------------------------------------------- //

     fn declare_vector_then_drop() {
         let v = vec![1, 2, 3, 4];

         println!("v = {v:?}")
     } // vector ``v`` and all its elements are dropped here after this scope

     declare_vector_then_drop();
}
