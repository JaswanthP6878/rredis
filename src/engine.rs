use crate::cli::Arguments;
use crate::cmd::Command;
use crate::{db, frame};
use crate::{
    db::Db,
    utils::Role,
};
use anyhow::{Error, Result};
use bytes::Bytes;
use core::time;
use std::{
    collections::HashMap,
    fs::remove_dir,
    ops::DerefMut,
    sync::Mutex,
    time::{Duration, SystemTime},
};

use crate::frame::Frame;

use tokio::sync::{mpsc, oneshot};


// requests sent to an engine
pub struct Request {
    pub protocol: Command,
    pub responder:  oneshot::Sender<Frame>, // for now changing to String
}



// Async wrapper around Engine for enabling mpcs communication
pub struct AsyncEngine {}

impl AsyncEngine {
    pub fn start(args: Arguments ) -> mpsc::Sender<Request>  {
        let (tx, mut rx) = mpsc::channel::<Request>(5);
        let mut engine =  Engine::init(args);

        tokio::spawn(async move {
            while let Some(req) = rx.recv().await {
                let result = engine.execute(req.protocol);
                // result Frame is sent in the oneshot channel
                // thats it
                let _ = req.responder.send(result);
            }
        });
        tx
    }
}


#[allow(dead_code)]
pub struct Engine {
    memory: HashMap<String, Bytes>,
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
    // make execute send frame
    // NOTE: sending null frame for now in case of errors need to fix later
    pub fn execute(&mut self, cmd: Command) -> Frame {
        match cmd {
            Command::Set(set) => {
                self.memory.insert(set.key().into(), set.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(get) => {
                if let Some(result) = self.memory.get(get.key()) {
                    Frame::Bulk(result.clone())
                } else {
                    Frame::Null
                }

            }
            Command::Ping(ping) => {
                let response = match ping.msg {
                    Some(msg) => Frame::Bulk(msg),
                    None => Frame::Simple("PONG".to_string()),
                };
                response
            },
            // NOTE: Will use it for the py-redis client sending "LIBINFO"
            // and other nonsense expext ok msg response from here
            Command::Unknown(_) => Frame::Simple("OK".to_string()),
        }
    }
}

