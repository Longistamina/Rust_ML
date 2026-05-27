/*
 * Modules let us organize code within a crate for readability and easy reuse.
 * Modules also allow us to control the privacy of items because code within a module is private by default.
 * Private items are internal implementation details not available for outside use.
 *
 * We can choose to make modules and the items within them public,
 * which exposes them to allow external code to use and depend on them.
 *
 * ###################################
 *
 * As an example, let’s write a library crate that provides the functionality of a restaurant.
 *
 * In the restaurant industry, some parts of a restaurant are referred to as front of house and others as back of house.
 *
 * Front of house: is where customers are; this encompasses where the hosts seat customers,
 *              servers take orders and payment, and bartenders make drinks.
 *
 * Back of house: is where the chefs and cooks work in the kitchen,
 *                dishwashers clean up, and managers do administrative work.
 *
 * Let's create a new library named ``restaurant`` by running ``cargo new restaurant --lib``
 */

 fn main() {
     println!("Please refer to ``restaurant`` and ``restaurant/src/lib.rs`` for demonstration")
/*
 crate
  └── front_of_house
      ├── hosting
      │   ├── add_to_waitlist
      │   └── seat_at_table
      └── serving
          ├── take_order
          ├── serve_order
          └── take_payment

  By using modules, we can group related definitions together and name why they’re related.
  Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions,
  making it easier to find the definitions relevant to them.

  Programmers adding new functionality to this code would know where to place the code to keep the program organized.
 */
 }
