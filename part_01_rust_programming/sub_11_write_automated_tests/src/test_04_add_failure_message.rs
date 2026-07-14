/*
 * You can also add a custom message to be printed with the failure message as optional arguments to the
 * assert!, assert_eq!, and assert_ne!
 *
 * assert(condition, "message")!
 */

 pub fn greeting(name: &str) -> String {
     format!("Hello {name}!")
 }

 #[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn greeting_succeeds() {
         let name = "Carol";
         let result = greeting(name);
         assert!(result.contains(name));
     }

     #[test]
     fn greeting_fails() {
         let name = "Isabella";
         let result = greeting(name);
         assert!(result.contains("Heurichke"), "The greeting() does not contain Heurichk!") // ``assert!()`` with message
     }
 }

 /*
  running 2 tests
  test test_04_add_failure_message::tests::greeting_fails ... FAILED
  test test_04_add_failure_message::tests::greeting_succeeds ... ok

  failures:

  ---- test_04_add_failure_message::tests::greeting_fails stdout ----

  thread 'test_04_add_failure_message::tests::greeting_fails' (1658533) panicked at part_01_rust_programming/sub_11_write_automated_tests/src/test_04_add_failure_message.rs:27:10:
  The greeting() does not contain Heurichk!
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


  failures:
      test_04_add_failure_message::tests::greeting_fails

  test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

  error: test failed, to rerun pass `-p sub_11_write_automated_tests --bin sub_11_write_automated_tests`
  */
