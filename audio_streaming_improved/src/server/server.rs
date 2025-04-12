// use std::net::UdpSocket;
// mod serverimpl;
// use serverimpl::request_looping;
mod server_struct;
use server_struct::UdpServer;
use std::io::Read;
use std::time::Duration;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
// use rodio::source::SamplesConverter;
use std::{clone, fs, io};
use tokio::net::UdpSocket;
use tokio::select;
use tokio::signal;
use tokio::sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender};
use tokio::time;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    //60s to rescan and check timeout.....
    let timeout = Duration::from_secs(60);
    //create the list of songs.....
    let mut songs_list: Vec<String> = Vec::new();

 
    let paths = fs::read_dir("./list_songs").unwrap();

    for path in paths {
        let res = format!("{}", path.unwrap().path().display());
        songs_list.push(res);
    }
    // for res in songs_list {
    //     println!("{}", res);
    // }

    
     // Create server with shutdown channel
    let (mut server, shutdown_tx) = UdpServer::new(socket, timeout, songs_list).await;

    // Run server in background
    let server_handle = tokio::spawn(async move {
        server.run().await;
    });

    // Setup signal handler (Ctrl+C)
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("\nCtrl+C received, shutting down gracefully...");
            shutdown_tx.send(()).unwrap();
        }
        // _ = tokio::time::sleep(Duration::from_secs(60)) => {
        //     println!("Demo timeout reached, shutting down...");
        //     shutdown_tx.send(()).unwrap();
        // }
    }

    // Wait for server to complete shutdown
    server_handle.await?;
    println!("Server shutdown complete");
    
    Ok(())
}

// done at server
