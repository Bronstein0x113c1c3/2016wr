use tokio;

#[derive(Debug, Clone)]
struct Message {
    id: u8,
    msg: String,
}

async fn sending(mut sender: loole::Sender<Message>, id: u8) {
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
                    // continue;
                    return;
                }
            }
        }
    }
    // drop(sender);
    sender.close();
}
async fn receiving(mut recv: loole::Receiver<Message>, id: u8) {
    // while let Ok(msg) = recv.recv().await {
    //     if (msg.id != id) {
    //         println!("Received message: \"{}\" at id {}", msg.msg, id);
    //     }
    // }
    let mut i = 0;
    while let x = recv.recv() {
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
                // continue;
                return;
            }
        }
    }
    println!("{i:?}");
}
async fn crossbeaming(
    mut sender: loole::Sender<Message>,
    mut recv: loole::Receiver<Message>,
    id: u8,
) {
    let t1 = tokio::spawn(sending(sender, id));
    let t2 = tokio::spawn(receiving(recv, id));
    tokio::join!(t1, t2);
}
#[tokio::main]
async fn main() {
    let (mut s, mut r) = loole::bounded::<Message>(1000);
    let s1 = s.clone();
    let r1 = r.clone();

    let s2 = s.clone();
    let r2 = r.clone();

    let per1 = tokio::spawn(async move {
        crossbeaming(s1, r1, 1).await;
    });
    let per2 = tokio::spawn(async move {
        crossbeaming(s2, r2, 2).await;
    });
    // r.close();
    tokio::join!(per1, per2);
    // println!("{}", r.same_channel(&r1));
}
