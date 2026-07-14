/*
 * In some programs, we need to write codes to handle errors, for example:
 ```
 if value < 1 || value > 100 {
     panic!("Guess value must be between 1 and 100, got {value}.");
 }
 ```
 * How can we test whether this error-handling section works or not?
 * => Use ``#[should_panic]``
 *
 * If the code panics -> test passes
 * If the code does not panic -> test fails
 *
 * For example, if we throw a ``value = 200``, we expect the code to panic,
 * and if it truly panics, it means that our error-handling code works
 * => the test should pass.
 *
 * In contrast, if the code still runs with ``value = 200`` without panic,
 * it means that our error-handling code has problems
 * => the test should fail
 *
 * This is how #[should_panic] works
 */

 #[allow(dead_code)]
 pub struct Score {
     value: f32,
 }

 impl Score {
     pub fn new(value: f32) -> Score {
         if !(1.0..=100.0).contains(&value) { // equivalent to ``if 1.0 < value || 100.0 > value``
             panic!("Score value must be between 1 and 100, got {value}.");
         }

         Score { value }
     }
 }

 mod tests {
     use super::*;

     #[test]
     #[should_panic]
     fn score_invalid() {
         Score::new(157.46); // this test will pass because 157.46 will panic the code, hence #[should_panic] is satisfied
     }

     #[test]
     #[should_panic]
     fn score_valid() {
         Score::new(89.2); // this test will fail because 89.2 will NOT panic the code, hence #[should_panic] is NOT satisfied
     }

     #[test]
     #[should_panic(expected = "less than or equal to 100")]
     fn score_invalid_fail() {
        Score::new(248.52); // this time, it will fail because we give it an ``expected`` message substring
     }
 }

 /*
 running 2 tests
 test test_05_should_panic::tests::score_valid - should panic ... FAILED
 test test_05_should_panic::tests::score_invalid - should panic ... ok

 failures:

 ---- test_05_should_panic::tests::score_valid stdout ----
 note: test did not panic as expected at part_01_rust_programming/sub_11_write_automated_tests/src/test_05_should_panic.rs:51:9

 failures:
     test_05_should_panic::tests::score_valid

 test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

 error: test failed, to rerun pass `-p sub_11_write_automated_tests --bin sub_11_write_automated_tests`
  */
