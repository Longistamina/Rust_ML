use std::env; // ``std::env::args()`` returns an iterator of the arguments passed into the program

fn main() {
    println!();

    let args: Vec<String> = env::args().collect(); // use ``args().collect()`` to turn the argument iterator into a true vector collection

    let program_name = &args[0]; // access the 1st element, this is alway the name of the program
    let query = &args[1]; // access the 2nd element, save it in a variable named ``query``
    let file_path = &args[2]; // access the 3rd element, save it in a variable named ``file_path``

    println!("Program name: {program_name}");
    println!("Searching for {query}");
    println!("In file: {file_path}");
}

// cd .../sub_12_01_args_get_arguments
// cargo run -- test sample.txt
// (here, ``test`` is the 1st argument, ``sample.txt`` is the 2nd argument)
/*
    Program name: /home/longdpt/Documents/Academic/Rust_ML/target/debug/sub_12_01_args_get_arguments
    Searching for test
    In file sample.txt
 */

/*
-------- The args Function and Invalid Unicode --------

Note that ``std::env::args`` will panic if any argument contains invalid Unicode.

If your program needs to accept arguments containing invalid Unicode, use ``std::env::args_os`` instead.
That function returns an iterator that produces ``OsString`` values instead of ``String`` values.

We’ve chosen to use ``std::env::args`` here for simplicity because ``OsString`` values differ per platform
and are more complex to work with than ``String`` values.
 */
