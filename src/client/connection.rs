use curl::easy;

use client::connection_error::ConnectionError;
use client::token;
use client::authorization_header::create_authorization_header;

pub struct Connection {
    handle: easy::Easy,
    token: token::Token
}

impl Connection {

    pub fn new(token_path: &str) -> Result<Connection, ConnectionError> {
        info!("creating new connection");

        info!("creating curl handle");
        let handle = easy::Easy::new();

        info!("retrieving app token from file \"{}\"", token_path);
        let token = try!(token::parse_app_token(token_path));

        let connection = Connection {
            handle: handle,
            token
        };

        Ok(connection)
    }

    pub fn request(&mut self, method: &str, path: &str) -> Result<String, ConnectionError> {
        let realm = format!("https://www.mkmapi.eu{}", path);

        info!("requesting {} {}", method, realm);

        let auth_hdr = try!(create_authorization_header(method, &realm, &self.token));

        let mut list = easy::List::new();
        try!(list.append(&auth_hdr));
        try!(self.handle.url(&realm));
        try!(self.handle.http_headers(list));
        try!(self.handle.perform());

        let response_code = try!(self.handle.response_code());
        info!("response code {}", response_code);

        if response_code != 200 {
            return Err(ConnectionError::BadResponse(response_code));
        }

        Ok(" ".to_owned())
    }


}
