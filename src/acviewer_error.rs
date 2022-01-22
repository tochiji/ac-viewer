use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum AcViewerError {
    NotFound(&'static str),
    NoSubmission(&'static str),
}

impl Display for AcViewerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AcViewerError::NotFound(s) => write!(f, "Error: {}", s),
            AcViewerError::NoSubmission(s) => write!(f, "Error: {}", s),
        }
    }
}

impl Error for AcViewerError {}
