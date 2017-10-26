use std::fmt;
use std::error::Error;

use rusqlite;

use mkm_client::client_error::ClientError;

#[derive(Debug)]
pub enum ResourceError {
    Sql(rusqlite::Error),
    MkmClient(ClientError)
}

impl From<rusqlite::Error> for ResourceError {
    fn from(err: rusqlite::Error) -> ResourceError {
        ResourceError::Sql(err)
    }
}

impl From<ClientError> for ResourceError {
    fn from(err: ClientError) -> ResourceError {
        ResourceError::MkmClient(err)
    }
}

impl Error for ResourceError {

    fn description(&self) -> &str {
        match *self {
            ResourceError::Sql(_) => "sqlite error",
            ResourceError::MkmClient(_) => "mkm client error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ResourceError::Sql(ref err) => Some(err),
            ResourceError::MkmClient(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResourceError::Sql(ref err) => write!(f, "{}: {}", self.description(), err),
            ResourceError::MkmClient(ref err) => write!(f, "{}: {}", self.description(), err)
        }
    }
}
