use std::os::unix::process;
use crate::protocol::Protocol;

pub struct Parser {
    value: String,
}

impl Parser {
    pub fn new(buf: String) -> Self {
        Self { value: buf }
    }
    fn parse_command(&self) -> Vec<&str> {
        let lines = self
            .value
            .split('\n')
            .map(|val| val.strip_suffix("\r").unwrap_or(val))
            .collect();
        return lines;
    }

    pub fn get_command(&self) -> Protocol {
        let parsed_command = self.parse_command();
        println!("{:?}", parsed_command);
        // let mut count_args = parsed_command[1]
        //     .strip_prefix("*")
        //     .unwrap()
        //     .parse::<u8>()
        //     .unwrap();
        let mut index = 2 as usize; // at indexes 2 multiples we would get the actual commands
        if parsed_command[index].to_uppercase() == "PING" {
            return Protocol::PING;
        } else if parsed_command[index].to_uppercase() == "ECHO" {
            index += 2;
            return Protocol::Echo(parsed_command[index].to_string());
        } else if parsed_command[index].to_uppercase() == "SET" {
            return Protocol::Set(parsed_command[index+2].to_string(), parsed_command[index+4].to_string());
        } else if parsed_command[index].to_uppercase() == "GET" {
            return Protocol::GET(parsed_command[index+2].to_string());
        }
        return Protocol::INVALID;
    }
}



// PING command would be sent as *1\r\n$4\r\nPING\r\n

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parser() {
        let parser = Parser::new("*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n".to_string());
        let parser = Parser::new("*1\r\n$4\r\nPING\r\n".to_string());
        println!("{:#?}", parser.get_command());
    }
}
