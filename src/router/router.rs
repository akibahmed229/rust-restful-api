/*
    Router module to parse the HTTP request and return the HTTP method and path
*/

use std::net::TcpStream; // listens for incoming TCP connections
use std::io::prelude::*; // brings in the read and write traits

use crate::controllers::controller::{ parse_request, handle_delete_request, handle_get_request, handle_invalid_request, handle_post_request, handle_put_request};

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    INVALID,
}

pub fn handle_connecton(mut stream: TcpStream) {
    let mut buffer = [0; 2048]; // buffer to store the data (e.g., request, response, file content, etc.)

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
        String::from_utf8_lossy(&buffer[..]) // convert the buffer to a string and print the request
    ); // print the request
}
