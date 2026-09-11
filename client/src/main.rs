use std::net::TcpStream;
use std::io::Write;
use std::io;

fn main() {
    make_server_connection()
}

fn make_server_connection() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:80") {
        chat_loop(stream);
    } else {
        println!("Could not connect to the server...");
    }
}

fn chat_loop(mut stream: TcpStream) {
    loop {
        let mut input_string = String::new();
        while input_string.trim() != "x" {
            input_string.clear();
            io::stdin().read_line(&mut input_string).unwrap();
            println!("You wrote {}", input_string);
            stream.write(&input_string.clone().into_bytes());
        }
        println!("See you later!") 
    }
}