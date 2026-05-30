use std::io::{self, Write}; // import Write to enable stdout().flush()

fn main() {
    print!("Please input your information: "); // print!() will not display the text, but store in a buffer, only waits for "\n" then then display
    io::stdout().flush().expect("Error: cannot flush the stdout!"); // manually flush the buffer, to force the text to be displayed

    let mut information = String::new();

    io::stdin()
        .read_line(&mut information)
        .expect("Error: cannot read the input!");

    println!("===============================================================================");

    println!("Your information is: {information}")
}
