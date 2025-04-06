// use std::net::UdpSocket;

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::net::UdpSocket;

#[tokio::main]
async fn main(){
    let mut socket = tokio::net::UdpSocket::bind("0.0.0.0:8080").await.unwrap();
    let mut socket= Arc::new(socket);
    let mut mapping:HashMap<&SocketAddr, tokio::sync::oneshot::Sender<[u8;1024]>> = HashMap::with_capacity(100);
    let mut buf =[0u8;1024]; //  it's just a spark!!!!, 1mb to spark
    let (mut conn_tx, mut conn_rx) = tokio::sync::mpsc::unbounded_channel();
    loop {
        // let s1 = socket.clone();
        // mapping.entry(key)
        // let s1 = socket.clone();
        let (len, addr) = socket.as_ref().recv_from(&mut buf).await.unwrap();
        // println!("{:?}, {:?}", addr, len);
        
        match mapping.get(&addr){
            None =>{
                let socket = socket.clone();
                let conn_tx = conn_tx.clone();
                conn_tx.send(tokio::spawn(processing(socket, addr)));
            },
            Some(sender)=>{
                // prefetch_write_data(data, locality);
                println!("nothing!!!!!");
            }
        }
        // addr.eq(other)'
        // if mapping.get(k)
        // tokio::spawn(processing(socket.clone(), addr)); 
        // let s = socket.clone()
    }
    // let mut buf = [0u8;32];
    // let (len, mut addr) = socket.recv_from(&mut buf).await.unwrap();
    
    // socket.send(buf)
}
// async fn transmitting()
async fn processing(socket: Arc<UdpSocket>, addr: SocketAddr){
    // socket.recv_from(buf)
    socket.send_to("bonjour!!!!".as_bytes(), addr).await;
}