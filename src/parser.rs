use std::{alloc::System, os::unix::process, thread::panicking, time::SystemTime};
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
            .filter(|val| !((*val).starts_with("$") || (*val).starts_with("*") || (*val) == "") )
            .collect();
        return lines;
    }

    pub fn get_command(&self) -> Protocol {
        let parsed_command = self.parse_command();
        println!("parsed command is {:?}", parsed_command);
        // let mut count_args = parsed_command[1]
        //     .strip_prefix("*")
        //     .unwrap()
        //     .parse::<u8>()
        //     .unwrap();
        let mut index = 0 as usize; // at indexes 2 multiples we would get the actual commands
        // println!("{}", parsed_command.len());
        if parsed_command[index].to_uppercase() == "PING" {
            return Protocol::PING;
        } else if parsed_command[index].to_uppercase() == "ECHO" {
            index += 1;
            return Protocol::Echo(parsed_command[index].to_string());
        } else if parsed_command[index].to_uppercase() == "SET" {
            let mut px_timeout = -1; // default value is -1;
            let mut time: Option<SystemTime>= None;
            if parsed_command.len() > 3{
                if parsed_command[index+3].to_uppercase() == "PX".to_owned() {
                if let Ok(val) = parsed_command[index+4].parse::<i32>() {
                    px_timeout = val;
                    time = Some(SystemTime::now())
                } else {
                    return Protocol::INVALID;
                }
                } else {
                    return Protocol::INVALID;
                }
            }
            return Protocol::Set(parsed_command[index+1].to_string(), parsed_command[index+2].to_string(), px_timeout,time);
        } else if parsed_command[index].to_uppercase() == "GET" {
            return Protocol::GET(parsed_command[index+1].to_string());
        } else if parsed_command[index].to_uppercase() == "CONFIG" {
            return Protocol::CONFIG(parsed_command[index+2].to_string());
        } else if parsed_command[index].to_uppercase() == "KEYS" {
            return Protocol::KEYS(parsed_command[index+1].to_string());
        } else if parsed_command[index].to_ascii_uppercase() == "SAVE" {
            return Protocol::SAVE;
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
        let mut  parser = Parser::new("*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n".to_string());
        parser = Parser::new("*1\r\n$4\r\nPING\r\n".to_string());
        parser = Parser::new("*3\r\n$3\r\nSET\r\n$3\r\nFOO\r\n$4\r\nBAR\r\n".to_string());
        println!("{:#?}", parser.get_command());
    }
}
