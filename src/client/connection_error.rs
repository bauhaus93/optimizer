use std::error::Error;
use std::fmt;
use std::str;

use curl;

use client::parse_error::ParseError;
use client::authorization_error::AuthorizationError;

#[derive(Debug)]
pub enum ConnectionError {
    Parse(ParseError),
    Authorization(AuthorizationError),
    Curl(curl::Error),
    BadResponse(u32),
    Utf8(str::Utf8Error)
}

impl From<ParseError> for ConnectionError {
    fn from(err: ParseError) -> ConnectionError {
        ConnectionError::Parse(err)
    }
}

impl From<AuthorizationError> for ConnectionError {
    fn from(err: AuthorizationError) -> ConnectionError {
        ConnectionError::Authorization(err)
    }
}

impl From<curl::Error> for ConnectionError {
    fn from(err: curl::Error) -> ConnectionError {
        ConnectionError::Curl(err)
    }
}

impl From<str::Utf8Error> for ConnectionError {
    fn from(err: str::Utf8Error) -> ConnectionError {
        ConnectionError::Utf8(err)
    }
}

impl Error for ConnectionError {

    fn description(&self) -> &str {
        match *self {
            ConnectionError::Parse(_) => "parse error",
            ConnectionError::Authorization(_) => "authorization error",
            ConnectionError::Curl(_) => "curl error",
            ConnectionError::BadResponse(_) => "bad http response",
            ConnectionError::Utf8(_) => "utf8 conversion error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectionError::Parse(ref parse_error) => Some(parse_error),
            ConnectionError::Authorization(ref auth_error) => Some(auth_error),
            ConnectionError::Curl(ref err) => Some(err),
            ConnectionError::BadResponse(_) => None,
            ConnectionError::Utf8(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::Parse(ref parse_error) => write!(f, "json parse error: {}", parse_error),
            ConnectionError::Authorization(ref auth_error) => write!(f, "authorization error: {}", auth_error),
            ConnectionError::Curl(ref err) => write!(f, "curl error: {}", err),
            ConnectionError::BadResponse(code) => write!(f, "bad http response: {}", code),
            ConnectionError::Utf8(ref err) => write!(f, "uft8 error: {}", err)
        }

    }
}
