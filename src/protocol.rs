use std::time::SystemTime;


#[derive(Debug)]
// This is for the input requests we get;
pub enum Protocol{
    Echo(String),
    PING,
    Set(String,String, i32, Option<SystemTime>), // the i32 is the px timeout
    GET(String),
    CONFIG(String),
    INVALID,
    /// Returns all the keys that match the String format;
    KEYS(String)
}
#[derive(Debug)]
pub struct Response<'a> {
    items: Vec<&'a str>
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        return Self {
            items: vec![]
        }
    }
    pub fn add_item(&mut self, val: &'a str) {
        self.items.push(val);
    }

    pub fn construct_response(&mut self) -> String {
        if self.items.len() == 0 {
            return "$-1\r\n".to_string();
        } else {
            let mut response_str = format!("${}\r\n", self.items.len());
            self.items.iter_mut().map(|val| format!("{}\r\n", val)).for_each(|val | response_str.push_str(&val));
           return response_str;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response() {
        let mut response = Response::new();
        response.add_item("bar");
        response.add_item("foo");
        assert_eq!("$2\r\nbar\r\nfoo\r\n".to_string(), response.construct_response());
    }
}







