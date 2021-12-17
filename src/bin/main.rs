use std::net::TcpListener;

fn main() {
    match TcpListener::bind("127.0.0.1:60080") {
        Ok(server) => {
            println!("Starting TCP server.");
            handle_server(server);
        },
        Err(e) => {
            panic!("Tcplistener binding error. Error detail:{}", e);
        },
    };
}

fn handle_server(s: TcpListener) {
    for stream in s.incoming().take(2) {
        println!("Getting request!");
    }
}
