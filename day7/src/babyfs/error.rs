use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FileSystemError {
    msg: String,
}
impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for FileSystemError {}

impl FileSystemError {
    pub fn new(msg: &str) -> Self {
        FileSystemError {
            msg: String::from(msg),
        }
    }
}
