use std::net::TcpListener;
use std::io::Read;

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
                        },
                        Err(e) => println!("there is an error {e:?}")
                    }
                    println!("New Client Accepted : {addr:?}")
                } ,
                Err(e) => println!("couldn't get client: {e:?}"),
            }     

        }

    }
}