use bytes::Bytes;



// parsing the redis protocol frames
#[derive(Debug, Clone)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}



impl Frame {
    pub fn array() -> Self {
        return Frame::Array(vec![]);
    }


    pub fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Frame::Array(vec) =>  vec.push(Frame::Bulk(bytes)),
            _ => panic!("cannot push to bulk as string not vec not initiated correctly")
        }
    }

    pub fn push_int(&mut self, value: u64) {
        match self {
            Frame::Array(vec) => vec.push(Frame::Integer(value)),
            _ => panic!("cannot push int; not a vec Frame")
        }
    }

}



