use std::error::Error;
use std::fmt;

use client::parse_error::ParseError;

#[derive(Debug)]
pub enum ConnectionError {
    Parse(ParseError)
}

impl From<ParseError> for ConnectionError {
    fn from(err: ParseError) -> ConnectionError {
        ConnectionError::Parse(err)
    }
}

impl Error for ConnectionError {

    fn description(&self) -> &str {
        match *self {
            ConnectionError::Parse(_) => "Parse error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectionError::Parse(ref parse_error) => Some(parse_error)
        }
    }
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::Parse(ref parse_error) => write!(f, "Failed to parse json, {}", parse_error.description())
        }

    }
}
