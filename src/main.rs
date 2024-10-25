mod commands;

use chrono;
use commands::get_command;
use dotenv::dotenv;
use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod, SslVerifyMode};
use std::env;
use std::pin::Pin;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio_openssl::SslStream;

async fn handle_client(stream: SslStream<tokio::net::TcpStream>) {
    let (reader, mut writer) = io::split(stream);
    let mut reader = BufReader::new(reader);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!(
                    "[{}] Client disconnected",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z")
                );
                break;
            }
            Ok(_) => {
                let received = line.trim().to_string();
                println!(
                    "[{}] Received: {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                    received
                );

                // Look up the command in the registry
                if let Some(cmd) = get_command(&received) {
                    cmd.execute();
                    let response = format!("Command '{}' executed successfully\n", received);
                    if let Err(e) = writer.write_all(response.as_bytes()).await {
                        eprintln!(
                            "[{}] Failed to send response: {}",
                            chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                            e
                        );
                        break;
                    }
                } else {
                    eprintln!(
                        "[{}] Unknown command received: {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                        received
                    );
                    let response = format!("Unknown command '{}'\n", received);
                    if let Err(e) = writer.write_all(response.as_bytes()).await {
                        eprintln!(
                            "[{}] Failed to send response: {}",
                            chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                            e
                        );
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "[{}] Failed to read from socket: {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                    e
                );
                break;
            }
        }
    }
}

fn build_address() -> String {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    format!("{}:{}", host, port)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!(
        "[{}] Start server on {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
        build_address()
    );

    // Set up OpenSSL acceptor
    let mut acceptor_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    acceptor_builder.set_private_key_file("key.pem", SslFiletype::PEM)?;
    acceptor_builder.set_certificate_chain_file("cert.pem")?;
    acceptor_builder.set_verify(SslVerifyMode::NONE);
    let acceptor = acceptor_builder.build();

    // Start the listener
    let listener = TcpListener::bind(build_address()).await?;
    println!(
        "[{}] Server listening...",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z")
    );

    loop {
        let (stream, _addr) = listener.accept().await?;
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let ssl = match Ssl::new(acceptor.context()) {
                Ok(ssl) => ssl,
                Err(e) => {
                    eprintln!(
                        "[{}] Failed to create SSL object: {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                        e
                    );
                    return;
                }
            };

            let mut ssl_stream = match SslStream::new(ssl, stream) {
                Ok(ssl_stream) => ssl_stream,
                Err(e) => {
                    eprintln!(
                        "[{}] Failed to create SSL stream: {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                        e
                    );
                    return;
                }
            };

            if let Err(e) = Pin::new(&mut ssl_stream).accept().await {
                eprintln!(
                    "[{}] Failed to accept SSL connection: {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                    e
                );
                return;
            }

            handle_client(ssl_stream).await;
        });
    }
}
