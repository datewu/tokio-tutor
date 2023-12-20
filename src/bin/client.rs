use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

use bytes::Bytes;
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    resp.send(res).unwrap();
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    resp.send(res).unwrap();
                }
            }
        }
    });
    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (o_tx, rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: o_tx,
        };
        tx.send(cmd).await.unwrap();
        let res = rx.await;
        println!("tx1 got = {:?}", res);
    });
    let t2 = tokio::spawn(async move {
        let (o_tx, rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: o_tx,
        };
        tx2.send(cmd).await.unwrap();
        let res = rx.await;
        println!("tx2 got = {:?}", res);
    });
    // while let Some(m) = rx.recv().await {
    //     println!("GOT ={:?}", m);
    // }
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
