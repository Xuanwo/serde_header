use std::fmt::Display;
use thiserror::Error;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    InvalidUtf8(#[from] std::str::Utf8Error),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<http::header::InvalidHeaderValue> for Error {
    fn from(e: http::header::InvalidHeaderValue) -> Self {
        Error::Message(e.to_string())
    }
}

impl From<http::header::InvalidHeaderName> for Error {
    fn from(e: http::header::InvalidHeaderName) -> Self {
        Error::Message(e.to_string())
    }
}