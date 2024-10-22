use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::env;
use dotenv::dotenv;
use chrono;


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection was closed
                println!("[{}] Client disconnected", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S %:z"));
                break;
            }
            Ok(bytes_read) => {
                let received = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("[{}] Received: {}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S %:z"), received);

                // Echo back the received data
                if let Err(e) = stream.write_all(&buffer[..bytes_read]) {
                    eprintln!("[{}] Failed to send response: {}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S %:z"), e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("[{}] Failed to read from socket: {}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S %:z"), e);
                break;
            }
        }
    }
}

fn build_address() -> String {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());
   
    return format!("{}:{}", host, port);
}

fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("[{}] Start server on {}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S %:z"), build_address());

    let listener = TcpListener::bind(build_address())?;
    println!("[{}] Server listening...", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S %:z"));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("[{}] Connection failed: {}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S %:z"), e);
            }
        }
    }

    Ok(())
}

