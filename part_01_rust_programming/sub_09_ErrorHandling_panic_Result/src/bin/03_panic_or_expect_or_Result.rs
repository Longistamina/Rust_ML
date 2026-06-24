/*
 * When to use ``panic!()`` and when to use ``Result<T, E>``?
 *
 * Remind:
 * + If use ``panic!()``, the codes panic whether the error is recoverable or not.
 * + If use ``Result<T, E>``, we give the calling code options, and it can use these options to handle recoverable error
 *
 * In situations such as examples, prototype code, and tests,
 * it’s more appropriate to write code that panics instead of returning a Result.
 *
 * The ``unwrap`` and ``expect`` methods are very handy when you’re prototyping and you’re not yet ready to decide how to handle errors.
 * They leave clear markers in your code for when you’re ready to make your program more robust.
 */

 use std::net::IpAddr;

 // ------------------------------------------------------------------------------------------------- //
 // --------------- Use ``expect()`` when you have more information than the compiler --------------- //
 // ------------------------------------------------------------------------------------------------- //
 /*
  * If you can ensure by manually inspecting the code that you’ll never have an Err variant,
  * it’s perfectly acceptable to call expect and document the reason you think you’ll never have an Err variant in the argument text.
  */

  #[allow(dead_code)]
  fn demo_expect() {
      let home: IpAddr = "127.0.1"
          .parse()
          .expect("Hardcoded IP address should be valid");

      println!("home = {home}")
  }

  // ------------------------------------------------------------------------------------------------- //
  // -------------------------------- create custom types for validation  ---------------------------- //
  // ------------------------------------------------------------------------------------------------- //
  // https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#custom-types-for-validation

  pub struct Guess {
      value: i32
  }

  impl Guess {
      pub fn new(value: i32) -> Guess {
          if value < 1 || value > 100 {
              panic!("Guess value must be between 1 and 100, got {value}.")
          }

          Guess {value} // if the value is valid (1 <= value <= 100), then returns it
      }

      pub fn value(&self) -> i32 {
          self.value
      }
  }

  //////////////////////
  //      main()      //
  //////////////////////

  fn main() {
      demo_expect();
  }
