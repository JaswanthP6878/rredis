use std::{collections::HashMap, sync::atomic};

#[derive(Debug, Clone)]
pub struct Arguments {
    args: HashMap<String, String>,
}

impl Arguments {
    pub fn new(mut args: Vec<String>) -> Self {
        let mut arguments: HashMap<String, String> = HashMap::new();
        for i in 0..args.len() {
            if args[i].starts_with("--") {
                println!(
                    "index value is {}, value is {}: {}",
                    i,
                    args[i],
                    args[i + 1]
                );
                let _val =
                    arguments.insert(args[i].split_off(2).to_owned(), args[i + 1].to_owned());
            }
        }
        Arguments { args: arguments }
    }

    pub fn get_arg(&self, val: String) -> Option<&String> {
        self.args.get(&val)
    }
    pub fn is_replica(&self) -> bool {
        if let Some(_) = self.args.get("replicaof") {
            return true;
        }

        return false;
    }

    pub fn get_dir(&self) -> Option<&String> {
        self.args.get("dir")
    }
    pub fn get_dbfile(&self) -> Option<&String> {
        self.args.get("dbfilename")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_args() {
        let args = vec![
            "--dir".to_string(),
            "/tmp/base-dir".to_string(),
            "--dbfilename".to_string(),
            "dump.rdb".to_string(),
        ];
        let arguments = Arguments::new(args);
        println!("{:#?}", arguments);
    }
}
