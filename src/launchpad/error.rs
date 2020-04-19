use std::error;
use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    ConnectionError(portmidi::Error),
    UnknownButtonError(String),
    IllegalButtonIdError(usize),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::ConnectionError(ref e) => Some(e),
            Error::UnknownButtonError(_) => None,
            Error::IllegalButtonIdError(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ConnectionError(e) => e.fmt(f),
            Error::UnknownButtonError(name) => write!(f, "unknown button with name {}", name),
            Error::IllegalButtonIdError(code) => write!(f, "given button code {} is invalid.", code),
        }
    }
}

impl From<portmidi::Error> for Error {
    fn from(err: portmidi::Error) -> Self {
        Error::ConnectionError(err)
    }
}