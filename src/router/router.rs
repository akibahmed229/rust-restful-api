use std::thread; // thread module 
use core::time::Duration; // time module

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    INVALID,
}

// Function to parse HTTP method and path from the request
pub fn parse_request(buffer: &[u8]) -> (HttpMethod, &str) {
    // Implement parsing logic here to extract method and path from the buffer

    // match the buffer with the request method and path and return tuple of HttpMethod and path
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
