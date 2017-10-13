use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AuthorizationHeaderError {
    Conversion
}

impl Error for AuthorizationHeaderError {

    fn description(&self) -> &str {
        match *self {
            AuthorizationHeaderError::Conversion => "ConversionError"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AuthorizationHeaderError::Conversion => None
        }
    }
}

impl fmt::Display for AuthorizationHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthorizationHeaderError::Conversion => write!(f, "Failed to convert string to header")
        }
    }
}
