use crate::cli::Arguments;
use crate::db;
use crate::{
    db::Db,
    protocol::{Protocol, Response},
    utils::Role,
};
use anyhow::{Error, Result};
use core::time;
use std::{
    collections::HashMap,
    fs::remove_dir,
    ops::DerefMut,
    sync::Mutex,
    time::{Duration, SystemTime},
};
use tokio::io::DuplexStream;

pub trait Gossip {
    fn run(&mut self) -> Result<()>;
    fn handshake(&mut self) -> Result<()>;

    // for talking with a master
    fn talk(&mut self, msg: &str) -> Result<String>;
}

#[allow(dead_code)]
pub struct Engine {
    // value strores also the timeout and the time the key is inserted at
    memory: HashMap<String, (String, i32, Option<SystemTime>)>,
    arguments: Arguments,
    rdb_file: String,
    rdb_path: String,
    db: Db,
    role: Role,
}

impl Engine {
    pub fn init(args: Arguments) -> Self {
        let mut rdb_file = String::from("dump.rdb");
        let mut rdb_path: String = String::from("/tmp");
        if let Some(val) = args.get_dbfile() {
            rdb_file = val.to_string();
        }
        if let Some(val) = args.get_dir() {
            rdb_path = val.to_string();
        }
        let rdb_path_db = rdb_path.clone();
        let rdb_file_db = rdb_file.clone();
        // identify master or slave role
        let mut role = Role::Master("8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb".to_string());
        if let Some(val) = args.get_arg("replicaof".to_string()) {
            let master_details = val.split_ascii_whitespace().collect::<Vec<_>>();
            role = Role::Slave(master_details[0].to_string(), master_details[1].to_string());
        }
        Engine {
            memory: HashMap::new(),
            rdb_file,
            rdb_path,
            arguments: args,
            db: Db::new(rdb_path_db, rdb_file_db),
            role,
        }
    }

    pub fn execute(&mut self, request: Protocol) -> String {
        match request {
            Protocol::Echo(val) => {
                format!("$3\r\n{}\r\n", val)
            }
            Protocol::PING => {
                println!("Recived Ping sending pong");
                "*1\r\n$4\r\nPONG\r\n".to_owned()
            }
            Protocol::Set(key, val, px, time_opt) => {
                self.memory.insert(key, (val, px, time_opt));
                return "OK".to_owned();
            }
            Protocol::GET(key) => {
                let (result, px, timeout) = self.memory.get(&key).unwrap().to_owned();
                if px < 0 {
                    return result;
                }
                if let Ok(val) = SystemTime::now().duration_since(timeout.unwrap()) {
                    println!("val as mill is {}", val.as_millis());
                    if val.as_millis() > px as u128 {
                        self.memory.remove(&key);
                        return "INVALID".to_owned();
                    }
                }
                return result;
            }
            Protocol::CONFIG(val) => {
                if let Some(v) = self.arguments.get_arg(val) {
                    return v.to_owned();
                } else {
                    return "INVALID".to_owned();
                }
            }
            Protocol::INVALID => {
                return "INVALD".to_owned();
            }
            Protocol::KEYS(val) => {
                println!("Key pattern is {}", val);
                let mut reponse: Response<'_> = Response::new();
                if val == "*".to_string() {
                    for key in self.memory.keys() {
                        reponse.add_item(key);
                    }
                } else if val.ends_with("*") {
                    let prefix = &val[..val.len() - 1];
                    let values = self
                        .memory
                        .keys()
                        .into_iter()
                        .filter(|s| s.starts_with(prefix))
                        .collect::<Vec<_>>();
                    for val in values {
                        reponse.add_item(val);
                    }
                } else {
                    let values = self
                        .memory
                        .keys()
                        .into_iter()
                        .filter(|s| *s == &val)
                        .collect::<Vec<_>>();
                    for val in values {
                        reponse.add_item(val);
                    }
                }

                return reponse.construct_response();
            }
            Protocol::SAVE => {
                let mut response: Response<'_> = Response::new();
                match self.db.persist_to_db(&self.memory) {
                    Ok(_val) => {
                        response.add_item("Ok");
                        return response.construct_response();
                    }
                    Err(e) => {
                        println!("error saving data: {}", e);
                        response.construct_response()
                    }
                }
            }
            Protocol::INFO(val) => {
                let mut response = Response::new();
                if val == "replication" {
                    match &self.role {
                        Role::Master(_replid) => {
                            response.add_item("role:master");
                            response
                                .add_item("master_replid:8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb");
                            response.add_item("master_repl_offset:0");
                        }
                        Role::Slave(_, _) => {
                            response.add_item("role:slave");
                        }
                    }
                }
                response.construct_response()
            }
        }
    }
}

impl Gossip for Engine {
    fn run(&mut self) -> Result<()> {
        self.handshake().expect("failed");
        Ok(())
    }

    fn handshake(&mut self) -> Result<()> {
        let _ = self.talk("ping")?;
        // println!("Recived command {message}");
        Ok(())
    }

    fn talk(&mut self, _msg: &str) -> Result<String> {
        use std::io::{Read, Write};
        use std::net::TcpStream;
        if let Some(val) = self.arguments.get_arg("replicaof".to_string()) {
            let master_details = val.split_ascii_whitespace().collect::<Vec<_>>();
            let (host, port) = (master_details[0].to_string(), master_details[1].to_string());

            match TcpStream::connect(format!("{}:{}", host, port)) {
                Ok(mut stream) => {
                    println!("successfully connected to master");
                    let val = "*1\r\n$4\r\nPING\r\n";

                    match stream.write(val.as_bytes()) {
                        Ok(_) => {
                            println!("sent data to master");
                            // Ok(format!("data sent"))
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                    // read response from stream
                    println!("waiting for resp from master");
                    let mut buffer = [0; 512];
                    match stream.read(&mut buffer) {
                        Ok(size) => Ok(String::from_utf8_lossy(&buffer[0..size]).to_string()),
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        } else {
            return Err(Error::msg("not a replica"));
        }
    }
}
