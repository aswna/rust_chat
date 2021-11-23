use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;

// https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn main() {
    println!("I am the server.");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                handle_connection(stream);
            }
            Err(e) => {
                println!("Connection cannot be established! Exception: {}", e);
            }
        }
    }
    println!("Exit.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
