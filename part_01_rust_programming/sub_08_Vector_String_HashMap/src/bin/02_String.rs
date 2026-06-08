/*
 * The String type, which is provided by Rust’s standard library rather than coded into the core language,
 * is a growable, mutable, owned, UTF-8 encoded string type.
 */

 fn main() {
     println!();
     // ----------------------------------------------------------------------------------- //
     // --------------------------------- Create a String --------------------------------- //
     // ----------------------------------------------------------------------------------- //

     /////////////////////////////////////////////////
     // Use ``String::new()`` or ``String::from()`` //
     /////////////////////////////////////////////////

     let string_empty = String::new(); // cannot be updated since it's immutable
     println!("string_empty = {string_empty}");

     let hello = String::from("안녕하세요");
     println!("hello = {hello}");

     let string_from = String::from("Created by String::from()");
     println!("string_from = {string_from}");

     ////////////////////////////
     // Use literl.to_string() //
     ////////////////////////////

     let literal: &'static str = "LITERAL_TO_STRING";

     let s_from_literal = literal.to_string(); // convert a static literal to a String
     println!("s_from_literal = {s_from_literal}");

     println!("\n===============================================================================\n");

     // ----------------------------------------------------------------------------------- //
     // --------------------------------- Update a String --------------------------------- //
     // ----------------------------------------------------------------------------------- //

     /////////////////////////////////////////////
     // Use string.push_str() and string.push() //
     /////////////////////////////////////////////

     let mut string_new = String::new(); // Create with ``mut`` to be mutable, grownable
     string_new.push_str("[Created empty ");
     string_new.push_str("just to be filled]");
     println!("string_new = {string_new}");

     let mut string_foobar = String::from("foo");
     let string_bar = "_bar";
     string_foobar.push_str(string_bar); // string_bar must be ``&str``, not String
     println!("string_foobar = {string_foobar}");

     let mut greeting = String::from("Bonjou");
     greeting.push('r'); // Must be a SINGLE ``char`` type ('r' not "r")
     println!("greeting = {greeting}");

     /////////////////////////////////////////////////
     // Use string.insert_str() and string.insert() //
     /////////////////////////////////////////////////

     let mut name = "Harry Potter".to_string();
     name.insert_str(0, "My name is ");
     println!("name = {name}")
 }
