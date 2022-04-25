use crate::http::Request;
use std::{convert::TryFrom, io::Read, net::TcpListener};
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("server running on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("server listening on {},{:?}", addr, stream);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved the request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => {
                                    println!("Failed to parse an request: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("unable to read from the connection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e)
                }
            }
        }
    }
}

// tuples in rust
// let tup = (5,"a",listener)
