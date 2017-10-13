use std::fs::File;
use std::fmt;

use serde_json;

use client::parse_error::ParseError;

#[derive(Deserialize, Debug)]
pub struct Token {
    app_token: String,
    app_secret: String,
    access_token: String,
    access_secret: String,
}

impl Token {

    pub fn get_app_token(&self) -> &str {
        &self.app_token
    }

    pub fn get_access_token(&self) -> &str {
        &self.access_token
    }

}

impl fmt::Display for Token {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "app_token: {}, app_secret: {}, access_token: {}, access_secret: {}",
            self.app_token,
            self.app_secret,
            self.access_token,
            self.access_secret)
    }
}

pub fn parse_app_token(path: &str) -> Result<Token, ParseError> {

    let file = try!(File::open(path));
    let token = try!(serde_json::from_reader(file));

    Ok(token)
}
