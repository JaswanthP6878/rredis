#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

mod parser;
mod types;

fn redis_parser(buffer: &[u8]) {
    let commands: Vec<String> = vec![];
    let lines = buffer
        .split(|&b| b == b'\n')
        .map(|line| line.strip_suffix(b"\r").unwrap_or(line));

    for i in lines {}
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    println!("started redis server in 6379");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(mut stream) => loop {
                let mut string_val = "".to_string();
                if let Ok(_size) = stream.read_to_string(&mut string_val) {
                    let protocol_msg = parser::Parser::new(string_val).get_command();
                    match protocol_msg  {
                        parser::Protocol::Echo(val) => {
                            let return_string = format!("$3\r\n{}\r\n", val);
                            stream.write(return_string.as_bytes()).unwrap();
                        }
                        parser::Protocol::PING => {
                            stream.write("*1\r\n$4\r\nPONG\r\n".as_bytes()).unwrap();
                        } _ => {
                            panic!("invallid type");
                        }
                    }
                }
            },
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}
