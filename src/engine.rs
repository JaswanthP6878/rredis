use core::time;
use std::{collections::HashMap, ops::DerefMut, sync::Mutex, time::{Duration, SystemTime}};
use tokio::io::DuplexStream;
use crate::protocol::Protocol;
use crate::cli::Arguments;

#[allow(dead_code)]
pub struct Engine {
    // value strores also the timeout and the time the key is inserted at
    memory: HashMap<String, (String,i32,Option<SystemTime>)>,
    rdb_file: String,
}

impl Engine {
    pub fn init() -> Self {
        Engine {
            memory: HashMap::new(),
            rdb_file: "dump.rdb".to_string(),
        }
    }
    
    pub fn execute(&mut self, request: Protocol) -> String  {
        match request {
            Protocol::Echo(val) => {
                format!("$3\r\n{}\r\n", val)
            },
            Protocol::PING => {
                "*1\r\n$4\r\nPONG\r\n".to_owned()
            }
            Protocol::Set(key, val, px,time_opt) => {
                self.memory.insert(key, (val, px, time_opt));
                return "OK".to_owned();
            },
            Protocol::GET(key) => {
                let (result, px, timeout) = self.memory.get(&key).unwrap().to_owned();
                if px < 0 {return result;}
                if let Ok(val) = SystemTime::now().duration_since(timeout.unwrap()) {
                    println!("val as mill is {}", val.as_millis() );
                    if val.as_millis() > px as u128 {
                        self.memory.remove(&key);
                        return "INVALID".to_owned()
                    }
                }
                return result;
            },
            Protocol::INVALID => {
                return "INVALD".to_owned();
            },
        }
    }

}

