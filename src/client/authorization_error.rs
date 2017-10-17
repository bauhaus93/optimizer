use std::fmt;
use std::error::Error;
use std::time;



#[derive(Debug)]
pub enum AuthorizationError {
    StringParse,
    SysTime(time::SystemTimeError)
}

impl From<time::SystemTimeError> for AuthorizationError {
    fn from(err: time::SystemTimeError) -> AuthorizationError {
        AuthorizationError::SysTime(err)
    }
}


impl Error for AuthorizationError {

    fn description(&self) -> &str {
        match *self {
            AuthorizationError::StringParse => "string parse error",
            AuthorizationError::SysTime(_) => "system time error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AuthorizationError::StringParse => None,
            AuthorizationError::SysTime(ref err) => Some(err)
        }
    }
}

impl fmt::Display for AuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthorizationError::StringParse => write!(f, "failed to parse string"),
            AuthorizationError::SysTime(ref err) => write!(f, "failure in system time: {}", err)
        }
    }
}
