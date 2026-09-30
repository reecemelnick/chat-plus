use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;

// netstat -ano | findstr <PID>
// (Get-Process -Id <PID>).Threads.Count

fn main() {
    let pid = std::process::id(); // get server pid for connection monitoring
    println!("The current process ID is: {pid}");

    let _socket_creation_res = create_server_socket();
}

fn create_server_socket() -> std::io::Result<()> {

    if let Ok(listener) = TcpListener::bind("127.0.0.1:9000") {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move || {
                        let packet_identified =  intitial_read(&mut stream);
                        if packet_identified {
                            // handle_client(stream);   
                        }
                    });
                }
                Err(e) => {
                    println!("Unable to connect client... {}", e)
                }
            }
        }
    } else {
        println!("Failed to bind the socket...");
    }
    Ok(())
} 

fn login_user(buffer: &[u8]) {

    let username = String::new();
    let password = String::new();

    let username_length_bytes: [u8; 2] = buffer[1..3].try_into().expect("Slice was not 2 bytes long...");
    let username_length = u16::from_be_bytes(username_length_bytes);

    let password_length_bytes: [u8; 2] = buffer[(3+username_length) as usize..(3+username_length+2) as usize].try_into().expect("Slice was not 2 bytes long...");
    let password_length = u16::from_be_bytes(password_length_bytes);

    println!("userlen {}", username_length);
    println!("passwordlen {}", password_length);

    for byte in buffer {
        println!("{}", byte);
    }
}

fn intitial_read(stream: &mut TcpStream) -> bool {
    let mut buffer = [0; 512];
    if let Ok(bytes_read) = stream.read(&mut buffer) {

        if bytes_read <= 0 {
            println!("Failed to read. Killing");
            return false;
        }

        match buffer[0] {
            1 => {
                println!("Login");
                login_user(&buffer)
            }
            2 => {
                println!("Register");
            }
            _ => {
                println!("UNKNOWN PACKET...");
            }
        }

    } else {
        println!("Failed to read data...");
        return false;
    };

    true
}

fn handle_client(mut stream: TcpStream) {

    // chat loop : needs to be moved
    loop {
        let mut buffer = [0; 512];
        let Ok(bytes_read) = stream.read(&mut buffer) else {
            println!("Failed to read to buffer");
            break;
        };

        if bytes_read <= 0 {
            println!("Failed to read. Killing");
            break;
        }

        if let Ok(text) = std::str::from_utf8(&buffer) {
            println!("{}", text);
        }
    }
}