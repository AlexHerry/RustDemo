use bytes::Bytes;
use tokio::sync::mpsc;
use anyhow::Result;
use mini_redis::client;
use tokio::sync::oneshot;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() -> Result<()>
{
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        }).await.unwrap();

        let res = resp_rx.await.unwrap();
        println!("Got response from server; resp={:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            resp: resp_tx,
        }).await.unwrap();

        let res = resp_rx.await.unwrap();
        println!("Got response from server; resp={:?}", res);
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(command) = rx.recv().await {
            match command {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    resp.send(res).unwrap();
                },
                Command::Set { key, value, resp } => {
                    let res = client.set(&key, value).await;
                    resp.send(res).unwrap();
                }
            }
        }
    });

    t2.await?;
    t1.await?;
    manager.await?;


    Ok(())
}

pub enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>
    }
}