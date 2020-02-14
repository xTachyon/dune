use core::fmt;
use std::fmt::{Debug, Display};
use std::io;
use bytes::Bytes;
use crate::PacketDirection;

#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Utf8(std::string::FromUtf8Error),
    TokioSend(tokio::sync::mpsc::error::SendError<(PacketDirection, Bytes)>),
    IntegerToEnum,
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for MyError {}

impl From<tokio::sync::mpsc::error::SendError<(PacketDirection, Bytes)>> for MyError {
    fn from(error: tokio::sync::mpsc::error::SendError<(PacketDirection, Bytes)>) -> Self {
        MyError::TokioSend(error)
    }
}

impl From<std::io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        MyError::Utf8(error)
    }
}

pub type MyResult<T = ()> = Result<T, MyError>;
