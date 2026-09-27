use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::os::windows::io::AsRawSocket;
use std::thread;

// netstat -ano | findstr <PID>
// (Get-Process -Id <PID>).Threads.Count

fn main() {
    let pid = std::process::id(); // get server pid for connection monitoring
    println!("The current process ID is: {pid}");

    let _socket_creation_res = create_server_socket();
}

fn create_server_socket() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Unable to connect client... {}", e)
            }
        }
    }
    Ok(())
} 

fn handle_client(mut stream: TcpStream) {

    let raw_socket: std::os::windows::io::RawSocket = stream.as_raw_socket();
    println!("User with id: {} has connected to the server.", raw_socket);

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