use core::time;
use std::{collections::HashMap, fs::remove_dir, ops::DerefMut, sync::Mutex, time::{Duration, SystemTime}};
use tokio::io::DuplexStream;
use crate::{db::Db, protocol::{Protocol, Response}};
use crate::cli::Arguments;
use crate::db;

#[allow(dead_code)]
pub struct Engine {
    // value strores also the timeout and the time the key is inserted at
    memory: HashMap<String, (String,i32,Option<SystemTime>)>,
    arguments: Arguments,
    rdb_file: String,
    rdb_path: String,
    db: Db,
}

impl Engine {
    pub fn init(args: Arguments) -> Self {
        let mut  rdb_file = String::from("dump.rdb");
        let mut rdb_path: String = String::from("/tmp");
        if let Some(val) = args.get_dbfile() {
            rdb_file = val.to_string();
        }
        if let Some(val) = args.get_dir() {
            rdb_path = val.to_string();
        }
        let rdb_path_db = rdb_path.clone();
        let rdb_file_db = rdb_file.clone();
        Engine {
            memory: HashMap::new(),
            rdb_file,
            rdb_path,
            arguments: args,
            db: Db::new(rdb_path_db, rdb_file_db)
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
                    let values = self.memory.keys().into_iter()
                    .filter(|s| s.starts_with(prefix))
                    .collect::<Vec<_>>();
                    for val in values {
                        reponse.add_item(val);
                    }
                } else {
                    let values = self.memory.keys().into_iter()
                    .filter(|s| *s == &val)
                    .collect::<Vec<_>>(); 
                    for val in values {
                        reponse.add_item(val);
                    }
                }

                return reponse.construct_response();
            },
            Protocol::SAVE => {
                let mut response: Response<'_> = Response::new();
                match self.db.persist_to_db(&self.memory) {
                    Ok(_val) => {
                        response.add_item("Ok");
                        return response.construct_response();
                    }, 
                    Err(e) => {
                        println!("error saving data: {}", e);
                        response.construct_response()
                    }
                }
            },
            
        }
    }

}

