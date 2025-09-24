use tokio::net::TcpListener;
// use tokio::net::TcpStream;
use anyhow::Result;
use tokio::io;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    // let socket = TcpStream::connect("127.0.0.1:6142").await?;
    // let (mut rd, mut wr) = io::split(socket);
    // let mut listener = TcpListener::bind("127.0.0.1:6379").await?;

    // tokio::spawn(async move {
    //     wr.write_all(b"hello\n").await.unwrap();
    //     wr.write_all(b"world\n").await.unwrap();
    //
    //     Ok::<(), io::Error>(())
    // });
    //
    // let mut buffer = vec![0; 128];
    //
    // loop {
    //     let n = rd.read(&mut buffer).await?;
    //     if n == 0 {
    //         break;
    //     }
    //
    //     println!("got {:?}", &buffer[..n]);
    // }
    // Ok(())

    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy from server");
            }
        });
    }

}