use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use std::process;
use std::thread;
use std::time;

// TODO: eliminate unwrap()?
// TODO: when server terminates client gets into infinite loop of printing "Incoming message: 'ACK'"

fn main() {
    println!("I am the client.");
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let stream_clone = stream.try_clone().expect("Cloning stream failed...");
    thread::spawn(|| { handle_incoming_messages(stream); });
    thread::spawn(|| { handle_outgoing_messages(stream_clone); });
    loop {
        // busy loop
    }
}

fn handle_incoming_messages(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        stream.read(&mut buffer).unwrap();
        println!("Incoming message: '{}'", String::from_utf8_lossy(&buffer[..]));
    }
}

fn handle_outgoing_messages(mut stream: TcpStream) {
    let msg = format!("Hello from client {}!", process::id());
    stream.write(msg.as_bytes()).unwrap();
    stream.flush().unwrap();
    thread::sleep(time::Duration::from_millis(1000));

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();
        println!("Outgoing message: '{}'", buffer);
        stream.write(buffer.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
