use std::collections::HashMap;
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;
use mini_redis::{Connection, Frame};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> Result<()>
{
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let db: Db = Arc::new(Mutex::new(HashMap::new()));


    loop {
        let (socket, _) = listener.accept().await?;
        let db = Arc::clone(&db);

        tokio::spawn(async move {
            process(socket, db).await
        });
    }
}

async fn process(socket: TcpStream, db: Db) -> Result<()> {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },
            cmd => panic!("unimplemented command: {:?}", cmd),
        };

        // sleep(Duration::from_secs(10)).await;
        //
        // let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }

    Ok(())
}