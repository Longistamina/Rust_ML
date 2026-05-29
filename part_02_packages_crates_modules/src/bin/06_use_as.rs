/*
 * ``use`` keyword helps shortcut the path to a function, struct, enum, ... from another crate
 * (like creating a symbolic link)
 * For example: ``use crate::front_of_house::hosting``
 * Later on, just need to call ``hosting``
 */

 // ---------------------------------------------------------------------------- //
 // ------------------------------- Basic usage -------------------------------- //
 // ---------------------------------------------------------------------------- //

 use restaurant::front_of_house::hosting; // Create a shortcut to ``hosting`` in ``restaurant::front_of_house::hosting``

 fn main() {
     println!();

     hosting::add_to_waitlist();
     println!("Added to waitlist")
 }

 // ----------------------------------------------------------------------------------- //
 // ------------------------------- Notice about scope -------------------------------- //
 // ----------------------------------------------------------------------------------- //
 /*
  * ``use`` only creates the shortcut for the particular scope in which the ``use`` occurs
  */

  // mod customer {
  //     pub fn eat_at_restaurant() {
  //         hosting::add_to_waitlist();
  //     }
  // }

  /*
   * Here, we create a new sub module named ``customer``,
   * then create a function named ``eat_at_restaurant()`` in this new submodule,
   * after that, we called ``hosting::add_to_waitlist()``
   * -> This is a completely different scope than the ``use`` statement
   * -> Cannot compile
   */

   // ---------------------------------------------------------------------------------------------------- //
   // ------------------------------- Re-exporting names with ``pub use`` -------------------------------- //
   // ---------------------------------------------------------------------------------------------------- //

   mod front_of_house {
       pub mod hosting2 {
           pub fn add_to_waitlist() {}
       }
   }

   pub use crate::front_of_house::hosting2;

   pub fn eat_at_restaurant() {
       hosting2::add_to_waitlist();
   }

   // Without ``pub use``, others must call ``restaurant::front_of_house::hosting::add_to_waitlist()``
   // With ``pub use``, just need ``restaurant::hosting::add_to_waitlist()``

   // ----------------------------------------------------------------------------------- //
   // ------------------------------- ``use ... as ...`` -------------------------------- //
   // ----------------------------------------------------------------------------------- //
   /*
    * ``use ... as ...`` helps provide new names for the types
    * -> solve the case where they have same name
    */

    use std::fmt::Result;
    use std::io::Result as IoResult;
