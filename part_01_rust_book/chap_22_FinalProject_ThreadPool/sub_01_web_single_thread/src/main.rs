/*
Before we begin, let’s look at a quick overview of the protocols involved in building web servers.

The two main protocols involved in web servers are
+ Hypertext Transfer Protocol (HTTP)
+ Transmission Control Protocol (TCP)

Both protocols are "request-response" protocols,
meaning a client initiates requests
and a server listens to the requests.
Then the server provides a response to the client.

The contents of those requests and responses
are defined by the protocols.

TCP is the lower-level protocol that describes the details of
how information gets from one server to another but doesn’t specify what that information is.

HTTP builds on top of TCP by defining the contents of the requests and responses.

We have another term called "Connection":
it is the name for the full request and response process
in which a client connects to a server,
the server generates a response,
then the server closes the connection
*/

use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    fs
};
// Use `TcpListener` to listens to a TCP connections.
// Use `TcpStream` for type annotation.
//
// Bring `std::io::BufReader` and `std::io::prelude` into scope to
// get access to traits and types that let us read from and write to the stream.
//
// use `fs` to read the contents from `hello.htm` and `404.html` to use it as response for the connection

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Create a listener that listens to the address 127.0.0.1:7878
    // The "127.0.0.1" is the IP address, representing our computer (same for mostly every computer)
    // The "7878" is the port
    // The `bind()` function works like the `new()` functions, it creates a `TcpListener` instance binding to port 7878
    // The binding can be successful or failed, so it returns a `Result<T, E>` -> use `unwrap()` to get the T, or panick if it is E

    for stream in listener.incoming() {
    // The `incoming()` method returns an iterator upon a sequence of streams (type TcpStream)
    // Each stream represents an open connection between the client and the server ("connection" explained above)
    // So, we will read this stream (TcpStream) to see what the client sent,
    // then we write the response to the stream and send it back to the client

        let stream = stream.unwrap();

        handle_connection(stream); // pass the `stream` to `handle_connection` function
    }
}

fn handle_connection(mut stream: TcpStream) {

    // ------------------------- //
    // Inspect the http_requests //
    // ------------------------- //

    let buf_reader = BufReader::new(&stream);
    // wrap the `stream` into a `BufReader` instance,
    // it adds buffering by managing calls to the `std::io::Read` trait methods for us.

    let http_request: Vec<_> = buf_reader // create a variable to store the requests read from the `BufReader` instance
        .lines() // return an iterator of `Result<String, std::io::Error>` by splitting the stream of data whenever it sees a newline byte
        .map(|result| result.unwrap()) // unwrap the result to get the `String`
        .take_while(|line| !line.is_empty()) // only take non-empty lines
        .collect();

    println!("Request: {http_request:#?}"); // print out the requests

    // ---- BOOK Implementation
    //
    // let buf_reader = BufReader::new(&stream); // create a new `buf_reader` here because the old `buf_reader` has been consumed
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    //
    // the `next` returns `Option<Result<String>>`, so unwrap to get the `Result<String>`,
    // then unwrap one more time to get the `String`.
    // This will return the first line of the `http_requests` only
    //
    // ---- BOOK Implementation

    let request_line = &http_request[0];
    // here, since we already read all the requests into the `http_request`,
    // we just need to indexing the first element to get the first request line,
    // no need to `next()` then `unwrap` twice like in the book

    // NOTE: don't try to create a second `buf_reader`,
    // because the first `buf_reader` already reads the entire request from the `TcpStream`,
    // so the second `buf_reader` will read nothing from the stream.
    // The `buf_reader` tries to get the input, while the browser waits for response,
    // resulting in a infinite mutual waiting loop.

    // set up components of the response
    let (status_line, filename) = {
        if request_line == "GET / HTTP/1.1" { // if get the expected request_line, then use status="OK", and filename="hello.html"
            ("HTTP/1.1 200 OK", "hello.html")
        } else { // else, use status="NOT FOUND", and filename="404.html"
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    let contents = fs::read_to_string(filename).unwrap(); // read contents from `filename` to use it for the response
    let length = contents.len();

    // create response from the components
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Write the response to `stream`.
    stream.write_all(response.as_bytes()).unwrap();
    // This modifies the `stream`, that's why we must pass `mut stream` to the function.
    // Here we convert the response string to bytes (with `as_bytes`) then write later.
    // Because the `write_all()` might fail, so we `unwrap` to get the result or panick
}

/*
```
cd path/to/sub_01_web_single_thread
cargo run
```

Then open 127.0.0.1:7878 address in the browser to see the rendered response
*/
