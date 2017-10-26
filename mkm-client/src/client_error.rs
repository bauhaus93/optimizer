use std::error::Error;
use std::fmt;
use std::io;
use std::str;

use curl;

use entities::entity_error::EntityError;
use authorization_error::AuthorizationError;

#[derive(Debug)]
pub enum ClientError {
    Io(io::Error),
    Authorization(AuthorizationError),
    Curl(curl::Error),
    BadResponse(u32),
    Utf8(str::Utf8Error),
    Entity(EntityError),
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> ClientError {
        ClientError::Io(err)
    }
}

impl From<AuthorizationError> for ClientError {
    fn from(err: AuthorizationError) -> ClientError {
        ClientError::Authorization(err)
    }
}

impl From<curl::Error> for ClientError {
    fn from(err: curl::Error) -> ClientError {
        ClientError::Curl(err)
    }
}

impl From<str::Utf8Error> for ClientError {
    fn from(err: str::Utf8Error) -> ClientError {
        ClientError::Utf8(err)
    }
}

impl From<EntityError> for ClientError {
    fn from(err: EntityError) -> ClientError {
        ClientError::Entity(err)
    }
}

impl Error for ClientError {

    fn description(&self) -> &str {
        match *self {
            ClientError::Io(_) => "io error",
            ClientError::Authorization(_) => "authorization error",
            ClientError::Curl(_) => "curl error",
            ClientError::BadResponse(_) => "bad http response",
            ClientError::Utf8(_) => "utf8 conversion error",
            ClientError::Entity(_) => "entity error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ClientError::Io(ref io_error) => Some(io_error),
            ClientError::Authorization(ref auth_error) => Some(auth_error),
            ClientError::Curl(ref err) => Some(err),
            ClientError::BadResponse(_) => None,
            ClientError::Utf8(ref err) => Some(err),
            ClientError::Entity(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientError::Io(ref io_error) => write!(f, "{}: {}", self.description(), io_error),
            ClientError::Authorization(ref auth_error) => write!(f, "{}: {}", self.description(), auth_error),
            ClientError::Curl(ref err) => write!(f, "c{}: {}", self.description(), err),
            ClientError::BadResponse(code) => write!(f, "{}: {}", self.description(), code),
            ClientError::Utf8(ref err) => write!(f, "{}: {}", self.description(), err),
            ClientError::Entity(ref err) => write!(f, "{}: {}", self.description(), err)
        }

    }
}
