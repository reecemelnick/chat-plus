use std::net::TcpStream;

fn main() {
    println!("Client");
    make_server_connection()
}

fn make_server_connection() {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:80") {
        println!("Connected to the server");
    } else {
        println!("Could not connect to the server...");
    }
}