use std::{io::Write, net, thread};
use std::io::{BufReader};
use std::fs::File;
use rodio::{source::Source, Decoder, OutputStream, Sink};
use serde_json::json;
    
fn handle_conn(conn: &mut net::TcpStream) {
    println!("connected to client, {}", conn.local_addr().unwrap());
    let file = BufReader::new(File::open("04 - In My Blood.flac").unwrap());
    /*
    Audio need 3 things:
        1. buffers/payload.
        2. sample rate
        3. channels
    */
    println!("getting file done, start extracting to info");
    
    let mut source = Decoder::new(file).unwrap();
    let buffer: Vec<i16> = source.by_ref().collect();
    let sample_rate = source.sample_rate();
    let channels = source.channels();

    println!("done getting info, start transferring");
    
    //done gathering info


    let res = json!({
        "payload": buffer,
        "sample_rate": sample_rate,
        "channels":channels,
    });
    conn.write_all(res.to_string().as_bytes());
    conn.flush();
    println!("transferring done, happy listening for client!");

    conn.shutdown(net::Shutdown::Both).unwrap();
}
fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();
    for conn in listener.incoming() {
        let mut conn = conn.unwrap();
        thread::spawn(move || handle_conn(&mut conn));
    }
}
