use std::str;
use std::fs::File;
use std::io::Read;

use curl::easy;
use url::percent_encoding::{ utf8_percent_encode, QUERY_ENCODE_SET };

use client::connection_error::ConnectionError;
use client::entities::token::Token;
use client::entities::entity::Entity;
use client::authorization_header::create_authorization_header;

fn create_uri(realm: &str, query: &Vec<(&str, &str)>) -> String {
    let mut query_string = String::new();
    query_string.push_str(realm);

    if !query.is_empty() {
        query_string.push_str("?");
        query.iter().for_each(| &(key, value) | {
            query_string.push_str(&format!("{}={}&", key, utf8_percent_encode(value, QUERY_ENCODE_SET)))
        });
        query_string.pop();
    }

    query_string
}

pub struct Connection {
    handle: easy::Easy,
    token: Token
}

impl Connection {

    pub fn new(token_path: &str) -> Result<Connection, ConnectionError> {
        info!("creating new connection");

        info!("creating curl handle");
        let handle = easy::Easy::new();

        info!("retrieving app token from file \"{}\"", token_path);
        let mut file = try!(File::open(token_path));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));
        let token = try!(Token::from_json(&content));

        let connection = Connection {
            handle: handle,
            token
        };

        Ok(connection)
    }

    pub fn request(&mut self, method: &str, path: &str, query: &Vec<(&str, &str)>) -> Result<String, ConnectionError> {
        let realm = format!("https://www.mkmapi.eu/ws/v2.0/output.json/{}", path);
        let uri = create_uri(&realm, query);

        info!("requesting {} {}", method, uri);

        let auth_hdr = try!(create_authorization_header(method, &realm, query, &self.token));

        let mut list = easy::List::new();
        try!(list.append(&auth_hdr));
        try!(self.handle.url(&uri));
        try!(self.handle.http_headers(list));

        let mut buffer = Vec::new();
        {
            let mut transfer = self.handle.transfer();
            try!(transfer.write_function(| data | {
                buffer.extend_from_slice(data);
                Ok(data.len())
            }));
            try!(transfer.perform());
        }

        let response_code = try!(self.handle.response_code());

        match response_code {
            200 | 206 => {  //206 = partial content, is returned when limiting search results
                info!("response code {}", response_code);
                info!("bytes read: {}", buffer.len());
                Ok(try!(str::from_utf8(&buffer)).to_string())
            },
            207 => {    //207 = no content
                info!("response code {}", response_code);
                info!("no content");
                Ok(String::new())
            },
            _ => Err(ConnectionError::BadResponse(response_code))
        }
    }


}
