use std::fmt;

pub enum CheckerError {
    EmptySource,
    NoHeaders,
    NoHeaderName(String),
    IoError(csv::Error),
    CsvError(csv::Error),
}

impl std::error::Error for CheckerError {}

impl fmt::Display for CheckerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptySource => write!(f, "File doesn't contain data"),
            Self::NoHeaders => write!(f, "There is no headers in file"),
            Self::NoHeaderName(name) => write!(f, "There is no header {} in file", name),
            Self::IoError(e) => write!(f, "IO Error caused by: {}", e),
            Self::CsvError(e) => write!(f, "CSV Error caused by: {}", e),
        }
    }
}

impl fmt::Debug for CheckerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptySource => write!(f, "File doesn't contain data"),
            Self::NoHeaders => write!(f, "There is no headers in file"),
            Self::NoHeaderName(name) => write!(f, "There is no header {} in file", name),
            Self::IoError(e) => write!(f, "IO Error caused by: {}", e),
            Self::CsvError(e) => write!(f, "CSV Error caused by: {}", e),
        }
    }
}
