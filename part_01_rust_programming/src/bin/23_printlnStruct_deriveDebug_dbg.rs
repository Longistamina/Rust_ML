/*
 * In Rust, after initilizing a custom struct, if we ``println!({}, struct_instance)``,
 * Rust will raise error.
 * -> error[E0277]: `StructName` doesn't implement `std::fmt::Display`
 *
 * The println! macro can do many kinds of formatting,
 * and by default, the curly brackets tell println! to use formatting known as Display:
 * -> output intended for direct end user consumption.
 *
 * But for Struct, the way println! should format the output is less clear
 *    (because there are more display possibilities)
 *    (possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown?)
 * -> structs don’t have a provided implementation of Display to use with ``println!`` and the {} placeholder.
 *
 * SOLUTIONS: use ``#[derive(Debug)]`` with ``{struct_instance:?}`` (or `{struct_instance:?}``),
 *            or ``dbg!(&struct_instance)``
 */

 #[derive(Debug)]
 struct Rectangle {
     length: u32,
     width: u32
 }
 /*
  * ``Debug`` here indicates the  an output format called Debug,
  * a format that is very useful for developers
  */

 fn area(dimensions: &Rectangle) -> u32 {
     dimensions.length * dimensions.width
 }

///////////////
//   main()  //
///////////////

 fn main() {

     // ------------------------------------------------------------------------------ //
     // ------------------------------ {:?} and {:#?} -------------------------------- //
     // ------------------------------------------------------------------------------ //

     let rectangle = Rectangle {
         length: 98,
         width: 32
     };

     println!("\nrectangle is {rectangle:?}"); // ``:?`` to use the ``Debug`` output format
     // rectangle is Rectangle { length: 98, width: 32 }

     println!("\nrectangle is {rectangle:#?}");
     /*
      * rectangle is Rectangle {
          length: 98,
          width: 32,
      }
      */

     let rectangle_area = area(&rectangle);
     println!("\nrectangle area = {rectangle_area}");

     println!("\n======================================================\n");

     // ---------------------------------------------------------------------- //
     // ------------------------------ dbg!() -------------------------------- //
     // ---------------------------------------------------------------------- //
     /*
      * ``dbg!()`` is a macro that takes the ownership of an expression (while ``println!()`` only takes reference),
      * then prints the file and line number of where that dbg! macro call occurs in your code,
      * along with the resultant value of that expression, and returns ownership of the value.
      */

      let rectangle2 = dbg!(Rectangle{length: 32, width: 25});
      // ``dbg!()`` takes the ownership of Rectangle{length: 32, width: 25}, then passes it to rectangle2
      /*
       * [part_01_rust_programming/src/bin/23_printlnStruct_deriveDebug_dbg.rs:73:24] Rectangle{length: 32, width: 25} = Rectangle {
           length: 32,
           width: 25,
       }
       */
       // Here, ``dbg!()`` also prints the file and line number of where that dbg! macro call occurs in the code

       println!(" ");
       dbg!(&rectangle2);
      let rectangle2_area = area(&rectangle2);
      println!("rectangle2 area = {rectangle2_area}");
      /*
       * Here, we use ``dbg!()`` again, but now we pass a reference ``&rectangle2``,
       * so the ``dbg!()`` could not take the ownership of ``&rectangle2``,
       * that's why ``&rectangle2`` is still valid, hence ``area()`` works
       */

       println!();
       dbg!(rectangle2);
       // let rectangle2_area = area(&rectangle2);
       /*
        * Here, we pass ``rectangle2`` directly to ``dbg!()``,
        * so ``dbg!()`` takes its ownership and ``rectangle2`` is no longer valid,
        * hence ``area(&rectangle2)`` cannot run anymore.
        */
}
