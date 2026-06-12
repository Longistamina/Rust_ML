/*
 * The type ``HashMap<K, V>`` stores a mapping of keys of type K to values of type V using a hashing function,
 *  which determines how it places these keys and values into memory.
 * (like dictionary in Python)
 *
 * Hash maps are useful when you want to look up data not by using an index, as you can with vectors,
 * but by using a key that can be of any type.
 *
 * Hash maps store values on the heap.
 *
 * Hash maps are homogenous
 * -> All the keys must have the same type
 * -> All the values must have the same type
 *
 * Each key is assigned with one value only.
 * Many keys can store the same identical value
 */

 use std::collections::HashMap;

 fn main() {
     println!();

     // ---------------------------------------------------------------------------------------------------- //
     // --------------------------------------- Create a Hash Map ------------------------------------------ //
     // ---------------------------------------------------------------------------------------------------- //

     let mut scores = HashMap::new(); // Create an empty hash map

     scores.insert(String::from("Blue"), 10); // Add a new K-V pair to the existing hash map
     scores.insert(String::from("Yellow"), 50);

     println!("scores = {scores:?}");

     println!("\n===============================================================================\n");

     // ---------------------------------------------------------------------------------------------------- //
     // -------------------------------- Accessing Values in a Hash Map ------------------------------------ //
     // ---------------------------------------------------------------------------------------------------- //

     let team_name = String::from("Blue");
     let score = scores.get(&team_name).copied().unwrap_or(0);
     println!("score of {team_name} = {score}");
     /*
      * ``.get()`` returns an returns an ``Option<&V>``
      * then call ``.coppied()`` to get an ``Option<V>`` (not a reference)
      * then call ``.unwrap_or(0)`` to get the score, and set it to zero if there is no such entry key
      */

     let score = scores.get("Yellow").copied().unwrap_or(0);
     println!("score of Yellow = {score}");

     println!("\n===============================================================================\n");

     // ------------------------------------------------------------------------------------------------ //
     // -------------------------------- Iterate through a Hash Map ------------------------------------ //
     // ------------------------------------------------------------------------------------------------ //

     for (key, value) in &scores {
         println!("{key}: {value}")
     }

     // -------------------------------------------------------------------------------------------- //
     // -------------------------------- Ownership and Hash Map ------------------------------------ //
     // -------------------------------------------------------------------------------------------- //
     /*
      * For types that implement the Copy trait, like i32, the values are copied into the hash map.
      * For owned values like String, the values will be moved and the hash map will be the owner of those values.
      */

     let field_name = String::from("Favorite color");
     let field_value = String::from("Blue");

     let mut map = HashMap::new();
     map.insert(field_name, field_value);
     // field_name and field_value are invalid at this point, try using them and
     // see what compiler error you get!

     println!("\n===============================================================================\n");

     // ----------------------------------------------------------------------------------------- //
     // -------------------------------- Updating a Hash Map ------------------------------------ //
     // ----------------------------------------------------------------------------------------- //

     ///////////////////////
     // Overwrite a value //
     ///////////////////////

     let mut scores = HashMap::new();

     scores.insert(String::from("Blue"), 10);
     scores.insert(String::from("Blue"), 25); // Blue get updated from 10 to 25

     println!("{scores:?}");

     ////////////////////////////////////////////////////////
     // Adding a Key and Value Only If a Key Isn’t Present //
     ////////////////////////////////////////////////////////

     let mut scores = HashMap::new();
     scores.insert(String::from("Blue"), 10);

     scores.entry(String::from("Yellow")).or_insert(50); // Add completely new Yellow key, with value = 50
     scores.entry(String::from("Blue")).or_insert(50); // The Blue stays the same 10, not updated

     println!("{scores:?}");

     /////////////////////////////////////////////
     // Updating a Value Based on the Old Value //
     /////////////////////////////////////////////

     let text = "hello world wonderful world";

     let mut map = HashMap::new();

     for word in text.split_whitespace() {
         let count = map.entry(word).or_insert(0);
         *count += 1;
     }

     println!("{map:?}"); // {"world": 2, "hello": 1, "wonderful": 1}
 }
