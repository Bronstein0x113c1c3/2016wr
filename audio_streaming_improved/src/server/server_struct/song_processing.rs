use std::{net::SocketAddr, process::Output, sync::Arc, time::Duration};

use rodio::Decoder;
use tokio::{
    net::UdpSocket,
    select,
    sync::mpsc::{self, Receiver, UnboundedReceiver, UnboundedSender, unbounded_channel},
    time::sleep,
};
// use tokio_util::sync::CancellationToken;

pub async fn song_processing(
    socket: Arc<UdpSocket>,
    mut song_rx: Receiver<([u8; 1024], usize)>,
    songs_list: Arc<Vec<String>>,
    addr: SocketAddr,
) {
    let (control, mut song_id_rx, mut seq_rx, _) = control(song_rx);
    let (mut chunk_tx, mut chunk_rx) = unbounded_channel();
    // seq_rx.recv_many(buffer, limit)
    tokio::spawn(song_finding(song_id_rx, songs_list, chunk_tx));
    tokio::spawn(control);

    // let mut id_check = 0usize;
    // let mut seq_check = 0u8;
    //for caching, if packet loss
    let mut cache: Vec<u8> = Vec::with_capacity(260);
    let mut user_req: u8 = 0; // -> request chunk
    let mut present: u8 = 0; // -> newest chunk possible

    /*
    packet loss occur when user_req != present (user_req != present)

    if user_req == present -> user request the newest chunk possible, increase the present, save the cache for later use
    if user_req != present -> packet loss occurred -> request the cache
     */
    loop {
        // let seq = seq_rx.recv().await;
        match seq_rx.recv().await {
            Some(index) => {
                user_req = index;
                println!("user_req: {}", user_req);
            }
            None => {
                break;
            }
        }
        // let mut received_chunk: Vec<u8> = Vec::with_capacity(260);
        if user_req == present {
            //request the newest chunk
            println!("requesting newest chunk!!");
            match chunk_rx.recv().await {
                Some(chunk) => {
                    // let check = chunk.clone();
                    // if &check == "done".as_bytes() {
                    //     socket.send_to("done!!!".as_bytes(), addr).await;
                    // } else {
                        cache = chunk;
                    // }
                }
                None => {
                    // socket.send_to("done!!!".as_bytes(), addr).await;
                    // println!("done request sent!!!!");
                    break;
                }
            }
            //update the cache, also update the cache
            //send the cache
            // socket.send_to(&cache, addr).await;
            let test_buf = [user_req];
            // socket.send_to(&test_buf, addr).await;
            // println!("{}",cache.len());
            socket.send_to(&cache, addr).await;
            println!("sending {}", cache[cache.len() - 1]);
            //update the present
            present = add(present);
        } else {
            println!("requesting cache chunk!!");
            let test_buf = [user_req];
            // println!("{}",cache.len());
            socket.send_to(&cache, addr).await;
            println!("sending {}", cache[cache.len() - 1]);
            //only send the cache -> packet loss detected!!!
        }
        // last byte of chunk == present -> send
    }
    // println!("nothing to see");
    // let (mut song_tx, mut song_rx) = mpsc::unbounded_channel();

    // println!("")
    /*
    plan:
    if a song -> spawn the song tasks, don't wait...

    if a chunk ->
     */

    // loop {}
}
fn adding(i: u8) -> u8 {
    if i == 255 {
        return 0;
    } else {
        return i + 1;
    }
}
pub async fn song_finding(
    mut find_song_rx: UnboundedReceiver<usize>,
    songs_list: Arc<Vec<String>>,
    chunk_tx: UnboundedSender<Vec<u8>>,
) {
    while let Some(index) = find_song_rx.recv().await {
        println!("song index called!!! {}", index);
        if index >= songs_list.len() {
            // socket.send_to("".as_bytes(), addr).await;
            chunk_tx.send(vec![]);
            continue;
        }
        let song_name = &songs_list[index];
        let file = std::io::BufReader::new(std::fs::File::open(song_name.as_str()).unwrap());
        let mut source = Decoder::new(file).unwrap();

        // source.
        let buffer: Vec<i16> = source.by_ref().collect();
        println!("len of song: {}",buffer.len());
        let bytes: Vec<u8> = buffer
            .iter()
            .flat_map(|&num| num.to_be_bytes().to_vec()) // little-endian
            // .flat_map(|&num| num.to_be_bytes().to_vec()) // big-endian
            .collect();
        let mut iter = bytes.chunks(256); // 1mb per chunk....
        let mut seq = 0;
        while let Some(res) = iter.next() {
            // let mut salt = "music_chunk".as_bytes().to_vec();
            let mut res = res.to_owned();
            res.push(seq);
            // res.append(&mut salt);
            chunk_tx.send(res);
            seq = add(seq); // i = i+1;
        }
        // let mut done = "done".as_bytes().to_vec();
        // done.push(seq);
        // chunk_tx.send(done);
        // drop(tx);
    }
    drop(chunk_tx);
}
fn add(mut i: u8) -> u8 {
    if i == 255 {
        return 0;
    } else {
        return i + 1;
    }
}

pub fn control(
    mut song_rx: Receiver<([u8; 1024], usize)>,
) -> (
    impl Future<Output = ()>,
    UnboundedReceiver<usize>,
    UnboundedReceiver<u8>,
    UnboundedReceiver<()>,
) {
    let (mut find_song_tx, mut find_song_rx) = mpsc::unbounded_channel(); //find song command
    let (mut seq_tx, mut seq_rx) = mpsc::unbounded_channel(); //sequence number
    let (mut fail_alert_tx, mut fail_alert_rx) = mpsc::unbounded_channel(); //wrong command

    // let (mut find_song_tx, mut find_song_rx) = mpsc::unbounded_channel(); let token = CancellationToken::new();
    // let token = CancellationToken::new();
    // let cancellation = token.clone();
    let operate = async move {
        while let Some(res) = song_rx.recv().await {
            let buf = res.0;
            let len = res.1;
            println!("what's received!!!: {:?}", &buf[..len]);
            //check if the sequence packet
            let seq_check = std::str::from_utf8(&buf[..3]).unwrap();
            if seq_check == "seq" {
                println!("sequence packet detected!!!!");
                // println!("{:?}", &buf[]);
                let seq_num = buf[len - 1];
                println!("{}", seq_num);
                seq_tx.send(seq_num);
                continue;
            }
            match std::str::from_utf8(&buf[..len]) {
                Err(_) => {
                    fail_alert_tx.send(());
                }
                Ok(res) => match res.parse::<usize>() {
                    Ok(res) => {
                        find_song_tx.send(res);
                    }
                    Err(_) => {
                        fail_alert_tx.send(());
                    }
                },
            }
        }
        drop(find_song_tx);
        drop(seq_tx);
        drop(fail_alert_tx);
        // println!("control task is done!!!!");
    };
    (operate, find_song_rx, seq_rx, fail_alert_rx)
}
/*
older code for reference


    // let str = std::str::from_utf8(&buf[..len]).unwrap();
    // println!("str: {}", str);
    // let parse_res = str.parse::<usize>();
    // let mut index: usize = 10000000000000;
    // match parse_res {
    //     Ok(res) => {
    //         index = res;
    //     }
    //     Err(e) => {
    //         println!("problem in parsing: {}", e);
    //     }
    // }

    // println!("index: {}", index);
    // if index >= songs_list.len() || index < 0 {
    //     socket.send_to("".as_bytes(), addr).await;
    //     return;
    // } else {
    //     match songs_list.get(index) {
    //         // println!("song is started to be processed!!!!");
    //         Some(str) => {
    //             println!("song name: {}", str);
    //             let new_str = str.to_owned();
    //             let (mut tx, mut rx) = mpsc::unbounded_channel();
    //             let t = tokio::spawn(async move {
    //                 let file =
    //                     std::io::BufReader::new(std::fs::File::open(new_str.as_str()).unwrap());
    //                 let mut source = Decoder::new(file).unwrap();

    //                 // source.
    //                 let buffer: Vec<i16> = source.by_ref().collect();
    //                 let bytes: Vec<u8> = buffer
    //                     .iter()
    //                     .flat_map(|&num| num.to_be_bytes().to_vec()) // little-endian
    //                     // .flat_map(|&num| num.to_be_bytes().to_vec()) // big-endian
    //                     .collect();
    //                 let mut iter = bytes.chunks(256); // 1mb per chunk....
    //                 let mut seq = 0;
    //                 while let Some(res) = iter.next() {
    //                     let mut salt = "music_chunk".as_bytes().to_vec();
    //                     let mut res = res.to_owned();
    //                     res.push(seq);
    //                     res.append(&mut salt);
    //                     tx.send(res);
    //                     seq += 1;
    //                 }
    //                 drop(tx);
    //             });
    //             //handling packet loss here!!!!
    //             while let Some(res) = rx.recv().await {
    //                 println!("receiving chunk...");
    //                 // tokio::time::sleep(Duration::from_millis(10)).await;
    //                 //send a packet 10 times to increase the chance to be received!!!!
    //                 // for _ in 0..20 {
    //                 socket.send_to(&res, addr).await.unwrap();
    //                 // }
    //             }
    //             tokio::time::sleep(Duration::from_millis(10)).await;

    //             println!("done receiving chunk...");
    //             socket.send_to("done".as_bytes(), addr).await.unwrap();
    //             println!("done receiving chunk...");
    //             // t.await;
    //             // let file = std::io::BufReader::new(std::fs::File::open(str.as_str()).unwrap());
    //             // let mut source = Decoder::new(file).unwrap();

    //             // // source.
    //             // let buffer: Vec<i16> = source.by_ref().collect();
    //             // let bytes: Vec<u8> = buffer
    //             //     .iter()
    //             //     .flat_map(|&num| num.to_le_bytes().to_vec()) // little-endian
    //             //     // .flat_map(|&num| num.to_be_bytes().to_vec()) // big-endian
    //             //     .collect();

    //             // // println!("Bytes: {:?}", bytes);
    //             // println!("buffer {}, bytes {}", buffer.len(), bytes.len());
    //             // socket.send_to(buffer., addr)
    //         }
    //         None => {
    //             socket.send_to("".as_bytes(), addr).await;
    //             return;
    //         }
    //     }
    // }

*/
