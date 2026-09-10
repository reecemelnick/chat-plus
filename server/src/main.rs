use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::fmt::Display;

fn main() {
    println!("Server");
    let _ = create_server_socket();
}

fn create_server_socket() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
} 

fn handle_client(mut stream: TcpStream) {
    println!("Handling client");
    let mut buffer = [0; 10];
    stream.read(&mut buffer);

    if let Ok(text) = std::str::from_utf8(&buffer) {
        println!("{}", text);
    }
}