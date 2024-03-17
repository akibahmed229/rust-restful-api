/*
    This is the main file of the project. It creates a TCP listener and listens for incoming connections.
    When a connection is established, it creates a thread pool and assigns the connection to a thread in the pool.
    The thread then reads the request from the stream, parses the request to get the method and path, and calls the appropriate handler function.
    The handler function then processes the request and sends the response back to the client.
*/

use std::net::TcpListener; // listens for incoming TCP connections

// custom modules
use crate::lib::lib::ThreadPool; // file system module
use router::router::handle_connecton;

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
