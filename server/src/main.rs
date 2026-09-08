use std::net::{TcpListener, TcpStream};

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

fn handle_client(stream: TcpStream) {
    println!("Handling client");
}