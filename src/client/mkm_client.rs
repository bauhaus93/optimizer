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

    pub fn find_metaproducts(&mut self, search: &str, exact: bool, game_id: Option<u32>, language_id: Option<u32>) -> Result<Vec<Metaproduct>, ClientError> {
        info!("finding metaproducts for {}, exact: {}, game_id: {:?}, language_id: {:?}", search, exact, game_id, language_id);

        let exact_str = exact.to_string();
        let game_id_str = match game_id {
            Some(id) => Some(id.to_string()),
            None => None
        };
        let language_id_str = match language_id {
            Some(id) => Some(id.to_string()),
            None => None
        };

        let mut query: Vec<(&str, &str)> = Vec::new();

        query.push(("search", search));
        query.push(("exact", &exact_str));

        match game_id_str {
            Some(ref id) => query.push(("idGame", id)),
            None => {}
        }

        match language_id_str {
            Some(ref id) => query.push(("idLanguage", id)),
            None => {}
        }

        let json_str = try!(self.connection.request("GET", "metaproducts/find", &query));
        let metaproducts = try!(Vec::<Metaproduct>::from_json(&json_str));

        info!("parsed {} metaproducts", metaproducts.len());

        Ok(metaproducts)
    }

    /*pub fn find_products(&mut self, search: &str, exact: bool, game_id: u32, language_id: u32, start: u32, max_results: u32) -> Result<Vec<Metaproduct>, ClientError> {
    }*/

}
