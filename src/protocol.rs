use std::time::SystemTime;


#[derive(Debug)]
pub enum Protocol{
    Echo(String),
    PING,
    Set(String,String, i32, Option<SystemTime>), // the i32 is the px timeout
    GET(String),
    INVALID,
}



