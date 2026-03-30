#![allow(unused_imports)]
use std::{
    any::Any, error::Error, io::{Read, Write}, net::TcpStream, path::StripPrefixError, rc::Rc, string, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration
};

use std::env;

mod engine;
mod parser;
mod protocol;
mod connection;
mod server;

use cli::Arguments;
use engine::{Engine};
use protocol::Protocol;
mod cli;
mod db;
mod utils;
pub mod frame;


mod cmd;
mod parse;

use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::oneshot};
use tokio::io;

use self::{connection::Connection, engine::{AsyncEngine, Request}, parser::Parser};

#[tokio::main]
async fn main() {
    // TODO: REimplemnt the master simply using tokio, we will look into the redis, master slave
    // later
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        println!("{:?}", args);
    }
    let arguments: Arguments = Arguments::new(args);
    let is_replica = arguments.is_replica(); // check if this is master or replica
    // default port number is 6379;
    let default_port_number = "6379".to_string();
    let port_number = arguments
        .get_arg("port".to_string())
        .cloned()
        .unwrap_or(default_port_number);
    // let engine: Arc<Mutex<Engine>> = Arc::new(Mutex::new(Engine::init(arguments))); # older sync implementation 
    //
    let tx = AsyncEngine::start(arguments); // async engine starts here

    println!("started redis server in {}", port_number);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_number)).await.unwrap();
    println!("started listening for messages");
    let server = server::Listener::new(listener);
    let _ = server.run().await;
    // TODO: Fix this with the new frame methodology
        // let engine_sender = tx.clone();
        // tokio::spawn(async move {
        //     // lets create a buffered reader
        //     let (reader, mut writer) = io::split(stream);
        //     let mut reader = BufReader::new(reader); // bufreader
        //     loop {
        //         let mut message = String::new();
        //         // reading till end of string is not valid
        //         // breaks running
        //         if reader.read_line(&mut message).await.unwrap() == 0 {  
        //             return;
        //         }
        //         println!{"{:?}", message};
        //         let command = Parser::new(message).get_command(); // Command is a protocol
        //         let (tx, rx) = oneshot::channel();
        //         let request = Request {
        //             protocol: command,
        //             responder: tx,
        //         };
        //         let _ = engine_sender.send(request).await; // sending it to the engine
        //         let val = rx.await;
        //         writer.write_all(val.unwrap().as_bytes()).await.unwrap();
        //     }
        // });
}
