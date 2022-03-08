use std::collections::LinkedList;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

// https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// see: https://stackoverflow.com/questions/60219160/how-to-store-tcpstream-inside-a-hashmap

// TODO: eliminate unwrap()?

fn main() {
    println!("I am the server.");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let list_of_streams = Arc::new(RwLock::new(LinkedList::new()));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                // TODO: use shared stream?
                let stream_clone1 = stream.try_clone().expect("Cloning stream failed...");
                let stream_clone2 = stream.try_clone().expect("Cloning stream failed...");
                let stream_clone3 = stream.try_clone().expect("Cloning stream failed...");

                let list_of_streams_clone1 = list_of_streams.clone();
                let list_of_streams_clone3 = list_of_streams.clone();
                thread::spawn(move || {
                    register_connection(stream_clone1, list_of_streams_clone1);
                    handle_connection(stream_clone2/*, list_of_streams*/);
                    deregister_connection(stream_clone3, list_of_streams_clone3);
                });
            }
            Err(e) => {
                println!("Connection cannot be established! Exception: {}", e);
            }
        }
        // TODO: list/print list_of_streams
    }
    println!("Exit.");
}

fn register_connection(stream: TcpStream, list_of_streams: Arc<RwLock<LinkedList<TcpStream>>>) {
    println!("Registering connection: {:?}", stream);
    list_of_streams.write().unwrap().push_back(stream);
}

fn handle_connection(mut stream: TcpStream/*, list_of_streams: Arc<RwLock<LinkedList<TcpStream>>>*/) {
    loop {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Got message: '{}' -- from {:?}", String::from_utf8_lossy(&buffer[..]), stream);
        match stream.write(b"ACK") {
            Ok(..) => {
                stream.flush().unwrap();
            }
            Err(..) => {
                println!("Failed to write to connection: {:?}", stream);
                return;
            }
        }
    }
}

fn deregister_connection(stream: TcpStream, list_of_streams: Arc<RwLock<LinkedList<TcpStream>>>) {
    println!("Unregistering connection: {:?}", stream);
    // TODO: !!!
    list_of_streams.write().unwrap().pop_front();
}
