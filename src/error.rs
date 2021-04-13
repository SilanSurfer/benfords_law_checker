use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckerError {
    #[error("File doesn't contain any data")]
    EmptySource,
    #[error("There is no headers in file")]
    NoHeaders,
    #[error("Couldn't find required header `{0}` in file")]
    NoHeaderName(String),
    #[error("IO error caused by: `{0}`")]
    IoError(csv::Error),
    #[error("CSV error caused by: `{0}`")]
    CsvError(#[from] csv::Error),
    #[error("Printing graph failed due to lack of output file extension")]
    GraphOutputFileError,
    #[error("Graph plotter error caused by: `{0}`")]
    GraphPlotterDrawingError(String),
}
