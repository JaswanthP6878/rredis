use std::{fs::File, io::{self, Write}, path::{Path, PathBuf}, time::SystemTime};
use std::collections::HashMap;


// persistence for the redis system
// Only Storing Key and values for simplicty 
// not storing as per the spec.. maybe future implmenentation will do
pub struct Db {
    db_file: PathBuf,
}

impl Db {
    pub fn new(dir: String, dbname: String) -> Self {
        let path = Path::new(&dir);
        let path_buf = path.join(Path::new(&dbname));

        Self {
            db_file: path_buf.as_path().to_owned(),
        }
    }
    // Saving the Key and value in the file;
    pub fn persist_to_db(&self, data: &HashMap<String, (String,i32,Option<SystemTime>)>) -> Result<(), io::Error> {
        let mut file_handle = File::create(&self.db_file)?;
        for (key, values) in data.iter() {
            file_handle.write_all(key.as_bytes())?;
            file_handle.write_all(b"\n")?;
            file_handle.write_all(values.0.as_bytes())?;
            file_handle.write_all(b"\n")?;
        }
        Ok(())
    }
}

