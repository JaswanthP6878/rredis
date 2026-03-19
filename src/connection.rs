use std::error::Error;

use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncReadExt, BufWriter};
use tokio::net::TcpStream;

use anyhow::Result;

use crate::frame::{self, Frame};


// connection instances for each connection
struct Connection {
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
                    return Err("Connection reset into peer".into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Option<Frame> {
        todo!("Define the parse frame and contiue")
    }
}
