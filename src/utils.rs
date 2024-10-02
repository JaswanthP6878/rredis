pub enum Role {
    // Slave has host and IP of master;
    // fist string is the hostIP and second string is port number
    Slave(String, String),
    // replcation ID for Master
    Master(String),
}
