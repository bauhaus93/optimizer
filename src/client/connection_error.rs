use std::error::Error;
use std::fmt;
use std::io;

use hyper;
use hyper_tls;

use client::parse_error::ParseError;
use client::authorization_error::AuthorizationError;

#[derive(Debug)]
pub enum ConnectionError {
    Parse(ParseError),
    Authorization(AuthorizationError),
    TokioCore(io::Error),
    HyperUri(hyper::error::UriError),
    Hyper(hyper::Error),
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

impl From<io::Error> for ConnectionError {
    fn from(err: io::Error) -> ConnectionError {
        ConnectionError::TokioCore(err)
    }
}

impl From<hyper::error::UriError> for ConnectionError {
    fn from(err: hyper::error::UriError) -> ConnectionError {
        ConnectionError::HyperUri(err)
    }
}

impl From<hyper::Error> for ConnectionError {
    fn from(err: hyper::Error) -> ConnectionError {
        ConnectionError::Hyper(err)
    }
}

impl Error for ConnectionError {

    fn description(&self) -> &str {
        match *self {
            ConnectionError::Parse(_) => "parse error",
            ConnectionError::Authorization(_) => "authorization error",
            ConnectionError::TokioCore(_) => "tokio core error",
            ConnectionError::HyperUri(_) => "hyper uri error",
            ConnectionError::Hyper(_) => "hyper error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectionError::Parse(ref parse_error) => Some(parse_error),
            ConnectionError::Authorization(ref auth_error) => Some(auth_error),
            ConnectionError::TokioCore(ref io_error) => Some(io_error),
            ConnectionError::HyperUri(ref uri_error) => Some(uri_error),
            ConnectionError::Hyper(ref hyper_error) => Some(hyper_error)
        }
    }
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::Parse(ref parse_error) => write!(f, "failed to parse json: {}", parse_error),
            ConnectionError::Authorization(ref auth_error) => write!(f, "authorization failure: {}", auth_error),
            ConnectionError::TokioCore(ref io_error) => write!(f, "tokio core io error: {}", io_error),
            ConnectionError::HyperUri(ref uri_error) => write!(f, "hyper uri error: {}", uri_error),
            ConnectionError::Hyper(ref hyper_error) => write!(f, "hyper error: {}", hyper_error)
        }

    }
}
