use std::fmt;

#[derive(Debug)]
pub enum MazeError {
    FileNotFound(String),
    InvalidFormat(String),
    NoBomb,
    OutOfBounds,
    Other(String),
}

impl fmt::Display for MazeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MazeError::FileNotFound(ref cause) => write!(f, "File not found: {}", cause),
            MazeError::InvalidFormat(ref cause) => write!(f, "Invalid format: {}", cause),
            MazeError::NoBomb => write!(f, "No Bomb found in the given coordinates"),
            MazeError::OutOfBounds => write!(f, "Coordinates out of bounds"),
            MazeError::Other(ref cause) => write!(f, "An error occurred: {}", cause),
        }
    }
}

impl std::error::Error for MazeError {}