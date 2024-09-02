use std::{collections::HashMap, ops::DerefMut, sync::Mutex};

use crate::protocol::Protocol;

pub struct Engine {
    memory: Mutex<HashMap<String, String>>,
    rdb_file: String
}

impl Engine {
    pub fn init() -> Self {
        Engine {
            memory: Mutex::new(HashMap::new()),
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
            Protocol::Set(Key, Val) => {
                self.memory.lock().unwrap().insert(Key, Val);
                return "OK".to_owned();
            },
            Protocol::GET(Key) => {
                let result = self.memory.lock().unwrap().get(&Key).unwrap().to_owned();
                return result;
            },
            Protocol::INVALID => {
                return "INVALD".to_owned();
            },
        }
    }

}

