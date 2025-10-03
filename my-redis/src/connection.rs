use std::io::Cursor;
use bytes::{Buf, BytesMut};
use mini_redis::{Frame, Result, frame::Error::Incomplete};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;


pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection
    {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096)
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>>
    {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err("connection closed".into())
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>>
    {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;

                buf.set_position(0);

                let frame = Frame::parse(&mut buf)?;

                self.buffer.advance(len);

                Ok(Some(frame))
            },
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    #[warn(dead_code)]
    async fn write_frame(&mut self, frame: Frame) -> Result<()>
    {
        match frame {
            Frame::Simple(simple) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write(simple.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Error(error) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write(error.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Integer(integer) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(integer).await?;
            },
            Frame::Bulk(data) => {
                let len = data.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(data.as_ref()).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            },
            Frame::Array(array) => unimplemented!()
        }

        self.stream.flush().await?;

        Ok(())
    }

    #[warn(dead_code)]
    async fn write_decimal(&mut self, val: u64) -> Result<()> {
        use std::io::Write;

        // Convert the value to a string
        let mut buf = [0u8; 12];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}