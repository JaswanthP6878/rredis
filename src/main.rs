#![allow(unused_imports)]
use std::{
    any::Any, io::{Read, Write}, net::{TcpStream}, path::StripPrefixError, rc::Rc, string, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration
};

use std::env;

mod engine;
mod parser;
mod protocol;

use cli::Arguments;
use engine::{Engine};
use protocol::Protocol;
mod cli;
mod db;
mod utils;

use tokio::net::TcpListener;
use tokio::io;


static COUNTE  : usize = AtomicUsize::new(1);
// for a client that connects
struct Client{
    client_id: i32,
}


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
    let engine: Arc<Mutex<Engine>> = Arc::new(Mutex::new(Engine::init(arguments)));
    println!("started redis server in {}", port_number);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_number)).await.unwrap();
    println!("started listening for messages");

    // PLAN: let us take a socket for each connection, and use CSP for sending messages back and
    // forth from the engine to solve the issues;
    // main loop; will continue later
    loop {
        let (mut socket , _) = listener.accept().await.unwrap();
        let (mut rd, mut rw) = io::split(socket); // split into rread and rw parts of the stream
                                                  //
    }

    for stream in listener.incoming() {
        let engine_temp: Arc<Mutex<Engine>> = Arc::clone(&engine); // are you really moving the
                                                                   // engine into each thread my
                                                                   // guy??
        thread::spawn(move || match stream {
            Ok(mut stream) => {
                println!("MASTER: Recived Data");
                let mut string_val = String::new();
                if let Ok(_size) = stream.read_to_string(&mut string_val) {
                    let protocol_msg = parser::Parser::new(string_val).get_command();
                    println!("{:?}", protocol_msg);
                    stream
                        .write_all(engine_temp.lock().unwrap().execute(protocol_msg).as_bytes())
                        .expect("error in sending the stream");
                } else {
                    println!("cannot read the string from stream");
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }

    // gossip_thread.join().unwrap();
}
