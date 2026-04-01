#![allow(unused_imports)]
use std::{
    any::Any, error::Error, io::{Read, Write}, net::TcpStream, path::StripPrefixError, rc::Rc, string, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration
};

use std::env;

mod engine;
mod connection;
mod server;

use cli::Arguments;
use engine::{Engine};
mod cli;
mod db;
mod utils;
pub mod frame;


mod cmd;
mod parse;

use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::oneshot};
use tokio::io;

use self::{connection::Connection, engine::{AsyncEngine, Request}};

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
    
    //NOTE:  this tx accepts Request types
    let tx = AsyncEngine::start(arguments);

    println!("started redis server in {}", port_number);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_number)).await.unwrap();
    println!("started listening for messages");
    let server = server::Listener::new(listener, tx);
    let _ = server.run().await;
}
