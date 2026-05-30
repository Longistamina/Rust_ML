pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// ############################################################################## //

pub mod front_of_house { // Define a module named ``front_of_house`` using ``pub mod`` keyword (so that other crates can access it)
    // goes in side the body of the module (inside the bracket {})

    pub mod hosting { // Place another module inside ``front_of_house`` module
        pub fn add_to_waitlist() {} // Define other things like function, struct, enum, etc inside a module

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

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

  #####################################################

  This tree shows how some of the modules nest inside other modules;
  for example, hosting nests inside front_of_house.

  The tree also shows that some modules are siblings, meaning they’re defined in the same module;
  hosting and serving are siblings defined within front_of_house.

  If module A is contained inside module B, we say that module A is the child of module B
  and that module B is the parent of module A.

  Notice that the entire module tree is rooted under the implicit module named crate.
 */
