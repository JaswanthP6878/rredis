use std::io::Cursor;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;
use anyhow::anyhow;
use bytes::{Buf, Bytes};
use std::fmt;

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

// error defintion
#[derive(Debug)]
pub enum Error {
    Incomplete,
    Other(anyhow::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Incomplete => "stream ended early".fmt(fmt),
            Error::Other(err) => err.fmt(fmt),
        }
    }
}

impl From<String> for Error {
    fn from(src: String) -> Self {
        Error::Other(anyhow!(src))
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<TryFromIntError> for Error {

    fn from(_value: TryFromIntError) -> Self {
        return Error::Other(anyhow!("Error parsing integer"));
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_value: FromUtf8Error) -> Self {
        Error::Other(anyhow!("Error converting from byte array to String"))
    }
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

    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        match get_u8(src)? {
            b'+' => { 
                get_line(src)?;
                Ok(())
            }
            b'-' => {
                get_line(src)?;
                Ok(())
            }
            b':' => {
                let _ = get_decimal(src)?;
                Ok(())
            }
            b'$' => {
                if b'-' == peek_u8(src)? {
                    skip(src, 4)
                } else {
                    let len: usize = get_decimal(src)?.try_into()?;

                    skip(src, len+2)
                }
            }
            b'*' => {
                let len = get_decimal(src)?;
                for _ in 0..len {
                    Frame::check(src)?
                }
                Ok(())
            }
            // corruption case as we cannot get this case at all
            actual => {
                Err(format!("protocol error; invalid frame type byte : {}", actual).into())
            }
        }
    }
    // main parse function;
    // NOTE: We use this only after fn check is successfull;
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        match get_u8(src)? {
            // Simple strings
            b'+' => { 
                let line = get_line(src)?.to_vec();
                let string = String::from_utf8(line)?;

                Ok(Frame::Simple(string))
            }
            // errors
            b'-' => {
                let line = get_line(src)?.to_vec();
                let string = String::from_utf8(line)?;
                Ok(Frame::Simple(string))
            }
            // integers
            b':' => { 
                let len = get_decimal(src)?;
                Ok(Frame::Integer(len))
            }
            // bulk string
            b'$' => {
                let len = get_decimal(src)?.try_into()?;
                let n = len + 2; 
                if src.remaining() < n {
                    return Err(Error::Incomplete);
                }
                let data = Bytes::copy_from_slice(&src.chunk()[..len]);
                skip(src, n)?;

                Ok(Frame::Bulk(data))
            }
            b'*' => {
                let len = get_decimal(src)?.try_into()?;
                let mut out = Vec::with_capacity(len);
                for _ in 0..len {
                    out.push(Frame::parse(src)?);
                }
                Ok(Frame::Array(out))

            }
            // corruption case s we cannot get this case at all
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn to_error(&self) -> anyhow::Error {
        anyhow!("unexpected frame : {}", self)
    }

}

fn peek_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    Ok(src.chunk()[0])
}


fn skip(src: &mut Cursor<&[u8]>, n: usize) -> Result<(), Error> {
    if src.remaining() < n {
        return Err(Error::Incomplete);
    }
    src.advance(n);
    Ok(())
}

fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<u64, Error> {
    use atoi::atoi;
    let line = get_line(src)?;
    atoi::<u64>(line).ok_or_else(|| "Protocol error; Invalid frame format".into())
}


fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error>{
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    // NOTE: cursor<>.get_u8 also moves the pointer by 1
    Ok(src.get_u8())
}


// gets the meaning full unit from format, while moving the cursor position
// to the next char after \r\n
fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
    let start = src.position() as usize;
    let end = src.get_ref().len() - 1;
    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i+1] == b'\n' {
            src.set_position((i + 2) as u64);
            return Ok(&src.get_ref()[start..i]);
        }
    }
    Err(Error::Incomplete)

}

// for debug purposes
impl fmt::Display for Frame {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::str;

        match self {
            Frame::Simple(response) => response.fmt(fmt),
            Frame::Error(msg) => write!(fmt, "error: {}", msg),
            Frame::Integer(num) => num.fmt(fmt),
            Frame::Bulk(msg) => match str::from_utf8(msg) {
                Ok(string) => string.fmt(fmt),
                Err(_) => write!(fmt, "{:?}", msg),
            },
            Frame::Null => "(nil)".fmt(fmt),
            Frame::Array(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        // use space as the array element display separator
                        write!(fmt, " ")?;
                    }

                    part.fmt(fmt)?;
                }

                Ok(())
            }
        }
    }
}

