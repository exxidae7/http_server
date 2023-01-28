use std::net::TcpListener;
use std::io::Read;
use std::convert::TryInto;
use std::convert::TryFrom;
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on Port {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("received a request : {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to read request : {}", e),
                            }
                            // let res : &Result<Request , _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection : {}", e),
                    }
                }

                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}