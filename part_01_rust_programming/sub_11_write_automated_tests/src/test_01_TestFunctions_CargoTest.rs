/*
 * Tests are Rust functions that verify that the non-test code is functioning in the expected manner.
 * The bodies of test functions typically perform these three actions:
 * + Set up any needed data or state.
 * + Run the code you want to test.
 * + Assert that the results are what you expect.
 *
 * To change a function into a test function, add ``#[test]`` on the line before ``fn``
 *
 * #########################
 *
 * To run the test for the whole project: cargo test
 *
 * To run the test for a specific dependency: cargo test -p <Package_name> -- file_name (no .rs)
 * example: cargo test -p sub_11_write_automated_tests -- test_01_TestFunctions_CargoTest
 */

 pub fn add<T: std::ops::Add<Output = T>>(left: T, right: T) -> T { // The function to be tested
     left + right
 }

 #[cfg(test)]
 mod tests { // Create a tests module that stores one or many test functions, and perhaps with non-test helper functions too
     use super::*; // import all dependencies previously imported or defined by outer module in to this tests module
                   // for example, the ``pub fn add()`` function

     fn str_to_numeric(input: &'static str) -> f32 { // a helper function
         input.parse::<f32>().unwrap()
     }

     #[test] // tells Rust this is a test function
     fn it_works() {
         let left = "2.3";
         let right = 2.7;

         let left = str_to_numeric(left);

         let result = add(left, right);
         assert_eq!(result, 5.0); // if fails (not equal), panic
     }

     #[test]
     fn let_panic() { // Add another test, intentionally panic to see the outcome of a fail test
         panic!("Make this test fail");
     }
 }

 /* =============================================================================
  * ========================== Single test: it_works() ==========================
  * =============================================================================
  *
  * Run ``cargo test -p sub_11_write_automated_tests -- test_01_TestFunctions_CargoTest``
  *

  running 1 test
  test test_01_TestFunctions_CargoTest::tests::it_works ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

  * test_01_TestFunctions_CargoTest::tests::it_works => the function to be tested
  * test result: ok => all tests passed
  * 1 passed; 0 failed => totals the number of tests that passed or failed
  * 0 measured =>  statistic is for benchmark tests that measure performance (no measurements here)
  *
  * ================================================================================================
  * ========================== Multiple tests: it_works() and let_panic() ==========================
  * ================================================================================================

  running 2 tests
  test test_01_TestFunctions_CargoTest::tests::it_works ... ok
  test test_01_TestFunctions_CargoTest::tests::let_panic ... FAILED

  failures:

  ---- test_01_TestFunctions_CargoTest::tests::let_panic stdout ----

  thread 'test_01_TestFunctions_CargoTest::tests::let_panic' (573447) panicked at part_01_rust_programming/sub_11_write_automated_tests/src/test_01_TestFunctions_CargoTest.rs:34:10:
  Make this test fail
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


  failures:
      test_01_TestFunctions_CargoTest::tests::let_panic

  test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

  error: test failed, to rerun pass `-p sub_11_write_automated_tests --bin sub_11_write_automated_tests`
  */
