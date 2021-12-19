use std::net::{ToSocketAddrs, TcpListener, TcpStream};
use std::io::Read;
use std::error::Error;

pub struct SimpleListener {
    status: bool,
}

impl SimpleListener {
    pub fn new<A>(listen_addr: A) -> SimpleListener
        where A: ToSocketAddrs
    {
        match TcpListener::bind(listen_addr) {
            Ok(server) => {
                println!("Starting TCP server.");
               handle_server(server);
               return SimpleListener{status: true}
            },
            Err(e) => {
                panic!("Tcplistener binding error. Error detail:{}", e);
            },
        };
    }
}

fn handle_server(listener: TcpListener) {
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("Getting request!");
        let request = read_request(stream).unwrap();
        let request = std::str::from_utf8(&request).unwrap();
        println!("{}", request);
    }
}

fn read_request(mut stream: TcpStream) -> Result<Box<[u8]>, Box<dyn Error>> {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => Ok(Box::new(buffer)),
        Err(e) => Err(Box::new(e)),
    }
}
