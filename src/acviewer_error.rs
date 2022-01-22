use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum AcViewerError {
    NotFoundError(&'static str),
}

impl Display for AcViewerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AcViewerError::NotFoundError(s) => write!(f, "Error: {}", s),
        }
    }
}

impl Error for AcViewerError {}
