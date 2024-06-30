use tokio;

#[tokio::main]

async fn main(){
    let t1 = tokio::spawn(async move{
        for i in 0..100{
            println!("{}",i);
        }
    });
    let t2 = tokio::spawn(async move{
        for i in 100..200{
            println!("{}",i);
        }
    });
    tokio::join!(t1,t2);

}