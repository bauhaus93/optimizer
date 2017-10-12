use std::error::Error;
use std::fmt;

use std::io;

use serde_json;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    Json(serde_json::Error)
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> ParseError {
        ParseError::Json(err)
    }
}

impl Error for ParseError {

    fn description(&self) -> &str {
        match *self {
            ParseError::Io(_) => "IO Error",
            ParseError::Json(_) => "JSON Parse Error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::Io(ref io_error) => Some(io_error),
            ParseError::Json(ref json_error) => Some(json_error)
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Io(ref io_error) => write!(f, "IO operation failed: {}", io_error.description()),
            ParseError::Json(ref json_error) => write!(f, "Could not parse json: {}", json_error.description()),
        }
    }
}
