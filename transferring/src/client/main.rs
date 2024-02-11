use std::{fs::File, io::{stdin, Read, Write}, net::{TcpListener,TcpStream}};
fn main(){
    let addr = "127.0.0.1";
    let port = 8080;
    let mut stream = TcpStream::connect(format!("{}:{}",addr,port)).expect("cannot connect....");
    let mut file = File::open("test.txt").unwrap();
    // file.chain(stream)
    stream.write_all(file.)
    
}