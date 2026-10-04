use std::net::TcpStream;
use std::io::Write;
use std::io;

pub struct LoginPayload {
    feature_id: u8,
    username_len: u16,
    username: String,
    password_len: u16,
    password: String,
}

pub fn login(stream: &mut TcpStream) {
    println!("Enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    println!("Enter your password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    username = username.trim().to_string();
    password = password.trim().to_string();

    let login_data = LoginPayload {
        feature_id: 1,
        username_len: username.len() as u16,
        username: username,
        password_len: password.len() as u16,
        password: password,
    };

    let byte_stream = serialize_login(login_data);
    let _ = stream.write(&byte_stream);

}

pub fn serialize_login(login: LoginPayload) -> Vec<u8> {
    let mut byte_stream = Vec::new();

    byte_stream.push(login.feature_id);
    byte_stream.extend_from_slice(&login.username_len.to_be_bytes());
    byte_stream.extend_from_slice(&login.username.as_bytes());
    byte_stream.extend_from_slice(&login.password_len.to_be_bytes());
    byte_stream.extend_from_slice(&login.password.as_bytes());

    byte_stream
}