use regex::Error as RegexError;
use reqwest::Error as ReqwestError;
use std::io::Error as IoError;

#[derive(Debug, Fail)]
/// Custom type for inspection errors
pub enum CiError {
    /// Error during I/O handling
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] IoError),
    /// Error during regex handling
    #[fail(display = "{}", _0)]
    Regex(#[fail(cause)] RegexError),
    /// Error when invoking tokio
    #[fail(display = "{}", _0)]
    Tokio(String),
    /// No input file
    #[fail(display = "{}", _0)]
    Input(String),
    /// HTTP request error
    #[fail(display = "{}", _0)]
    Request(#[fail(cause)] ReqwestError),
}

impl From<IoError> for CiError {
    fn from(e: IoError) -> Self {
        CiError::Io(e)
    }
}

impl From<RegexError> for CiError {
    fn from(e: RegexError) -> Self {
        CiError::Regex(e)
    }
}

impl From<()> for CiError {
    fn from(_e: ()) -> Self {
        CiError::Tokio(String::from("Error while executing tokio core"))
    }
}

impl From<reqwest::Error> for CiError {
    fn from(e: reqwest::Error) -> Self {
        CiError::Request(e)
    }
}
