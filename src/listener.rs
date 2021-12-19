use std::net::{ToSocketAddrs, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::error::Error;

pub struct SimpleListener {
    listener: TcpListener,
}

impl SimpleListener {
    pub fn new<A>(listen_addr: A) -> SimpleListener
        where A: ToSocketAddrs
    {
        match TcpListener::bind(listen_addr) {
            Ok(server) => {
                println!("Starting TCP server.");
                return SimpleListener{listener: server}
            },
            Err(e) => {
                panic!("Tcplistener binding error. Error detail:{}", e);
            },
        };
    }

    pub fn run(&self) {
        for stream in self.listener.incoming().take(2) {
            println!("Getting request!");

            let mut stream = stream.unwrap();
            let request = read_request(&stream).unwrap();
            let request = std::str::from_utf8(&request).unwrap();
            println!("{}", request);

            let response = String::from("HTTP/1.1 200 OK \r\n\r\n");
            stream.write(response.as_bytes()).unwrap();
        }
    }
}

fn read_request(mut stream: &TcpStream) -> Result<Box<[u8]>, Box<dyn Error>> {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => Ok(Box::new(buffer)),
        Err(e) => Err(Box::new(e)),
    }
}
