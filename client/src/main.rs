use std::net::TcpStream;
use std::io::Write;
use std::io;

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
                break;
            }
            "2" => {
                println!("Register user...");
                register(stream);
                break;
            }
            _ => {
                println!("Please select a valid option...");
            }
        }
    }
}

struct RegisterPayload {
    feature_id: u8,
    user_id: u16,
    payload_length: u16,
    payload: String,
}

fn register(stream: &mut TcpStream) {
    println!("Enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    println!("Enter your password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    let new_reg = RegisterPayload {
        feature_id: 1,
        user_id: 1,
        payload_length: 5,
        payload: String::from("hello"),
    };

    let byte_stream = serialize_register(new_reg);
    let _ = stream.write(&byte_stream);

}

fn serialize_register(reg: RegisterPayload) -> Vec<u8> {
    let mut byte_stream = Vec::new();

    byte_stream.push(reg.feature_id);
    byte_stream.extend_from_slice(&reg.user_id.to_be_bytes());
    byte_stream.extend_from_slice(&reg.payload_length.to_be_bytes());
    byte_stream.extend_from_slice(&reg.payload.as_bytes());

    byte_stream
}