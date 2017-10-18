use client::client_error::ClientError;
use client::connection::Connection;

use client::entities::entity::Entity;
use client::entities::metaproduct::Metaproduct;

pub struct MKMClient {
    connection: Connection
}

impl MKMClient {

    pub fn new(token_path: &str) -> Result<MKMClient, ClientError> {

        let connection = try!(Connection::new(token_path));

        let client = MKMClient {
            connection: connection
        };

        Ok(client)
    }

    pub fn find_metaproducts(&mut self, name: &str, exact: bool) -> Result<Vec<Metaproduct>, ClientError> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        query.push(("search", name));
        query.push(("exact",
            match exact {
                true => "true",
                false => "false"
            })
        );

        let json_str = try!(self.connection.request("GET", "metaproducts/find", &query));
        let metaproducts = try!(Vec::<Metaproduct>::from_json(&json_str));

        Ok(metaproducts)

    }

}
