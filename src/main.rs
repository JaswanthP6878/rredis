#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
    path::StripPrefixError,
    rc::Rc,
    string,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
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

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        println!("{:?}", args);
    }
    let arguments: Arguments = Arguments::new(args);
    // let is_replica = arguments.is_replica(); // check if this is master or replica
    // default port number is 6379;
    let default_port_number = "6379".to_string();
    let port_number = arguments
        .get_arg("port".to_string())
        .cloned()
        .unwrap_or(default_port_number);
    let engine: Arc<Mutex<Engine>> = Arc::new(Mutex::new(Engine::init(arguments)));
    println!("started redis server in {}", port_number);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_number)).unwrap();

    println!("started listening for messages");

    for stream in listener.incoming() {
        let engine_temp: Arc<Mutex<Engine>> = Arc::clone(&engine);
        // TODO:  Use a thead pool instead
        thread::spawn(move || match stream {
            Ok(mut stream) => {
                let mut string_val = String::new();
                if let Ok(_size) = stream.read_to_string(&mut string_val) {
                    let protocol_msg = parser::Parser::new(string_val).get_command();
                    stream
                        .write(engine_temp.lock().unwrap().execute(protocol_msg).as_bytes())
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
}
