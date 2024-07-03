use std::sync::Arc;

use async_channel::Sender;
use tokio;

use tokio::sync::broadcast;
#[derive(Debug, Clone)]
struct Message {
    id: u8,
    msg: String,
}

async fn sending(mut sender: broadcast::Sender<Message>, id: u8) {
    for i in 0..100 {
        let m = Message {
            id: id,
            msg: format!("Send: {}, id {}, value {}", "hello", id, i),
        };
        loop {
            let x = sender.send(m.clone());
             match x {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    println!("problem...");
                    continue;
                }
            }
        }
    }
    drop(sender);
}
async fn receiving(mut recv: broadcast::Receiver<Message>, id: u8) {
    // while let Ok(msg) = recv.recv().await {
    //     if (msg.id != id) {
    //         println!("Received message: \"{}\" at id {}", msg.msg, id);
    //     }
    // }
    let mut i = 0;
    while let x = recv.recv().await {
        match x {
            Ok(msg) => {
                if (msg.id != id) {
                    println!("Received message: \"{}\" at id {}", msg.msg, id);
                    i += 1;
                }
            }
            Err(_) => {
                println!("missed!!!!");
                println!("{i:?}");
                return;
            }
        }
    }
    println!("{i:?}");
}
async fn crossbeaming(
    mut sender: broadcast::Sender<Message>,
    mut recv: broadcast::Receiver<Message>,
    id: u8,
) {
    let t1 = tokio::spawn(sending(sender, id));
    let t2 = tokio::spawn(receiving(recv, id));
    tokio::join!(t1, t2);
}
#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<Message>(130);
    let mutex = Arc::new(tx);
    let tx1 = (*mutex).to_owned();
    let tx2 = (*mutex).to_owned();
    let rx1 = (*mutex).subscribe();
    let rx2 = (*mutex).subscribe();

    let per1 = tokio::spawn(async move {
        crossbeaming(tx1, rx1, 1).await;
    });
    let per2 = tokio::spawn(async move {
        crossbeaming(tx2, rx2, 2).await;
    });
    drop(mutex);
    // let mut inx = vec![];
    // inx.push(per1);
    // inx.push(per2);

    // // let mut outx = vec![];
    // for ins in inx {
    //     // outx.push(ins);
    //     tokio::join!(ins);
    // }
    tokio::join!(per1, per2);
}
// use async_channel::{bounded, Receiver, Sender};
// use tokio;

// async fn producer(mut tx: Sender<u32>, data: &[u32]) {
//     for value in data {
//         tx.send(*value).await.expect("Failed to send data");
//     }
// }

// async fn consumer(mut rx: Receiver<u32>, id: u8) {
//     while let Ok(value) = rx.recv().await {
//         println!("Received value: {}, id {}", value, id);
//     }
// }

// #[tokio::main]
// async fn main() {
//     let (tx, rx) = bounded::<u32>(10); // Channel with a capacity of 10
//     let mut rx1 = rx.clone();
//     let mut rx2 = rx.clone();
//     println!("{}", tx.receiver_count());

//     let data = vec![1, 2, 3, 4, 5];

//     // Spawn a producer task
//     tokio::spawn(async move {
//         producer(tx.clone(), &data).await;
//     });

//     // Share the receiver with multiple consumer tasks (can be spawned in a loop)
//     tokio::spawn(consumer(rx1, 0));

//     tokio::spawn(consumer(rx2, 1));

//     // Wait for all tasks to finish
//     // (Consider using join! or other methods for better control)
//     tokio::task::yield_now().await;
// }
