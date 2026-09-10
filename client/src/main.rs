use std::net::TcpStream;
use std::io::Write;

fn main() {
    println!("Client");
    make_server_connection()
}

fn make_server_connection() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:80") {
        println!("Connected to the server");
        stream.write(b"some bytes");
        chat_loop();
    } else {
        println!("Could not connect to the server...");
    }
}

fn chat_loop() {
    loop {

    }
}