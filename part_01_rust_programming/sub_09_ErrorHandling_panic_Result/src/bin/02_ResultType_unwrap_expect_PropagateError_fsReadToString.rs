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

use std::fs::{
    self,
    File
};
use std::io::{
    self,
    ErrorKind,
    Read
};

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

// ---------------------------------------------------------------------------------------------------------- //
// ----------------------------- Shortcuts for Panic on Error 1 - ``unwrap()``) ----------------------------- //
// ---------------------------------------------------------------------------------------------------------- //
/*
 * The ``Result<T, E>`` has many helper methods (like the ``unwrap_or_else()``)
 * that are defined to do various, more specific tasks.
 *
 * ``unwrap()`` is a shortcut method implemented just like the ``match`` expression
 * + if result is ``Ok(value)``, ``unwrap()`` returns the value for us
 * + if result is ``Err(error)``, ``unwrap()`` will call the ``panic!()`` macro for us
 */

#[allow(unused)]
fn demo_unwrap() {
    let file = File::open("hello.txt").unwrap();
}

// ---------------------------------------------------------------------------------------------------------- //
// ----------------------------- Shortcuts for Panic on Error 2 - ``expect()``) ----------------------------- //
// ---------------------------------------------------------------------------------------------------------- //
/*
 * ``expect()`` method lets us also choose the ``panic!()`` error message.
 * Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier
 */

#[allow(unused)]
fn demo_expect() {
    let file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

// ---------------------------------------------------------------------------------------------------------- //
// ------------------------------------------- Propagating Errors ------------------------------------------- //
// ---------------------------------------------------------------------------------------------------------- //
/*
 * When a function’s implementation calls something that might fail,
 * instead of handling the error within the function itself,
 * you can return the error to the calling code so that it can decide what to do.
 * => This is called "propagating errors"
 */

 #[allow(unused)]
 fn demo_propagate_error() {

     fn read_contents_from_file() -> Result<String, io::Error> { // Read a file, return a Result type ``Result<String, io::Error>`` with specific types
         let file_result = File::open("hello.txt");              // read success -> String, read fail -> io::Error

         let mut file = match file_result {
             Ok(file) => file, // read success, returns the result to ``file``
             Err(e) => return Err(e), // stop the functions immediately and return the error (of type ``io::Error``)
         };

         // If the file_result is Ok(file), get the file then proceeds here

         let mut contents = String::new(); // Create an empty mutable String to store the contents

         /*
          * To read contents from a file, we use method ``read_to_string(&mut contents)`` of that file object,
          * the contents will be read and stored in the ``&mut contents``.
          *
          * But ``.read_to_string(&mut contents)`` will also return a Result Type incase the reading fails (like when the object is Err(e)),
          * so we have to use ``match file.read_to_string(&mut contents) {}`` to handle it
          */

         match file.read_to_string(&mut contents) {
             Ok(_) => Ok(contents), // return the ``Ok(contents)`` if it succeeds
             Err(e) => Err(e) // return the ``Err(e)`` if it fails
         }
         // we don't need to use ``return`` here be cause this ``match`` is the final expression
     }

     let output = read_contents_from_file();
     println!("\noutput = {output:?}") // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
 }

 // ----------------------------------------------------------------------------------------------------------- //
 // ---------------------------------- ``?`` operator for Propagating Errors ---------------------------------- //
 // ----------------------------------------------------------------------------------------------------------- //
 /*
  * The ``?`` placed after a Result value is defined to work in almost the same way as the match expressions that we used to handle the Result values
  *
  * For ``Result<T, E>?``:
  * + if it is ``Ok(T)``, the value ``T`` will be returned, and the program continues
  * + if it is ``Err(E)``, the entire ``Err(E)`` will be retunred as if we had used the ``return`` keyword in a function
  *
  * This shortens the handling codes alot
  * => The ``?`` operator eliminates a lot of boilerplate and makes this function’s implementation simpler
  */
#[allow(unused)]
fn demo_propagate_operator1() {
    fn read_contents_from_file() -> Result<String, io::Error> {
        let mut file = File::open("hello.txt")?; // if Ok(T), return T. Else if Err(E), return Err(E)
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // Read the contents into ``contents``, with the same match checking logic
        Ok(contents) // because the returning signature is ``Result<String, io::Error>``, we have to wrap it in ``Ok(contents)`` to match the type
    }

    let output = read_contents_from_file();
    println!("\noutput = {output:?}") // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
}

/*
 * We could even shorten the codes more like this using chaining methods
 */
#[allow(unused)]
fn demo_propagate_operator2() {
    fn read_contents_from_file() -> Result<String, io::Error> {
        let mut contents = String::new();
        File::open("hello.txt")?.read_to_string(&mut contents)?; // chaining all the steps in one place here
        Ok(contents)
    }

    let output = read_contents_from_file();
    println!("\noutput = {output:?}") // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
}

/*
 * Reading a file into a string is a fairly common operation
 * => Rust devs have made a standalone function ``fs::read_to_string()`` that puts all the steps together
 * (opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it)
 */

#[allow(unused)]
fn demo_fs_read2string() {
    fn read_contents_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    let output = read_contents_from_file();
    println!("\noutput = {output:?}") // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
}

/*
 * How to use ``?`` operator correctly?
 * (where and when)
 *
 * First, the ``?`` operator can only be used in functions whose return type is compatible with the type the ``?`` returns.
 * For example:
 * + if ``?`` returns ``Result<int32, Error>``, and the function also returns ``Result<int32, Error>`` -> Succeeds
 * + if ``?`` returns ``Result<int32, Error>``, but the function returns ``Result<f16, Error>`` -> Fails
 *
 * Second, the ``?`` only works when it follows something similar to ``Option<T>`` (like ``Result<T, E>``),
 * For example:
 * + ``File::open("hello.txt")?`` works
 * + ``String::new()?`` does NOT work
 */

 fn demo_operator_fail() {
     fn return_somehting() { // This function returns a unit type ``()`` (like general main() function)
         let file = File::open("hello.txt")?; // but this returns a ``File`` or an ``Err(E)``, not ``()``
         file // this will panic the program since the return types are mismatch
     }
 }

//////////////////////////
//        main()        //
//////////////////////////

fn main() {
    // demo_result_type();
    // demo_error_kind();
    // demo_unwrap_or_else();
    // demo_unwrap();
    // demo_expect();
    // demo_propagate_error();
    // demo_propagate_operator1();
    // demo_propagate_operator2();
    demo_fs_read2string();
}
