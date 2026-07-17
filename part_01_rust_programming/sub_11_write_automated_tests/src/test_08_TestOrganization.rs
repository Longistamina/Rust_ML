/*
 * In the Rust community, tests belong to two main categories:
 * + unit tests (illustrated by test_01 .. test_07)
 * + integration tests
 *
 * #######################################################################################
 *                                       Unit tests
 * #######################################################################################
 *
 * The purpose of unit tests is to test each unit of code in isolation from the rest of the code
 * to quickly pinpoint where code is and isn’t working as expected.
 *
 * The convention is to create a module named ``tests`` in each file to contain the test functions
 * and to annotate the module with ``#[cfg(test)]``.
 *
 * Files test_01 .. test_07 demonstrate this
 *
 * The ``#[cfg(test)]`` annotation on the tests module tells Rust
 * to compile and run the test code only when you run ``cargo test``
 * (not when you run ``cargo build``)
 *
 * So when we run ``cargo build``, the modules with ``#[cfg(test)]`` will not be compiled
 * => save compile time and reduce the size of the compiled binary file
 *
 * #######################################################################################
 *                                  Integration tests
 * #######################################################################################
 *
 * In Rust, integration tests are entirely external to your library.
 * They use your library in the same way any other code would,
 * which means they can only call functions that are part of your library’s public API.
 *
 * Purpose: test whether many parts of your library work together correctly
 *
 * Why? Because units of code can work well separately,
 * but when they run in an integrated way, they could produce errors.
 *
 * We create a ``tests`` directory inside our project ``adder`` like this
 adder
 ├── Cargo.lock
 ├── Cargo.toml
 ├── src
 │   └── lib.rs
 └── tests
     └── test_01.rs
     └── test_02.rs
 ******************************************
 * This is the content of src/lib.rs

 pub fn add_two(a: u64) -> u64 {
     internal_adder(a, 2)
 }

 fn internal_adder(left: u64, right: u64) -> u64 {
     left + right
 }
 ******************************************
 * This is the content of test_01.rs

 use adder::add_two;

 #[test]
 fn it_adds_two() {
     let result = add_two(2);
     assert_eq!(result, 4);
 }
 ******************************************
 * Each file in the tests directory is a separate crate,
 * so we need to bring our library into each test crate’s scope.
 * That's why we add `` we add use adder::add_two``
 *
 * We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
 * Cargo treats the tests directory specially and compiles files in this directory only when we run ``cargo test``.
 ******************************************
 * Sometimes, the test files can share identical helper functions.
 * To reduce the repeated declaration, we can put the identical helper functions in one place,
 * that is ``tests/common/mod.rs``
 ├── Cargo.lock
 ├── Cargo.toml
 ├── src
 │   └── lib.rs
 └── tests
     ├── common
     │   └── mod.rs
     └── test_01.rs
     └── test_02.rs
 * The ``tests/common/mod.rs`` contains the identical functions that can be shared by ``test_01.rs`` and ``test_02.rs``
 * For example:
 use adder::add_two;

 mod common; // import module ``common``

 #[test]
 fn it_adds_two() {
     common::setup(); // use function ``setup()`` from module ``common``

     let result = add_two(2);
     assert_eq!(result, 4);
 }
 */
