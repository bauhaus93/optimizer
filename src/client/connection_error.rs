use std::error::Error;
use std::fmt;
use std::str;
use std::io;

use curl;

use client::authorization_error::AuthorizationError;
use client::entities::entity_error::EntityError;

#[derive(Debug)]
pub enum ConnectionError {
    Io(io::Error),
    Authorization(AuthorizationError),
    Curl(curl::Error),
    BadResponse(u32),
    Utf8(str::Utf8Error),
    Entity(EntityError)
}

impl From<io::Error> for ConnectionError {
    fn from(err: io::Error) -> ConnectionError {
        ConnectionError::Io(err)
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

impl From<EntityError> for ConnectionError {
    fn from(err: EntityError) -> ConnectionError {
        ConnectionError::Entity(err)
    }
}

impl Error for ConnectionError {

    fn description(&self) -> &str {
        match *self {
            ConnectionError::Io(_) => "io error",
            ConnectionError::Authorization(_) => "authorization error",
            ConnectionError::Curl(_) => "curl error",
            ConnectionError::BadResponse(_) => "bad http response",
            ConnectionError::Utf8(_) => "utf8 conversion error",
            ConnectionError::Entity(_) => "entity error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectionError::Io(ref io_error) => Some(io_error),
            ConnectionError::Authorization(ref auth_error) => Some(auth_error),
            ConnectionError::Curl(ref err) => Some(err),
            ConnectionError::BadResponse(_) => None,
            ConnectionError::Utf8(ref err) => Some(err),
            ConnectionError::Entity(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::Io(ref io_error) => write!(f, "io error: {}", io_error),
            ConnectionError::Authorization(ref auth_error) => write!(f, "authorization error: {}", auth_error),
            ConnectionError::Curl(ref err) => write!(f, "curl error: {}", err),
            ConnectionError::BadResponse(code) => write!(f, "bad http response: {}", code),
            ConnectionError::Utf8(ref err) => write!(f, "utf8 error: {}", err),
            ConnectionError::Entity(ref err) => write!(f, "entity error: {}", err)
        }

    }
}
