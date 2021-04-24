use tokio_test::*;
use tokio::sync::{broadcast, mpsc, Mutex};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() -> Result<()>{
    println!("Hello, world!");

    // test_a().await;
    // test_broadcast().await;
    // test_mpsc().await;
    // test_net().await;
    test_timeout().await;
    println!("success");
    Ok(())
}


async fn test_a() {
    println!("hello world");
}

async fn test_broadcast() {
    let (tx, mut rx1) = broadcast::channel(6000);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        println!("r: {}",rx1.recv().await.unwrap());
        println!("r: {}",rx1.recv().await.unwrap());
    });

    tokio::spawn(async move {
        println!("r2: {}",rx2.recv().await.unwrap());
        println!("r2: {}",rx2.recv().await.unwrap());
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
}

async fn test_mpsc() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let tx = Arc::new(Mutex::new(tx));

    tokio::spawn(async move {
        for _ in 0..2 {
            let c = rx.recv().await.unwrap();
            println!("c: {}",c);
        }
    });

    let tx1 = tx.clone();
    tokio::spawn(async move {
        tx1.lock().await.send(1).unwrap();
    });

    tx.lock().await.send(2).unwrap();
}

async fn test_net() {
    println!("start");
    let mut client = timeout(
        Duration::from_secs(3),
                             TcpStream::connect("127.0.0.1:22")).await.unwrap().unwrap();
    let mut resp = String::new();
    // println!("start write");
    //
    // let c = "sss".as_bytes();
    // client.write(c).await.unwrap();
    //
    // println!("start read");
    // client.read_to_string(&mut resp).await.unwrap();
    // println!("resp: {}", resp);
}

async fn test_timeout() {
    let ti = timeout(
        Duration::from_secs(3),
        async move {
            tokio::time::sleep(Duration::from_secs(6)).await;
            println!("hello world ccc")
        },
    ).await;

  println!("r: {:?}", ti);
}