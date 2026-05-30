/*
 * A crate is the smallest amount of code that the Rust compiler considers at a time.
 * Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.
 *
 * A crate can come in one of two forms: a binary crate or a library crate.
 *
 * Binary crates are programs you can compile to an executable that you can run, such as a command line program or a server.
 * Each must have a function called ``main()`` that defines what happens when the executable runs.
 * All the crates we’ve created so far have been binary crates.
 *
 * Library crates don’t have a ``main()`` function, and they don’t compile to an executable.
 * Instead, they define functionality intended to be shared with multiple projects.
 * For example, the ``rand`` crate provides functionality that generates random numbers.
 *
 * (Most of the time when Rustaceans say “crate,” they mean library crate,
 * and they use “crate” interchangeably with the general programming concept of a “library.”)
 */

 fn main() {
     println!("This file '01_crate.rs' is a crate!")
 }
/*
 * Whenever we run ``rustc 01_crate.rs`` to compile ``01_crate.rs``,
 * the file ``01_crate.rs`` is called a crate
 */
