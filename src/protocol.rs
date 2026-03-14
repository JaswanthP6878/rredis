use std::time::SystemTime;

#[derive(Debug)]
// This is for the input requests we get;
pub enum Protocol {
    Echo(String),
    PING,
    Set(String, String, i32, Option<SystemTime>), // the i32 is the px timeout
    GET(String),
    CONFIG(String),
    // save the values on the hashmap onto a disk
    SAVE,
    INVALID,
    /// Returns all the keys that match the String format;
    KEYS(String),
    INFO(String),
}
#[derive(Debug)]
pub struct Response {
    items: Vec<String>,
}

impl Response {
    pub fn new() -> Self {
        return Self { items: vec![] };
    }
    pub fn add_item(&mut self, val: String) {
        self.items.push(val);
    }

    pub fn construct_response(&self) -> String {
        if self.items.len() == 0 {
            return "$-1\r\n".to_string();
        } else {
            let mut response_str = format!("${}\r\n", self.items.len());
            self.items
                .iter()
                .map(|val| format!("{}\r\n", val))
                .for_each(|val| response_str.push_str(&val));
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
        response.add_item("bar".into());
        response.add_item("foo".into());
        assert_eq!(
            "$2\r\nbar\r\nfoo\r\n".to_string(),
            response.construct_response()
        );
    }
}
