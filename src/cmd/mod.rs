use anyhow::Result;



// construct from frame parsing
#[derive(Debug)]
pub enum Command {
    Get(Get),
    Publish(Publish),
    Set(Set),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
    Ping(Ping),
    Unknown(Unknown),
}

impl Command {
    fn from_frame(frame: Frame) -> Result<Command> {
        
    }
}

