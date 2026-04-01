// main server handling logic; spliting from main

use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};

use crate::cmd::Command;
use crate::connection::Connection;
use crate::engine::Request;
use crate::parse::Parse;

use crate::frame::Frame;

// top level listener, that takes in the TcpListner and spawns handlers for 
// each tcp stream
pub struct Listener {
    connection : TcpListener,
    tx: mpsc::Sender<Request>
}

impl Listener {
    pub fn new(listener: TcpListener, tx : mpsc::Sender<Request>) -> Self {
        Self {
            connection: listener,
            tx,
        }
    }
    pub async fn run(&self) -> Result<()> {
        println!("Started listner run");
        loop {
            if let Ok((socket, _))  = self.connection.accept().await {
                let tx = self.tx.clone();
                tokio::spawn(async move {
                    let mut handler = Handler::new(socket, tx);
                    let _ = handler.run().await;
                });
            }
        }
    }
}

// handler for each TcpStream from listener
struct Handler {
    connnection: Connection,
    tx: mpsc::Sender<Request>
}

// wrapper around connection
impl Handler {
    fn new(socket: TcpStream, tx: mpsc::Sender<Request>) -> Self {
        Self {
            connnection: Connection::new(socket),
            tx
        }
    }
    // listens to requests and loops
    pub async fn run(&mut self) -> Result<()> {
        loop {
            if let Some(val) = self.connnection.read_frame().await? {
                // val is frame
                let cmd = Command::from_frame(val)?;
                let (tx, rx) = oneshot::channel::<Frame>();
                let request  = Request {
                    protocol: cmd,
                    responder: tx
                };
                // sending the request to AsyncDBworker 
                self.tx.send(request).await?;

                let response = rx.await?;
                
                self.connnection.write_frame(&response).await?;
            } else {
                break;
            }
        }
        Ok(())
    }

}
