use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listenre = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening on 6379");
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listenre.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);

    while let Some(f) = connection.read_frame().await.unwrap() {
        let reponse = match Command::from_frame(f).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(v) = db.get(cmd.key()) {
                    Frame::Bulk(v.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&reponse).await.unwrap();
    }
}
