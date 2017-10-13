
use client::connection_error::ConnectionError;
use client::token;

pub struct Connection {
}



impl Connection {

    pub fn new(token_path: &str) -> Result<Connection, ConnectionError> {

        info!("Retrieving app token from file \"{}\"", token_path);
        let token = try!(token::parse_app_token(token_path));

        info!("Parsed token: {},", token);

        Ok(Connection {})
    }


}
