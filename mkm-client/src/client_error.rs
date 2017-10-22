use std::error::Error;
use std::fmt;

use connection_error::ConnectionError;
use entities::entity_error::EntityError;

#[derive(Debug)]
pub enum ClientError {
    Connection(ConnectionError),
    Entity(EntityError)
}

impl From<ConnectionError> for ClientError {
    fn from(err: ConnectionError) -> ClientError {
        ClientError::Connection(err)
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
            ClientError::Connection(_) => "connection error",
            ClientError::Entity(_) => "entity error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ClientError::Connection(ref connection_error) => Some(connection_error),
            ClientError::Entity(ref entity_error) => Some(entity_error)
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientError::Connection(ref connection_error) => write!(f, "connection error: {}", connection_error),
            ClientError::Entity(ref entity_error) => write!(f, "entity error: {}", entity_error)
        }

    }
}
