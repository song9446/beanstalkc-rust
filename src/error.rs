use std::fmt;
use std::error::Error;
use std::io::Error as IoError;
use std::net::AddrParseError;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug, Clone)]
pub enum BeanstalkcError {
    ConnectionError(String),
    UnexpectedResponse(String),
    CommandFailed(String),
}

impl fmt::Display for BeanstalkcError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            BeanstalkcError::ConnectionError(msg) => format!("Connection error: {}", msg),
            BeanstalkcError::UnexpectedResponse(msg) => format!("Unexpected response: {}", msg),
            BeanstalkcError::CommandFailed(msg) => format!("Command failed: {}", msg),
        };

        write!(formatter, "{}", description)
    }
}

impl Error for BeanstalkcError {
    fn description(&self) -> &str {
        ""
    }
}

impl From<IoError> for BeanstalkcError {
    fn from(err: IoError) -> Self {
        BeanstalkcError::ConnectionError(err.to_string())
    }
}

impl From<AddrParseError> for BeanstalkcError {
    fn from(err: AddrParseError) -> Self {
        BeanstalkcError::ConnectionError(err.to_string())
    }
}

impl From<ParseIntError> for BeanstalkcError {
    fn from(err: ParseIntError) -> Self {
        BeanstalkcError::UnexpectedResponse(err.to_string())
    }
}

impl From<FromUtf8Error> for BeanstalkcError {
    fn from(err: FromUtf8Error) -> Self {
        BeanstalkcError::UnexpectedResponse(err.to_string())
    }
}

pub type BeanstalkcResult<T> = Result<T, BeanstalkcError>;
