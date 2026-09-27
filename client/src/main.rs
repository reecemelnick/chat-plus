use std::net::TcpStream;
use std::io::Write;
use std::io;
mod register;
mod login;

fn main() {

    make_server_connection()
}

fn make_server_connection() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:80") {
        start_menu(&mut stream);
        chat_loop(stream);
    } else {
        println!("Could not connect to the server...");
    }
}

fn chat_loop(mut stream: TcpStream) {
    let mut input_string = String::new();
    while input_string.trim() != "x" {
        input_string.clear(); 
        io::stdin().read_line(&mut input_string).unwrap();
        println!("You wrote {}", input_string);
        let _ = stream.write(&input_string.clone().into_bytes());
    }
    println!("See you later!") 
}

fn start_menu(stream: &mut TcpStream) {

    loop {
        println!("Login (1)");
        println!("Register (2)");

        let mut menu_option = String::new();
        io::stdin().read_line(&mut menu_option).unwrap();
        
        match menu_option[..].trim() {
            "1" => {
                println!("Loging in...");
                login::login(stream);
                break;
            }
            "2" => {
                println!("Register user...");
                register::register(stream);
                break;
            }
            _ => {
                println!("Please select a valid option...");
            }
        }
    }
}