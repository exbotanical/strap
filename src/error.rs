use std::io;

#[derive(Debug)]
pub enum StrapError {
    StrapError(String),
    InvalidConfig(String),
    IoError(String),
}

impl From<io::Error> for StrapError {
    fn from(s: io::Error) -> Self {
        StrapError::IoError(s.to_string())
    }
}

impl From<serde_yaml::Error> for StrapError {
    fn from(error: serde_yaml::Error) -> Self {
        StrapError::InvalidConfig(error.to_string())
    }
}

impl From<String> for StrapError {
    fn from(s: String) -> Self {
        StrapError::StrapError(s)
    }
}
