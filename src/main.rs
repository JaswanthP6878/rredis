#![allow(unused_imports)]
use std::{
    io::{Read, Write}, net::TcpListener, path::StripPrefixError, rc::Rc, string, sync::{Arc, Mutex}, thread 
};

use std::env;


mod parser;
mod protocol;
mod engine;

use protocol::Protocol;
use engine::Engine;



fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        println!("{:?}", args);
    }
    println!("Logs from your program will appear here!");
    let engine = Arc::new(Mutex::new(Engine::init()));
    println!("started redis server in 6379");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let engine_temp = Arc::clone(&engine);
        // TODO:  Use a thead pool instead
        thread::spawn( move || match stream { 
            Ok(mut stream) =>  {
                let mut string_val = String::new();
                if let Ok(_size) = stream.read_to_string(&mut string_val) {
                    let protocol_msg = parser::Parser::new(string_val).get_command();
                    stream.write(engine_temp.lock().unwrap().execute(protocol_msg).as_bytes()).expect("error in sending the stream");
                } else {
                    println!("cannot read the string from stream");
                }
            },
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}
