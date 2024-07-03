use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc::{Receiver, Sender};

use rodio::{Decoder, OutputStream, Sink};
use tokio;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
#[tokio::main]
async fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink = Sink::try_new(&stream_handle).unwrap();
    let sample_rate = 44100;
    let channels: u16 = 2;
    let (sender, mut recv) = mpsc::unbounded_channel::<Vec<i16>>();
    let sink = Arc::new(Mutex::new(sink));
    let t1 = tokio::task::spawn(async move {
        sending(sender).await;
    });

    // tokio::task::spaw

    let t2 = tokio::task::spawn_blocking(move || {
        // thread::spawn(move || {
        //     sink.sleep_until_end();
        // }).join();

        let sink_clone = Arc::clone(&sink);
        thread::spawn(move || {
            loop {
                let x = recv.blocking_recv();
                match x {
                    Some(k) => {
                        // println!("asdfsdf");
                        let new_source =
                            rodio::buffer::SamplesBuffer::new(channels, sample_rate, k);
                        let mut sink = sink_clone.lock().unwrap();
                        sink.append(new_source);
                        // sink.sleep_until_end();
                    }
                    None => {
                        println!("nothing");
                        break;
                    }
                }
            }
        })
        .join();
        sink.lock().unwrap().sleep_until_end();

        // receiver(recv);
    });
    tokio::join!(t1, t2);
    // sink.sleep_until_end();
}
async fn sending(sender: UnboundedSender<Vec<i16>>) {
    let file = BufReader::new(File::open("rightforyou.mp3").unwrap());

    let mut source = Decoder::new(file).unwrap();
    let buffer: Vec<i16> = source.by_ref().collect();
    
    tokio::spawn(async move {
        let mut iter = buffer.chunks(100);
        loop {
            let x = iter.next();
            match x {
                Some(k) => {
                    println!("sadfsfsdf");
                    let _ = sender.send(k.to_owned());
                }
                None => {
                    println!("nothing");
                    break;
                }
            }
        }
    });

    // sender.closed();

    // for i in 0..7 {
    //     let _ = sender.send(i);
    // }};
}
async fn receiver(mut receiver: UnboundedReceiver<Vec<i16>>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink = Sink::try_new(&stream_handle).unwrap();
    let sample_rate = 44100;
    let channels: u16 = 2;

    loop {
        let x = receiver.recv().await;
        match x {
            Some(k) => {
                println!("asdfwaerewre");
                // let new_source = rodio::buffer::SamplesBuffer::new(channels, sample_rate, k);
                // sink.append(new_source);
                // sink.sleep_until_end();
                // sink
            }
            None => {
                println!("nothing");
                break;
            }
        }
    }
}

// async fn sending(sender: UnboundedSender<i32>) {
//     async{
//     for i in 0..7 {
//         let _ = sender.send(i);
//     }};
//     sender.closed();
// }
// async fn receiver(receiver: &mut UnboundedReceiver<i32>) {
//     loop {
//         let x = receiver.recv().await;
//         match x {
//             Some(k) => println!("{k:?}"),
//             None => {
//                 println!("nothing");
//                 break;
//             }
//         }
//     }
// }
