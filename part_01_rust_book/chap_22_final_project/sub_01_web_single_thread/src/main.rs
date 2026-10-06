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

use std::net::TcpListener; // Use `TcpListener` to listens to a TCP connections.

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

        println!("Connection established")
    }
}
