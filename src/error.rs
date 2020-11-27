use std::fmt;

pub enum CheckerError {
    EmptySource,
    IoError(csv::Error),
    CsvError(csv::Error),
}

impl std::error::Error for CheckerError {}

impl fmt::Display for CheckerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptySource => write!(f, "File doesn't contain data"),
            Self::IoError(e) => write!(f, "IO Error caused by: {}", e),
            Self::CsvError(e) => write!(f, "CSV Error caused by: {}", e),
        }
    }
}

impl fmt::Debug for CheckerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptySource => write!(f, "File doesn't contain data"),
            Self::IoError(e) => write!(f, "IO Error caused by: {}", e),
            Self::CsvError(e) => write!(f, "CSV Error caused by: {}", e),
        }
    }
}
