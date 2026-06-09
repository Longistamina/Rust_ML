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

     println!("\n===============================================================================\n");

     // --------------------------------------------------------------------------------------------------------------- //
     // ---------------------- Rust DOES NOT support indexing and slicing for String and &str ------------------------- //
     // --------------------------------------------------------------------------------------------------------------- //

     ///////////////////////////////////////////////////////////////////////////////////////
     // Rust does not support indexing with ``string[index]`` for String and literal &str //
     ///////////////////////////////////////////////////////////////////////////////////////

     let s1 = String::from("hola");
     // let first = s1[0];
     println!("s1_bytes = {:?}", s1.bytes()); // s1_bytes = Bytes(Copied { it: Iter([104, 111, 108, 97]) })

     let s2 = "Здравствуйте"; // &str
     // let first = hello[0];
     println!("s2_bytes = {:?}", s2.bytes()); // s2_bytes = Bytes(Copied { it: Iter([208, 151, 208, 180, 209, 128, 208, 176, 208, 178, 209, 129, 209, 130, 208, 178, 209, 131, 208, 185, 209, 130, 208, 181]) })

     /*
      * These codes could not run
      *     let first = s1[0];
      *     let first = hello[0];
      *  because of several reasons.
      *
      * A String is a wrapper over a Vec<u8>.
      * The ``s1 = String::from("hola")`` has length = 4 bytes
      * -> each character occupies 1 byte
      * -> each byte represents 1 unicode value
      *
      * But in the case of ``s2 = "Здравствуйте"``,
      * its length = 24
      * -> each character occupies 2 bytes
      * -> each character needs 2 unicode values
      *
      * => Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.
      *
      * For example, in the ``s2 = "Здравствуйте"``
      * look at the character "3" (not three but the Cyrillic letter Ze),
      * this "3" takes two bytes, each byte stores a unicode value
      * the 1st byte is 208
      * the 2nd byt is 151
      * -> So when we index ``s2[0]``, what we got is the unicode value of the 1st byte: 208
      * -> But this 208 unicode value alone is not a valid character on its own
      * -> indexing fail...
      */

      /////////////////////////////////////////////////////////////////////////////////////
      // Rust does not support slicing with ``string[i..j]`` for String and literal &str //
      /////////////////////////////////////////////////////////////////////////////////////

      /*
       * For the same reason above, with ``s2 = String::from("Здравствуйте");``,
       * if we slice it like ``s2[0..2]``, we will get these unicode values: [208, 151, 208]
       * -> Fail because the last 208 alone is not a valid character
       *
       * Even when we do ``s2[0..1]`` and get [208, 151] (a complete character),
       * -> still fail
       */

       println!("\n===============================================================================\n");

       // --------------------------------------------------------------------------------------------------------------------- //
       // ---------------------- Iterate through a String or a &str with ``.chars()`` or ``.bytes()`` ------------------------- //
       // --------------------------------------------------------------------------------------------------------------------- //

       //////////////////////
       // Use ``.chars()`` //
       //////////////////////

       let s = String::from("Здравствуйте");
       println!("s_chars = {:?}", s.chars()); // Chars(['З', 'д', 'р', 'а', 'в', 'с', 'т', 'в', 'у', 'й', 'т', 'е'])

       for c in s.chars() {
           println!("{c}")
       }

       println!();

       //////////////////////
       // Use ``.bytes()`` //
       //////////////////////

       let s = "Hermione";
       println!("s_bytes = {:?}", s.bytes());
       for unicode_value in s.bytes() {
           println!("{unicode_value}")
       }

       println!("\n===============================================================================\n");

       // ---------------------------------------------------------------------------------------------------------------- //
       // ---------------------- Indexing a String with ``.chars().nth()`` or ``.bytes().nth()`` ------------------------- //
       // ---------------------------------------------------------------------------------------------------------------- //
       /*
        * Since ``.chars()`` and ``.bytes()`` return an iterator,
        * we can use their method ``.nth(index)`` to return the nth element (0-based index) in the iterator
        * NOTE: it return an ``Option<<Chars<'_>``
        * -> Use ``string.chars().nth(index).unwrap()`` to get the value (same for ``.bytes().nth()``)
        */

        let s = String::from("Здравствуйте");

        let s_2nd_char = s.chars().nth(1).unwrap();
        println!("s_2nd_char = {s_2nd_char}");

        let s_4th_byte = s.bytes().nth(3).unwrap();
        println!("s_4th_byte = {s_4th_byte}")
 }
