use std::fmt;
use std::str;
use std::time::SystemTime;
use std::time;

use hyper::header::Scheme;
use rand;
use rand::Rng;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

use client::authorization_header_error::AuthorizationHeaderError;
use client::token::Token;

#[derive(Clone, Debug)]
pub struct AuthorizationHeader {
    realm: String,
    consumer_key: String,
    token: String,
    query: Vec<(String, String)>
}

impl AuthorizationHeader {

    pub fn new(token: &Token, realm: &str, query: Vec<(String, String)>) -> Result<AuthorizationHeader, AuthorizationHeaderError> {
        let header = AuthorizationHeader {
            realm: realm.to_owned(),
            consumer_key: token.get_app_token().to_owned(),
            token: token.get_access_token().to_owned(),
            query: query
        };
        Ok(header)
    }

}

impl AuthorizationHeader {

    fn calculate_signature(&self) -> String {
        let mut base_string = String::new();

        base_string.push_str("GET&");
        base_string.push_str(utf8_percent_encode(self.realm, DEFAULT_ENCODE_SET));


        let timestamp = SystemTime::now().duration_since(time::UNIX_EPOCH);
        let nonce = format!("{:X}", rand::thread_rng().next_u64());


        " ".to_owned()
    }
}

impl Scheme for AuthorizationHeader {

    fn scheme() -> Option<&'static str> {
        Some("MKM Authorization")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "")
    }
}

impl str::FromStr for AuthorizationHeader {
    type Err = AuthorizationHeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = &mut s.split(',');

        let realm = match parts.next() {
            Some(part) => part.to_owned(),
            None => return Err(AuthorizationHeaderError::Conversion)
        };

        let consumer_key = match parts.next() {
            Some(part) => part.to_owned(),
            None => return Err(AuthorizationHeaderError::Conversion)
        };

        let token = match parts.next() {
            Some(part) => part.to_owned(),
            None => return Err(AuthorizationHeaderError::Conversion)
        };

        let header = AuthorizationHeader {
            realm: realm,
            consumer_key: consumer_key,
            token: token
        };

        Ok(header)
    }
}
