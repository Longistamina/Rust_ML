/*
 * ``cargo test`` compiles the code in test mode and runs the resultant test binary.
 * By default, ``cargo test`` runs all the tests in parallel and capture the ouputs,
 * to prevent them from being displayed, making it easier to read the test result.
 *
 * ``cargo test`` has some options:
 * + cargo test --help => display the options you can use with ``cargo test``
 * + cargo test -- <test_file> --help => display the options you can use after the separator ``--``
 * + cargo test -- <test_file> --test-threads=2 => run the test with 2 parallel threads
 * + cargo test -p <package_name> -- <test_file> => run a specific test that belongs to a specific package
 * + cargo test -- <test_name> --show-output => run test with outputs displayed
 *
 * + cargo test <sub_test>
 *   cargo test -p <package_name> <test_file>::tests::<subtest_name>
 * => Filter and run specific tests only
 *    For example: ``cargo test -p sub_11_write_automated_tests test_07_CargoTest_options::tests::add``
 *                  -> only runs the test that contains "add" in the name (add_two_and_two, add_three_and_two)
 *
 * #[ignore] => ignore this test during ``cargo test``
 *
 * + cargo test -- <test_file> --ignored => Only runs the test flagged with ``#[ignore]``
 * + cargo test -- <test_file> --include-ignored => Runs both ignored and non-ignored tests

 */

 fn prints_and_returns_10(a: i32) -> i32 {
     println!("I got the value {a}");
     10
 }

 pub fn add_two(a: u64) -> u64 {
     a + 2
 }

 #[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn this_test_will_pass() {
         let value = prints_and_returns_10(4);
         assert_eq!(value, 10);
     }

     #[test]
     fn this_test_will_fail() {
         let value = prints_and_returns_10(8);
         assert_eq!(value, 5);
     }

     #[test]
     fn add_two_and_two() {
         let result = add_two(2);
         assert_eq!(result, 4);
     }

     #[test]
     fn add_three_and_two() {
         let result = add_two(3);
         assert_eq!(result, 5);
     }

     #[test]
     #[ignore] // ignore this test with during ``cargo test``, only runs with flags ``--ignored`` or ``--include-ignored``
     fn one_hundred() {
         let result = add_two(100);
         assert_eq!(result, 102);
     }

 }
