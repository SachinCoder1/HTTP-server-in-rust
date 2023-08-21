use crate::http::Request;
use std::convert::{TryFrom, TryInto};
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Server listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("recieved a request {}", String::from_utf8_lossy(&buf));

                            let res: &Result<Request, _> = &buf[..].try_into();

                            match Request::try_from(&buf[..]) {
                                Ok(request) => {}
                                Err(e) => println!("there is an error {e:?}"),
                            }
                        }
                        Err(e) => println!("there is an error {e:?}"),
                    }
                    println!("New Client Accepted : {addr:?}")
                }
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        }
    }
}
