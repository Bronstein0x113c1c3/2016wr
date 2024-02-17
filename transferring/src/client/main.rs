use std::{fs::File, io::{stdin, Read, Write}, net::{TcpListener,TcpStream}};
fn main(){
    let addr = "127.0.0.1";
    let port = 8080;
    let mut stream = TcpStream::connect(format!("{}:{}",addr,port)).expect("cannot connect....");
    let mut file = File::create("test2.txt").unwrap();
    let mut buffer = [0u8;1024];
    loop{
        match stream.read(&mut buffer) {
            Ok(size) =>{
                if size==0 {
                    drop(file);
                    break;         
                }
                file.write(&buffer[0..size]);
            },
            _=>{
               drop(file);
               eprintln!("cannot receive chunks...");
               return
            }            
        }
    }
    println!("{}","Writing done!!!");
    return;
}