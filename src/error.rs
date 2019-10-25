use crate::PacketDirection;
use bytes::Bytes;
use core::fmt;
use std::fmt::{Debug, Display};
use std::io;

#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Utf8(std::string::FromUtf8Error),
    ChannelRecv(std::sync::mpsc::RecvError),
    ChannelSend(std::sync::mpsc::SendError<(PacketDirection, Bytes)>),
    IntegerToEnum,
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for MyError {}

impl From<std::io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::sync::mpsc::RecvError> for MyError {
    fn from(error: std::sync::mpsc::RecvError) -> Self {
        MyError::ChannelRecv(error)
    }
}

impl From<std::sync::mpsc::SendError<(PacketDirection, Bytes)>> for MyError {
    fn from(error: std::sync::mpsc::SendError<(PacketDirection, Bytes)>) -> Self {
        MyError::ChannelSend(error)
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        MyError::Utf8(error)
    }
}

pub type MyResult<T = ()> = Result<T, MyError>;
