use std::error::Error;
use std::fmt;

use serde_json;

#[derive(Debug)]
pub enum EntityError {
    Json(serde_json::Error)
}

impl From<serde_json::Error> for EntityError {
    fn from(err: serde_json::Error) -> EntityError {
        EntityError::Json(err)
    }
}

impl Error for EntityError {

    fn description(&self) -> &str {
        match *self {
            EntityError::Json(_) => "json parse error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            EntityError::Json(ref json_error) => Some(json_error)
        }
    }
}

impl fmt::Display for EntityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EntityError::Json(ref json_error) => write!(f, "json parse error: {}", json_error)
        }
    }
}
