use std::error::Error;
use std::io::Cursor;

use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncReadExt, BufWriter};
use tokio::net::TcpStream;

use bytes::Buf;

use anyhow::{Result, anyhow};

use crate::frame::{self, Frame};


// connection instances for each connection
pub struct Connection {
    socket: BufWriter<TcpStream>,
    buffer: BytesMut,
}


impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket : BufWriter::new(socket),
            // a 5kb buffer for each conection
            buffer: BytesMut::with_capacity(5 * 1024),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        // incrementally building the frame
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }
            // error varient returnes while parsing has to treated as
            // that the stream in closed or done
            if 0 == self.socket.read_buf(&mut self.buffer).await? {
                // return None, when clean connection reset
                if self.buffer.is_empty() {
                    return Ok(None)
                } else {
                    return Err(anyhow!("Connection reset by peer"));
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        use frame::Error::Incomplete;
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;

                buf.set_position(0);

                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into())
        }

    }
}
