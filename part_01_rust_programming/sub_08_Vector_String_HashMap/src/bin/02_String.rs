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
     println!("name = {name}");

     let mut introduction = "My name is ".to_string();
     let name = 'A';
     introduction.insert(introduction.len(), name);
     println!("introduction = {introduction}");

     println!("\n===============================================================================\n");

     // ----------------------------------------------------------------------------------- //
     // --------------------- Concatenate with ``+`` or ``format!()`` --------------------- //
     // ----------------------------------------------------------------------------------- //

     ////////////////////////
     // Use ``+`` operator //
     ////////////////////////

     let s1 = String::from("Hello, ");
     let s2 = String::from("world!");
     let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
     // let s3 = s1.add(&s2);

     /*
      * The ``.add()`` takes literal string ``&str`` as input,
      * So theoretically, we cannot add two Strings together.
      * But the Rust compiler can coerce the &String argument into a &str
      * -> The codes still run
      */

     println!("s3 = {s3}");

     ///////

     let s1 = String::from("tic");
     let s2 = String::from("tac");
     let s3 = String::from("toe");

     let s = s1 + "-" + &s2 + "-" + &s3; // In this cases, use ``+`` is much more convenient than ``.add()``
     println!("s = {s}");

     ///////////////////////
     // Use ``format!()`` //
     ///////////////////////

     let s1 = String::from("tic");
     let s2 = String::from("tac");
     let s3 = String::from("toe");

     let s_format = format!("{s1}+{s2}+{s3}");
     println!("s_format = {s_format}");

 }
