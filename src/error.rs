use serde::de;
use std::fmt::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    De(#[from] de::value::Error),
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::De(de::value::Error::custom(msg))
    }
}
