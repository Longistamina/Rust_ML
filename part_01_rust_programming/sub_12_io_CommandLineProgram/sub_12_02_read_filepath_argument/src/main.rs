use std::env;
use std::fs;

fn main() {
    println!();

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for '{query}'");
    println!("In file: {file_path}");

    let contents = fs::read_to_string(file_path) // Use ``fs::read_to_string()`` to read the string contents in the file_path
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// cd .../sub_12_02_read_filephat_argument
// cargo run -- the ./source/poem.txt
/*
    Searching for 'the'
    In file: ./src/poem.txt
    With text:
    I'm nobody! Who are you?
    Are you nobody, too?
    Then there's a pair of us - don't tell!
    They'd banish us, you know.

    How dreary to be somebody!
    How public, like a frog
    To tell your name the livelong day
    To an admiring bog!
 */
