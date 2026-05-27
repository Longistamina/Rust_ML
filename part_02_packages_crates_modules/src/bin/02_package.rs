/*
 * A package is a bundle of one or more crates that provides a set of functionality.
 * A package contains a ``Cargo.toml`` file that describes how to build those crates.
 *
 * Cargo is actually a package that contains the binary crate for the command line tool you’ve been using to build your code.
 * The Cargo package also contains a library crate that the binary crate depends on.
 * Other projects can depend on the Cargo library crate to use the same logic the Cargo command line tool uses.
 *
 * A package can contain as many binary crates as you like, but at most only one library crate.
 * A package must contain at least one crate, whether that’s a library or binary crate.
 *
 * ######################################################
 *
 * When you run ``cargo new part_02_packages_crates_modules``,
 * the ``part_02_packages_crates_modules`` directory is now a package,
 * and the ``.rs`` files like ``01_crate.rs`` and ``02_package.rs`` inside are its crates.
 *
 * There is also a ``Cargo.toml`` file which gives us a package.
 *
 * ###########################################################
 *
 * The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.
 *
 * src/main.rs -> the crate root of a binary crate with the same name as the package.
 * src/lib.rs -> the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.
 *
 * => Cargo passes the crate root files to ``rustc`` to build the library or binary.
 *
 * ###########################################################
 *
 * Here, in this example, if our package ``part_02_packages_crates_modules`` only contains ``src/main.rs``,
 * it means that it only contains a binary crate named ``part_02_packages_crates_modules``
 *
 * ##########################################################
 *
 * If a package contains ``src/main.rs`` and ``src/lib.rs``,
 * it has two crates: a binary and a library, both with the same name as the package.
 *
 * A package can have multiple binary crates by placing files in the ``src/bin`` directory:
 * Each file will be a separate binary crate.
 * (like ``01_crate.rs`` and ``02_package.rs``)
 */

 fn main() {

 }
