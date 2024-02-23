use std::fmt;

#[derive(Debug)]
pub enum MazeError {
    FileNotFound(String),
    InvalidFormat(String),
    Other(String),
}

impl fmt::Display for MazeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MazeError::FileNotFound(ref cause) => write!(f, "File not found: {}", cause),
            MazeError::InvalidFormat(ref cause) => write!(f, "Invalid format: {}", cause),
            MazeError::Other(ref cause) => write!(f, "An error occurred: {}", cause),
        }
    }
}

impl std::error::Error for MazeError {}