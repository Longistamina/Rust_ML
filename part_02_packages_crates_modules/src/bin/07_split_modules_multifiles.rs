/*
 * When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.
 *
 * To do so, first, in the root files (src/main.rs or src/lib.rs),
 * we declare ``mod module1;``, ``mod module2;`` without curly brackets.
 *
 * Then, create a file name ``module1.rs`` (src/module1.rs),
 * and put the ``pub mod module1 {}`` with curly brackets inside that file (do the same for ``module2`` and ``src/module2.rs``)
 *
 * By doing so, we allocate the definitions of ``module1`` and ``module2`` with curly brackets ``{}``
 * to separate files: ``module1.rs`` and ``module2.rs``
 *
 * Meanwhile, in the root files (src/main.rs or src/lib.rs), just need to declare and call out ``mod module1``,
 * no need to define the body of the modules in these root files
 */

 fn main() {}
