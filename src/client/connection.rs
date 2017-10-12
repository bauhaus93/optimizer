use std::fs::File;
use std::fmt;

use hyper::client::Client;


use serde_json;

use client::parse_error::ParseError;
use client::connection_error::ConnectionError;

#[derive(Deserialize, Debug)]
pub struct Token {
    app_token: String,
    app_secret: String,
    access_token: String,
    access_secret: String,
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

pub struct Connection {
}


fn parse_app_token(path: &str) -> Result<Token, ParseError> {

    let file = try!(File::open(path));
    let token = try!(serde_json::from_reader(file));

    Ok(token)
}


impl Connection {

    pub fn new(token_path: &str) -> Result<Connection, ConnectionError> {

        info!("Retrieving app token from file \"{}\"", token_path);
        let token = try!(parse_app_token(token_path));

        info!("Parsed token: {},", token);

        Ok(Connection {})
    }


}
