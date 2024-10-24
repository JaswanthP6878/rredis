#![allow(unused_imports)]
use std::{
    any::Any, io::{Read, Write}, net::{TcpListener, TcpStream}, path::StripPrefixError, rc::Rc, string, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration
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
    let is_replica = arguments.is_replica(); // check if this is master or replica
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

    // spawing gossip thread
    // let gossip_thread = thread::spawn(move || {
    //     loop {
    //         println!("Gossiping...");
    //         if is_replica {
    //             println!("connected to master");
    //             let mut stream = TcpStream::connect("127.0.0.1:5000").expect("cannot connect to stream");
    //             let message = "*1\r\n$4\r\nPING\r\n";
    //             stream.write_all(message.as_bytes()).unwrap();
    //             println!("write complete");
    //             let mut buffer = [0; 512];
    //             let n = stream.read(&mut buffer).unwrap();
    //             println!("read complete");
    //             println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    //         } else {
    //             break;
    //         }
    //         thread::sleep(Duration::from_secs(5));
    //     }
    // });

    for stream in listener.incoming() {
        let engine_temp: Arc<Mutex<Engine>> = Arc::clone(&engine);
        // TODO:  Use a thead pool instead
        thread::spawn(move || match stream {
            Ok(mut stream) => {
                println!("MASTER: Recived Data");
                let mut string_val = String::new();
                if let Ok(_size) = stream.read_to_string(&mut string_val) {
                    let protocol_msg = parser::Parser::new(string_val).get_command();
                    println!("{:?}", protocol_msg);
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

    // gossip_thread.join().unwrap();
}
