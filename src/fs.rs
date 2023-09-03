#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait FileReader {
    fn read_file_string(&self, path: &str) -> Result<String, std::io::Error>;
}

pub struct RealFileReader;

impl FileReader for RealFileReader {
    fn read_file_string(&self, path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
}
