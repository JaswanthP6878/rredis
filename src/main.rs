#![allow(unused_imports)]
use std::{
    io::{Read, Write}, net::TcpListener, rc::Rc, sync::Arc, thread
};

mod parser;
mod protocol;
mod engine;

use protocol::Protocol;
use engine::Engine;



fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let engine = Arc::new(Engine::init());
    println!("started redis server in 6379");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        thread::spawn( || match stream {
            Ok(mut stream) => loop {
                let mut string_val = "".to_string();
                if let Ok(_size) = stream.read_to_string(&mut string_val) {
                    let protocol_msg = parser::Parser::new(string_val).get_command();
                    Arc::clone(&engine).execute(protocol_msg); // error in this block
                }
            },
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}
