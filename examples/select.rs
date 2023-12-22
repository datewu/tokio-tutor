use tokio::sync::oneshot;
#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        println!("send one begin");
        let _ = tx1.send("one");
        println!("send one end");
    });
    tokio::spawn(async {
        println!("send two begin");
        let _ = tx2.send("two");
        println!("send two end");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}
