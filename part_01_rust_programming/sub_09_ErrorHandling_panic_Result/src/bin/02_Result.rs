/*
* Sometimes, there are errors that are not serious enough to panic
* => Use Result Type to handles Recoverable Errors
*
* enum Result<T, E> {
*    Ok(T),
*    Err(E),
* }
*
* T: the type of the value that will be returned in a success case within the ``Ok`` variant
* E: the type of the error that will be returned in a failure case within the ``Err`` variant
*
* => Use ``match`` to handle
*
* #############################################
*
* Can use ``error.kind()`` (from ``Err(error)``) to handle different kinds of errors
* (for io task like file handling, can use ``std::io::ErrorKind``)
*
* #############################################
*
* Handling multiple errors can result in a lot of ``match`` syntaxes, which is very verbose :((
*
* => Can use ``.unwrap_or_else(|error| {...})`` to handle this better
*/

use std::fs::File;
use std::io::ErrorKind;

// ------------------------------------------------------------------------------------------------- //
// ------------------------------------------ Result Type ------------------------------------------ //
// ------------------------------------------------------------------------------------------------- //

#[allow(unused)]
fn demo_result_type() {
    let file_result = File::open("hello.txt");
    /*
     * ``File::open()`` returns a ``Result<T, E>`` Type
     * => Use ``match`` to handle
     */

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem oppening file: {error:?}"),
    };

    dbg!(file);
}

// ------------------------------------------------------------------------------------------------ //
// ----------------------------- error.kind(): handle multiple errors ----------------------------- //
// ------------------------------------------------------------------------------------------------ //

#[allow(unused)]
fn demo_error_kind() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // ``match error.kind()`` to check error kind
            ErrorKind::NotFound => match File::create("hello.txt") {
                // If the error is ``NotFound``, create that file (also returns a Result Type, use match agian)
                Ok(fc) => fc, // return the new created file
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };

    dbg!(file);
}

// --------------------------------------------------------------------------------------------------------------------------- //
// ----------------------------- ``unwrap_or_else(|error| {...})``: beter handle multiple errors ----------------------------- //
// --------------------------------------------------------------------------------------------------------------------------- //

#[allow(unused)]
fn demo_unwrap_or_else() {
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    dbg!(file);
}

/*
 * As said before, ``File::open()`` returns a ``Result<T, E>`` Type.
 * So, ``.unwrap_or_else(error)`` will unwrap it to get the T in case it succeeds,
 * and return the ``error`` incase it fails
 * => This shortens the syntaxe compared to using ``match``
 *
 * About the |error|, it is the syntax used to define an input parameter for a closure.
 * + ``fn handle_error(error: std::io::Error) { ... }``: a normal function, parentheses () define the input
 * + ``|error| { ... }``                               : a closure (anonymous function), the `||` pipes the input to the function directly
 *
 * So in this case ``unwrap_or_else(|error| {...})``, instead of returning the ``error``,
 * we pipe the it into an anonymous function to better handling this case (check the ``error.kind()``, create new file, etc...)
 * => eliminates the usage of many verbose ``match`` syntaxes
 */

//////////////////////////
//        main()        //
//////////////////////////

fn main() {
    // demo_result_type();
    // demo_error_kind();
    demo_unwrap_or_else();
}
