// use tokio::net::unix::SocketAddr;

use std::{net::SocketAddr, string};

#[tokio::main]
async fn main() {
    let mut client_socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap();

    let remote_addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    client_socket.connect(remote_addr).await.unwrap();
    let mut buf  = [0u8;1024].to_vec();
    for i in 0..5 {
        client_socket.send("".as_bytes()).await.unwrap();
        let len = client_socket.recv(&mut buf).await.unwrap();
        println!("{:?}",&buf);
        match String::from_utf8(buf[..len].to_vec()) {
            Ok(s) => println!("Received valid UTF-8: {}", s),
            Err(e) => {
                // If the data isn't valid UTF-8, use lossy conversion
                let s = String::from_utf8_lossy(&buf[..len]);
                println!("Received invalid UTF-8: {}", s);
            }
        }
    }
}
