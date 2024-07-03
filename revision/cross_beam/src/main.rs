// #![feature(async_closure)]
// use tokio;
// use tokio::sync::broadcast;
// async fn sender_task(tx: broadcast::Sender<String>, id: u8, h: &str) {
//     // tokio::spawn(async {
//     //     println!("future");
//     // });
//     for i in 0..10 {
//         tx.send(format!("Send: {}, id {}, value {}", h, id, i))
//             .unwrap();
//         // println!("Sent message: {}", i);
//     }
//     drop(tx);
// }

// async fn receiver_task(mut receiver: broadcast::Receiver<String>, id: u8) {
//     // println!("id {} is working",id);
//     while let Ok(msg) = receiver.recv().await {
//         println!("Received message: {} at id {}", msg, id);
//     }
// }

// #[tokio::main]
// async fn main() {
//     // Create a broadcast channel.
//     let (tx, rx) = broadcast::channel::<String>(16);

//     let tx1 = tx.clone();
//     let tx2 = tx.clone();
//     // let tx3 = tx.clone();
//     let sending = async move |tx: broadcast::Sender<String>| {
//         println!("the bigger task");
//         for i in 0..7 {
//             let tx1 = tx.clone();
//             tokio::spawn(async move {
//                 // println!("{i:?}");
//                 sender_task(tx1, i, "hello").await;
//             }).await;
//         }
//     };
//     let receiving = async move |mut receiver: broadcast::Receiver<String>| {
//         for i in 0..10 {
//             println!("Receiving: {i:?}");
//             let rx1 = receiver.resubscribe();
//             tokio::spawn(receiver_task(rx1, i));
//         }
//     };
//     // Spawn the sender task.
//     let t1 = tokio::spawn(sending(tx1));
//     let t2 = tokio::spawn(sender_task(tx2, 8, "hello"));

//     drop(tx);
//     // Create a receiver for the channel.
//     // let receiver = rx.resubscribe();

//     // Spawn the receiver task.
//     let t3 = tokio::spawn(receiving(rx));
//     // let rx1 = rx.resubscribe();
//     // let rx2 = rx.resubscribe();
//     // let rx3 = rx.resubscribe();
//     // let t3 = tokio::spawn(receiver_task(rx1));
//     // let t4 = tokio::spawn(receiver_task(rx2));
//     // let t5 = tokio::spawn(receiver_task(rx3));

//     tokio::join!(t1, t2, t3);
// }
#![feature(async_closure)]
use std::fmt::Debug;

use tokio;
use tokio::sync::broadcast;

async fn sender_task(tx: broadcast::Sender<msg>, id: u8, h: &str) {
    for i in 0..10 {
        tx.send(msg {
            id: id,
            mess: format!("Send: {}, id {}, value {}", h, id, i),
        })
        .unwrap();
    }
    // drop(tx);
    println!("{id:?} is done");
}
async fn sending(tx: broadcast::Sender<msg>, max_id: u8, h: &'static str) {
    let mut list_task = vec![];
    for i in 0..max_id {
        let tx_clone = tx.clone();
        let t = tokio::spawn(async move {
            sender_task(tx_clone, i, h).await;
        });
        list_task.push(t);
    }
    for task in list_task {
        tokio::join!(task).0;
    }
    drop(tx);
}
async fn receiver_task(mut receiver: broadcast::Receiver<msg>, id: u8) {
    let mut i = 0;
    while let Ok(msg) = receiver.recv().await {
        if (msg.id == id) {
            println!("Received message: \"{}\" at id {}", msg.mess, id);
            i += 1;
        }
    }
    println!("{i:?}");
}
#[derive(Clone, Debug)]
struct msg {
    id: u8,
    mess: String,
}

#[tokio::main]

async fn main() {
    // Create a broadcast channel.
    let (tx, rx) = broadcast::channel::<msg>(150);
    let tx_clone = tx.clone();
    let rx1 = tx_clone.subscribe();
    let rx2 = tx_clone.subscribe();
    let t1 = tokio::spawn(sending(tx_clone, 10, "hello!!"));
    drop(tx);
    let t2 = tokio::spawn(receiver_task(rx1, 0));
    let t3: tokio::task::JoinHandle<()> = tokio::spawn(receiver_task(rx2, 1));
    

    tokio::join!(t1, t2, t3);
    
    // Spawn the sender tasks.
    // let tasks = (0..8).into_iter().map(move |i| {
    //     let tx_clone = tx_clone.clone();
    //     sender_task(tx_clone, i, "hello");
    // })

    // Create and spawn a single receiver task.
    // let mut receiver = tx.subscribe();
    // let receiving_task = tokio::spawn(async move {
    //     receiver_task(receiver, 0).await;
    // });

    // // Wait for all tasks to finish.
    // let total = for task in tasks.into_iter() {
    //     tokio::spawn(async move{
    //         task;
    //     });
    // };
    // tokio::join!(tokio::spawn(async move{total;}),receiving_task);
    // receiving_task.await;
}
