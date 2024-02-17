use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8;1024];
    let mut fs = File::open("test.txt").unwrap();
    loop{
        match fs.read(&mut buffer) {
            Ok(size) =>{
                match size{
                    0 =>{
                        println!("{}","Done!!!!!");
                        break;
                    },
                    _=>{
                        stream.write(&mut buffer[0..size]);
                    },
                }
            },
            _=>{
                panic!();
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
