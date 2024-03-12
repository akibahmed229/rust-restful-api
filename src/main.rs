/*
    This is the main file of the project. It creates a TCP listener and listens for incoming connections.
    When a connection is established, it creates a thread pool and assigns the connection to a thread in the pool.
    The thread then reads the request from the stream, parses the request to get the method and path, and calls the appropriate handler function.
    The handler function then processes the request and sends the response back to the client.
*/

use std::net::{TcpListener, TcpStream}; // listens for incoming TCP connections
use std::io::prelude::*; // brings in the read and write traits

// custom modules
use crate::lib::lib::ThreadPool; // file system module
use router::router::HttpMethod;
use router::router::parse_request;
use crate::controllers::controller::{handle_delete_request, handle_get_request, handle_invalid_request, handle_post_request, handle_put_request};

// thread pool
mod lib {
    pub mod lib;
}
// router
mod router {
    pub mod router;
}
// controller
mod controllers {
    pub mod controller;
}

fn main() {
    // bind to the address and port and start listening for incoming connections
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();

    let pool = ThreadPool::new(10); // create a thread pool with 4 threads

    // incoming() returns an iterator that gives us a sequence of streams
    // incoming().take(n) limits the number of connections to n and stops listening after n connections
    for stream in  listener.incoming().take(10){
        let stream = stream.unwrap();
        // send the connection to a worker thread in the pool for execution 
        pool.execute(|| {
            handle_connecton(stream); // handle the connection in a separate thread
        });
    }
}

fn handle_connecton(mut stream: TcpStream) {
    let mut buffer = [0; 2048]; // buffer to store the data

    // read the data from the stream and store it in the buffer
    stream.read(&mut buffer).unwrap();     

    // parse the request to get the method (GET,POST,PUT,DELETE) and path
    let ( method , path ) = parse_request(&buffer);

    // match the request method and call the appropriate handler function
    match method {
        HttpMethod::GET => handle_get_request(stream, path),
        HttpMethod::POST => handle_post_request(stream, path, &buffer),
        HttpMethod::PUT => handle_put_request(stream, path, &buffer),
        HttpMethod::DELETE => handle_delete_request(stream, path),
        _ => handle_invalid_request(stream, path)
    }

    println!(
        "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    ); // print the request
}
