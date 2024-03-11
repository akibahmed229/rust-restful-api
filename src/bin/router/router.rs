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
    
    let get = b"GET / HTTP/1.1\r\n"; // b"..." is a byte string literal
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";
    let put = b"PUT / HTTP/1.1\r\n";
    let delete = b"DELETE / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
       (HttpMethod::GET, "index.html") 
    } else if buffer.starts_with(sleep) {
       thread::sleep(Duration::from_secs(5));
       (HttpMethod::GET, "index.html")
    } else if buffer.starts_with(post) {
       (HttpMethod::POST, "index.html")
    } else if buffer.starts_with(put) {
       (HttpMethod::PUT, "index.html")
    } else if buffer.starts_with(delete){
        (HttpMethod::DELETE , "index.html")
     } else {
       (HttpMethod::INVALID, "404.html")
    }
}
