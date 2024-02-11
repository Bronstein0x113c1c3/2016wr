use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    loop {
        match stream.read_to_end(&mut buffer) {
            Ok(_) => {
                if buffer.is_empty() { return; } // connection closed
                // println!("{:?}",buffer);
                stream.write_all(&buffer).expect("Could not write back to the socket");

                buffer.clear();
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Unable to connect: {}", e);
            }
        }
    }
}



//create the listener
/*
   for every new connection{
       -spawn a thread for it and waiting it to be end.
       -each thread running a same function called process.....
   }
   func process(){
       -receiving the chunk.....
       -return the result..... to the client
       -test using telnet....
   }
*/
