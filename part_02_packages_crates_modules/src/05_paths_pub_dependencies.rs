/*
 * To show Rust where to find an item in a module tree,
 * we use a path in the same way we use a path when navigating a filesystem.
 *
 * Absolute path: starting from a crate root; for code from an external crate, the absolute path begins with the crate name,
 *                and for code from the current crate, it starts with the literal ``crate``.
 *
 * Relative path: starts from the current module and uses ``self``, ``super``, or an identifier in the current module.
 *
 * ``pub`` is the keyword to make module, function, struct, enum, etc public
 */

 /*
  * NOTE: to make ``restaurant`` available to ``05_paths.rs``
  * 1) Go to ``part_02.../Cargo.toml``
  * 2) Add ``restaurant = { path = "./restaurant" }`` below the ``[dependencies]``
  */

// ------------------------------------------------------ //
// --------------- Absolute/Relative Path --------------- //
// ------------------------------------------------------ //

use restaurant::front_of_house;

use crate::public_things::{Apetizer, Breakfast};

pub fn eat_at_restaurant() {
    // Absolute path (from ``restaurant``)
    restaurant::front_of_house::hosting::add_to_waitlist(); // Or can be ``crate::front_of_house::hosting::add_to_waitlist()``

    // Relative path (without ``restaurante::`` ahead)
    front_of_house::hosting::add_to_waitlist(); // After the first call with absolute path, can use relative path like this

    // NOTE: the ``hosting`` and ``add_to_waitlist()`` must be public (keyword ``pub``), or compiler will panic

}

/*
 * Items in a parent module can’t use the private items inside child modules,
 * but items in child modules can use the items in their ancestor modules.
 *
 * This is because child modules wrap and hide their implementation details,
 * but the child modules can see the context in which they’re defined.
 *
 * To continue with our metaphor,
 * think of the privacy rules as being like the back office of a restaurant:
 * What goes on in there is private to restaurant customers,
 * but office managers can see and do everything in the restaurant they operate.
 */

// ----------------------------------------------------------- //
// --------------- ``super`` and relative path --------------- //
// ----------------------------------------------------------- //

// Relative path with ``super``:
// to construct relative paths that begin in the parent module, rather than the current module or the crate root
// Using ``super`` allows us to reference an item that we know is in the parent module
// -> which can make rearranging the module tree easier,
//    when the module is closely related to the parent but the parent might be moved elsewhere in the module tree someday.

fn deliver_order() {}

mod something_new {
    fn fix_incorect_order() {
        process_order();
        super::deliver_order();
    }

    fn process_order() {}
}

// ---------------------------------------------------------------------------- //
// --------------- Use ``pub`` to make struct, enum, etc public --------------- //
// ---------------------------------------------------------------------------- //

mod public_things {
    pub struct Breakfast { // a public struct
        pub toast: String, // a public field
        seasonal_fruit: String, // a private field
        /*
         * Structs are often useful without their fields being public,
         * so struct fields follow the general rule of everything being private by default unless annotated with pub.
         */
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // a public method
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Apetizer {
        Soup,
        Salad
    /*
     * Enums aren’t very useful unless their variants are public;
     * it would be annoying to have to annotate all enum variants with pub in every case,
     * so the default for enum variants is to be public.
     */
    }
}

#[allow(unused_variables)]
pub fn eat_breakfast() {
    // Use public method ``public_things::Breakfast::summer()`` to create a mutable ``breakfast``
    let mut breakfast = public_things::Breakfast::summer("Croissant");

    // Modify public field ``toast`` is allowed.
    breakfast.toast = String::from("Baguette");
    println!("Breakfast toast = {}", breakfast.toast);

    // Modify a private field is not allowed here
    // breakfast.seasonal_fruit = String::from("Apple");

    // Define an apetizer with public enum ``public_things::Apetizer``
    // Because ``Apetizer`` is public, we use its variants like ``Salad`` and ``Soup``
    let apetizer = public_things::Apetizer::Salad;


}

// ########################### //

fn main() {
    println!();
    eat_breakfast();
}
