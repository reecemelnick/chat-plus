use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;
mod login;
mod register;
mod database_manager;

// netstat -ano | findstr <PID>
// (Get-Process -Id <PID>).Threads.Count

fn main() {
    let pid = std::process::id(); // get server pid for connection monitoring
    println!("The current process ID is: {pid}");

    database_manager::init_db();

    let _socket_creation_res = create_server_socket();
}

fn create_server_socket() -> std::io::Result<()> {

    if let Ok(listener) = TcpListener::bind("127.0.0.1:9000") {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move || {
                        let new_client =  intitial_read(&mut stream);
                        if new_client {
                            println!("New client established...");
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

fn intitial_read(stream: &mut TcpStream) -> bool {
    let mut buffer = [0; 128];
    if let Ok(bytes_read) = stream.read(&mut buffer) {

        if bytes_read <= 0 {
            println!("Failed to read. Killing");
            return false;
        }

        match buffer[0] {
            3 => {
                println!("Login");
                login::login_user(&buffer);
            }
            2 => {
                println!("Register");
                register::register_user(&buffer);
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