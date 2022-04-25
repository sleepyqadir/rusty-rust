#![allow(dead_code)]
use http::{Method, Request}; 
use server::Server;
mod http;
mod server;

fn main() {
    let url = String::from("127.0.0.1:8080");

    let server = Server::new(url);

    server.run();
}

// notes

// &str is a String slice, a string slice is a immutable string
// enums in memory are save as 1,2,3,4,5,6

/*

 GET /user?id=10 HTTP/1.1\r\n
 HEADERS \r\n
 BODY

*/
