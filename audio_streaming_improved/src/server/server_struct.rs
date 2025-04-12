//  song_processing;

// use crate::song_processing::something;
mod song_processing;
use rodio::Decoder;
use std::{
    collections::HashMap,
    net::SocketAddr,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    net::UdpSocket,
    select,
    sync::{
        Mutex,
        mpsc::{self, UnboundedReceiver, UnboundedSender, unbounded_channel},
        oneshot,
    },
    task::JoinHandle,
    time,
};

struct ConnectionState {
    sender: UnboundedSender<([u8; 1024], usize)>,
    last_active: Instant,
}

pub struct UdpServer {
    socket: Arc<UdpSocket>,
    connections: Arc<Mutex<HashMap<String, ConnectionState>>>,
    timeout: Duration,
    shutdown_rx: UnboundedReceiver<()>,
    task_handles: Arc<Mutex<Vec<JoinHandle<()>>>>, // Track all tasks
    // file_path: &'static str,
    songs_list: Arc<Vec<String>>,
}

impl UdpServer {
    pub async fn new(
        socket: UdpSocket,
        timeout: Duration,
        songs_list: Vec<String>,
    ) -> (Self, UnboundedSender<()>) {
        let (shutdown_tx, shutdown_rx) = unbounded_channel();
        // let file_path = Path::new(file_path.as_str());
        (
            Self {
                socket: Arc::new(socket),
                connections: Arc::new(Mutex::new(HashMap::new())),
                timeout,
                shutdown_rx,
                task_handles: Arc::new(Mutex::new(Vec::new())),
                // file_path: file_path,
                songs_list: Arc::new(songs_list),
            },
            shutdown_tx,
        )
    }

    pub async fn run(&mut self) {
        // let res = self.songs_list.clone();
        let mut interval = time::interval(Duration::from_secs(10));
        let mut buf = [0u8; 1024];
        loop {
            select! {
                // Shutdown signal
                _ = self.shutdown_rx.recv() => {
                    println!("Shutdown initiated, closing connections...");
                    self.cleanup_all_connections().await;
                    return;
                }

                // Periodic cleanup
                _ = interval.tick() => {
                    println!("timeout check!!!!");
                    self.cleanup_stale_connections().await;
                }

                // Incoming packets
                res = self.socket.recv_from(&mut buf) => {
                    let (len, addr) = match res {
                        Ok(v) => {
                            v
                        },
                        Err(e) => {
                            eprintln!("Receive error: {}", e);
                            continue;
                        }
                    };
                    self.handle_packet(addr, len, buf).await;
                }
            }
        }
    }

    async fn cleanup_all_connections(&mut self) {
        // Close all senders first
        let mut connections = self.connections.lock().await;
        for (_, state) in connections.iter_mut() {
            drop(state.sender.clone()); // Close channel to signal tasks to exit
        }
        connections.clear();

        // Wait for all tasks to complete
        let mut handles = self.task_handles.lock().await;
        while let Some(handle) = handles.pop() {
            if let Err(e) = handle.await {
                eprintln!("Task failed during shutdown: {}", e);
            }
        }
        println!("All connections closed");
    }

    async fn cleanup_stale_connections(&mut self) {
        let now = Instant::now();
        let mut connections = self.connections.lock().await;

        connections.retain(|addr, state| {
            if now.duration_since(state.last_active) > self.timeout {
                println!("Connection from {} timed out", addr);
                false
            } else {
                true
            }
        });
    }

    async fn handle_packet(&mut self, addr: SocketAddr, len: usize, buf: [u8; 1024]) {
        let addr_str = addr.to_string();
        let mut connections = self.connections.lock().await;
        // let mut buf = [0u8; 1024];
        match connections.get_mut(&addr_str) {
            Some(state) => {
                println!("request from client: {:?}", &buf[..len]);
                state.last_active = Instant::now();
                if len == 0 {
                    connections.remove(&addr_str);
                    println!("Connection from {} closed", addr);
                } else if let x = std::str::from_utf8(&buf[..len]) {
                    // in case the user want to quit!!!!
                    match x {
                        Ok(str) => {
                            if str == "quit" {
                                println!("Quit request called!!! Connection from {} closed", addr);
                                connections.remove(&addr_str);
                                println!("Connection from {} closed", addr);
                            }else{
                                state.sender.send((buf, len)).unwrap();
                            }
                        }
                        Err(e) => {
                            state.sender.send((buf, len)).unwrap();
                        }
                    }
                } else {
                    state.sender.send((buf, len)).unwrap();
                }
            }
            None => {
                let (tx, rx) = unbounded_channel();
                let socket = self.socket.clone();
                let timeout = self.timeout;

                let task_handle = tokio::spawn(Self::processing_task(
                    socket,
                    addr,
                    rx,
                    timeout,
                    self.songs_list.clone(),
                ));

                // Store the task handle
                self.task_handles.lock().await.push(task_handle);
                tx.send((buf, len)).unwrap();
                connections.insert(
                    addr_str.clone(),
                    ConnectionState {
                        sender: tx,
                        last_active: Instant::now(),
                    },
                );
            }
        }
    }

    async fn processing_task(
        socket: Arc<UdpSocket>,
        addr: SocketAddr,
        mut rx: UnboundedReceiver<([u8; 1024], usize)>,
        timeout: Duration,
        // list_of_song:
        songs_list: Arc<Vec<String>>,
    ) {
        println!("New connection from {}", addr);
        //load the list first.....
        // println!("{:?}", songs_list.len());
        // for i in songs_list.as_ref() {
        //     socket.send_to(i.as_bytes(), addr).await;
        //     // socket.recv_buf(buf)
        // }
        // socket.send_to("done!!!!".as_bytes(), addr);
        // //

        let socket_clone = socket.clone();
        let songs_list_clone = songs_list.clone();
        let (mut song_tx, mut song_rx) = mpsc::channel(100);
        tokio::spawn(song_processing::song_processing(
            socket_clone,
            song_rx,
            songs_list_clone,
            addr,
        ));
        loop {
            select! {
                Some((buf, len)) = rx.recv() => {
                    //quit request!!!
                    // if let xstd::str::from_utf8(&buf[..len]) == "quit"{
                    //     println!("Quit requested at first!!!!: {}", addr);
                    //     break;
                    // }
                    // entry request!!!!
                    if len==0{
                        println!("{:?}",songs_list.len());
                    for i in songs_list.as_ref(){
                        socket.send_to(i.as_bytes(), addr).await;
                        // socket.recv_buf(buf)
                    }
                        socket.send_to("".as_bytes(), addr).await;

                    }
                    else{
                        song_tx.send((buf,len)).await;


                        // println!("something is fixing");
                        // the cause of overhead!!!!

                        // need for the independence!!!!
                        // Self::song_processing(socket.clone(), buf,len, songs_list.clone(),addr).await;
                    }

                }

                // _ = time::sleep(timeout) => {
                //     println!("Connection to {} timed out", addr);
                //     return;
                // }
                else=>{
                    // println!("Done: {}", addr);
                    // drop(song_tx);

                    break;
                }
            }
            // drop(song_tx);
        }
        println!("Done: {}", addr);
    }
}
