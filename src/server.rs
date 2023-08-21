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
                Ok((_socket, addr)) => println!("New Client Accepted : {addr:?}"),
                Err(e) => println!("couldn't get client: {e:?}"),
            }     

        }

    }
}