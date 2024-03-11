use std::net::{TcpListener, TcpStream}; // listens for incoming TCP connections
use std::io::prelude::*; // brings in the read and write traits

// custom modules
use server::ThreadPool; // file system module
use router::router::HttpMethod;
use router::router::parse_request;
use crate::controllers::controller::{handle_delete_request, handle_get_request, handle_invalid_request, handle_post_request, handle_put_request};

mod router {
    pub mod router;
}
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
        pool.execute(|| {
            handle_connecton(stream);
        });
    }
}

fn handle_connecton(mut stream: TcpStream) {
   let mut buffer = [0; 2048]; // buffer to store the data

    // read the data from the stream and store it in the buffer
    stream.read(&mut buffer).unwrap();     
    
    let ( method , path ) = parse_request(&buffer);
    
    // Route
    match method {
       HttpMethod::GET => handle_get_request(stream, path),
       HttpMethod::POST => handle_post_request(stream, path, &buffer),
       HttpMethod::PUT => handle_put_request(stream, path, &buffer),
       HttpMethod::DELETE => handle_delete_request(stream, path),
       _ => handle_invalid_request(stream)
    }
   
   println!(
       "Request: {}",
       String::from_utf8_lossy(&buffer[..])
   ); // print the request
}
