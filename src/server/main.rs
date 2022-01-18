use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

// https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn main() {
    println!("I am the server.");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Connection cannot be established! Exception: {}", e);
            }
        }
    }
    println!("Exit.");
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Got message: '{}'", String::from_utf8_lossy(&buffer[..]));
        stream.write(b"ACK").unwrap();
        stream.flush().unwrap();
    }
}
