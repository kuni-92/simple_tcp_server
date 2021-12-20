use std::net::{ToSocketAddrs, TcpListener};

pub struct SimpleListener {
    listener: TcpListener,
}

mod  request {
    use std::net::TcpStream;
    use std::io::Read;
    use std::error::Error;

    pub fn read(mut stream: &TcpStream) -> Result<Box<[u8]>, Box<dyn Error>> {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => Ok(Box::new(buffer)),
            Err(e) => Err(Box::new(e)),
        }
    }
}

mod response {
    use std::net::TcpStream;
    use std::io::Write;

    pub fn write(mut stream: &TcpStream, response: String) {
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
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

            let stream = stream.unwrap();
            let request = request::read(&stream).unwrap();
            let request = std::str::from_utf8(&request).unwrap();
            println!("{}", request);

            let response_content = String::from("HTTP/1.1 200 OK \r\n\r\n");
            response::write(&stream, response_content);
        }
    }
}
