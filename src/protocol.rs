
#[derive(Debug)]
pub enum Protocol{
    Echo(String),
    PING,
    Set(String,String),
    GET(String),
    INVALID,
}



