// use tokio::net::unix::SocketAddr;

use std::{
    net::SocketAddr,
    string,
    sync::{Arc, Mutex},
    io
};

use tokio::{
    net::UdpSocket,
    sync::mpsc::{
        self, Receiver, Sender, UnboundedReceiver, UnboundedSender, channel, unbounded_channel,
    },
    time::timeout,
};

use rodio::{Decoder, OutputStream, Sink};
fn adding(i: u8) -> u8 {
    if i == 255 {
        return 0;
    } else {
        return i + 1;
    }
}
#[tokio::main]
async fn main() {
    let mut client_socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let mut client_socket = Arc::new(client_socket);
    let remote_addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    client_socket.connect(remote_addr).await.unwrap();

    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
    client_socket.send("".as_bytes()).await;
    let mut list_song = vec![];
    loop {
        let mut buf = vec![];
        client_socket.recv_buf(&mut buf).await;
        // println!("{}",buf.len());
        if buf.len() == 0 {
            break;
        }
        list_song.push(String::from_utf8(buf).unwrap());
    }
    for i in 0..list_song.len(){
        println!("{}: {}", i, list_song[i]);
    }

    println!("which song do you want to play????: ");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    let song = input_string.as_bytes();
    println!("{:?}",song);
    let song = &song[..song.len()-1];
    client_socket.send(&song).await;
    let mut present: u8 = 0;
    let mut size: usize = 0;
    let mut seq = [115, 101, 113, 0];

    // let mut data: Vec<i16> = vec![];
    // let
    let (async_work, sender) = collecting();
    let t1 = tokio::spawn(async_work);
    loop {
        let mut buf = Vec::with_capacity(600);
        client_socket.send(&seq).await;
        if let Err(_) = timeout(
            tokio::time::Duration::from_secs(60),
            client_socket.recv_buf(&mut buf),
        )
        .await
        {
            println!("out of time!!!!");
            drop(sender);
            break;
        }
        println!("{}", buf.len());
        // size += (buf.len() - 1);
        sender.send(buf[..buf.len() - 1].to_vec()).await;
        // let res = rodio::buffer::SamplesBuffer::new(2, 48000, res);
        // s1.lock().unwrap().append(res);
        //         tx.send(res).await;
        if buf[buf.len() - 1] == seq[3] {
            println!("received!!! {}", seq[3]);
            seq[3] = adding(seq[3]);
        }
    }
    // println!("{}",);
    let data = t1.await.unwrap();
    println!("gathering data done, happy playing");
    let s1 = sink.clone();
    let t1 = tokio::spawn(async move{
        let res = rodio::buffer::SamplesBuffer::new(2, 48000, data);
        s1.lock().unwrap().append(res);
    });
    t1.await;
    sink.lock().unwrap().sleep_until_end();

    










    // println!("{}", data.len());
    // println!("{}", data.len());
}

fn collecting() -> (impl Future<Output = Vec<i16>>, Sender<Vec<u8>>) {
    let (buffer_in, mut buffer_out) = channel::<Vec<u8>>(500);
    let some_async = async move {
        let mut data:Vec<i16> =vec![];
        // let mut total: usize = 0;
        while let Some(buf) = buffer_out.recv().await {
            let mut res: Vec<i16> = buf
                .chunks_exact(2)
                .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                .collect();
            println!("len of {}", res.len());
            data.append(&mut res);
            // total += res.len();
        }
        return data;
    };
    (some_async, buffer_in)
}
fn control_instance() -> (impl Future<Output = ()>, UnboundedReceiver<String>) {
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // The future that will process messages
    let future = async move {};

    (future, rx)
}
// fn network_instance(socket: Arc<UdpSocket>, command_receiver: UnboundedReceiver<String>) -> {
//     let (mut data_sender, mut data_receiver)  =
// }
fn sound_instance() {}
