/*
    This module contains the logic to handle different HTTP requests.
    It provides functions to handle GET, POST, PUT, DELETE, and invalid requests.
    Each function takes a TcpStream and a path as input, and returns the appropriate response.
*/

#![allow(unused)]

use std::net::{TcpListener, TcpStream}; // listens for incoming TCP connections
use std::io::prelude::*; // brings in the read and write traits
use std::fs;
use std::thread; // thread module 
use core::time::Duration; // time module

use crate::router::router::{ HttpMethod, handle_connecton}; 

// Function to parse HTTP method and path from the request
pub fn parse_request(buffer: &[u8]) -> (HttpMethod, &str) {
    // Implement parsing logic here to extract method and path from the buffer

    // match the buffer with the request method and path and return tuple of HttpMethod and path
    // b'' is a byte string literal wihch is a sequence of bytes
    match buffer {
        _ if buffer.starts_with(b"GET / HTTP/1.1\r\n") => (HttpMethod::GET, "index.html"),
        _ if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") => {
            thread::sleep(Duration::from_secs(5));
            (HttpMethod::GET, "index.html")
        },
        _ if buffer.starts_with(b"POST / HTTP/1.1\r\n") => (HttpMethod::POST, "index.html"),
        _ if buffer.starts_with(b"PUT / HTTP/1.1\r\n") => (HttpMethod::PUT, "index.html"),
        _ if buffer.starts_with(b"DELETE / HTTP/1.1\r\n") => (HttpMethod::DELETE, "index.html"),
        _ => (HttpMethod::INVALID, "404.html")
    }
}

// Function to handle GET requests
pub fn handle_get_request(mut stream: TcpStream, path: &str) {
    // Implement logic to read file content and send it in the response
    // read the contents of the file into a string
    let content = fs::read_to_string(path).unwrap(); 
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        "HTTP/1.1 200 OK",
        content.len(),
        content
    );

    // write the response to the stream
    stream.write(response.as_bytes()).unwrap(); 
    stream.flush().unwrap(); // flush the stream to ensure all data is sent
}

// Function to handle POST requests
pub fn handle_post_request(mut stream: TcpStream, path: &str, buffer: &[u8]) {
    // Implement logic to parse request body, create or append data to a file, and send appropriate response
}

// Function to handle PUT requests
pub fn handle_put_request(mut stream: TcpStream, path: &str, buffer: &[u8]) {
    // Implement logic to parse request body, update existing file content, and send appropriate response
}

// Function to handle DELETE requests
pub fn handle_delete_request(mut stream: TcpStream, path: &str) {
    // Implement logic to delete a file from the file system and send appropriate response
}

// Function to handle invalid requests
pub fn handle_invalid_request(mut stream: TcpStream, path: &str) {
    // Implement logic to handle invalid requests and send appropriate response
    let content = fs::read_to_string(path).unwrap(); 
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        "HTTP/1.1 200 OK",
        content.len(),
        content
    );

    // write the response to the stream
    stream.write(response.as_bytes()).unwrap(); 
    stream.flush().unwrap(); // flush the stream to ensure all data is sent
}
