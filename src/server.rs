// main server handling logic; spliting from main

use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};

use crate::connection::Connection;

// top level listener, that takes in the TcpListner and spawns handlers for 
// each tcp stream
pub struct Listener {
    connection : TcpListener,
}

impl Listener {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            connection: listener
        }
    }
    pub async fn run(&self) -> Result<()> {
        println!("Started listner run");
        loop {
            if let Ok((socket, _))  = self.connection.accept().await {
                tokio::spawn(async move {
                    let mut handler = Handler::new(socket);
                    let _ = handler.run().await;
                });
            }
        }
    }
}

// handler for each TcpStream from listener
struct Handler {
    connnection: Connection
}

// wrapper around connection
impl Handler {
    fn new(socket: TcpStream) -> Self {
        Self {
            connnection: Connection::new(socket),
        }
    }

    // main handler runs here; for now prints frames
    pub async fn run(&mut self) -> Result<()> {
        loop {
            if let Some(val) = self.connnection.read_frame().await? {
                println!("{:?}", val); // convert it into a command and apply that command
            } else {
                break;
            }
        }
        Ok(())
    }

}
