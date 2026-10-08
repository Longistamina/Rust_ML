/*
In the single-thread version, the server will process each request in turn,
meaning it won't process the second connection until the first one is finished processing.
Therefore, if the server receive more and more requests, this serial execution is not good.
=> Let's make it multi-threaded.

In this example, we will not spawn new thread for any new request,
this risks being attacked by DDoS.

Instead, we will create a thread pool with a finite number of threads,
so the code can only use those number of threads to host the server,
not all threads of the device.
*/

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use sub_02_web_multi_threads::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Create a pool with a finite number of threads
    let pool = ThreadPool::new(4); // the implementation of `ThreadPool` is in `src/lib.rs`

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] { // use `match` instead of `if`
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"), // 127.0.0.1:7878
        "GET /sleep HTTP/1.1" => { // 127.0.0.1:7878/sleep
            thread::sleep(Duration::from_secs(5)); // Simulate time-consuming task that makes sequential implementation a nightmare
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
