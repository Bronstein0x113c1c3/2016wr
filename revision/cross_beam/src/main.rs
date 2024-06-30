// use tokio::sync::broadcast;

// #[tokio::main]
// async fn main() {
//     // Create a broadcast channel.
//     let (tx, rx) = broadcast::channel::<u32>(16); // Channel capacity of 16

//     // Spawn two tasks that will send messages on the channel.
//     tokio::spawn(async move {
//         for i in 0..5 {
//             tx.send(i).unwrap();
//             println!("Sent message: {}", i);
//         }
//     });

//     // tokio::spawn(async move {
//     //     for i in 5..10 {
//     //         tx.send(i).unwrap();
//     //         println!("Sent message: {}", i);
//     //     }
//     // });

//     // Create a receiver for the channel.
//     let mut receiver = rx.resubscribe();

//     // Loop to receive messages from the channel.
//     while let Ok(msg) = receiver.recv().await {
//         println!("Received message: {}", msg);
//     }
// }

use tokio::sync::broadcast;
use tokio;
async fn sender_task(tx: broadcast::Sender<u32>) {
    tokio::spawn(async {
        println!("future");
    });
    for i in 0..10 {
        tx.send(i).unwrap();
        println!("Sent message: {}", i);
    }
    drop(tx);
}

async fn receiver_task(mut receiver: broadcast::Receiver<u32>) {
    while let Ok(msg) = receiver.recv().await {
        println!("Received message: {}", msg);
    }
}


#[tokio::main]
async fn main() {
    // Create a broadcast channel.
    let (tx, rx) = broadcast::channel::<u32>(100);
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    // let tx3 = tx.clone();
    // let tx4 = tx.clone();
    // let tx5 = tx.clone();
    // let tx6 = tx.clone();

    // Spawn the sender task.
    let t1=tokio::spawn(
       
           sender_task(tx1)
           // sender_task(tx2).await;

       
        // sender_task(tx2).await; 
        // sender_task(tx3).await; 
        // sender_task(tx4).await; 
        // sender_task(tx5).await; 
        // sender_task(tx6).await; 
    );
    let t2=tokio::spawn(
        
            sender_task(tx2)
            // sender_task(tx2).await;

        
        // sender_task(tx2).await;
        // sender_task(tx3).await;
        // sender_task(tx4).await;
        // sender_task(tx5).await;
        // sender_task(tx6).await;
    );

    drop(tx);
    // Create a receiver for the channel.
    // let receiver = rx.resubscribe();

    // Spawn the receiver task.
    let t3 = tokio::spawn(receiver_task(rx));
    tokio::join!(t1,t2,t3);
}