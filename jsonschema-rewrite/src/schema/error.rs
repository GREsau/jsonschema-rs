use std::{fmt, result};
use url::ParseError;

/// An error that happened compiling a JSON Schema.
#[derive(Debug)]
pub enum Error {
    /// URL is not valid.
    /// Could happen in `$id`, `$ref` and some other keywords.
    InvalidUrl(ParseError),
    /// Network failure during resolving.
    Network(reqwest::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self::InvalidUrl(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Network(error)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl(error) => error.fmt(f),
            Error::Network(error) => error.fmt(f),
        }
    }
}