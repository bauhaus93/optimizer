use std::path::Path;
use rusqlite::Connection;

use resource_error::ResourceError;
use mkm_client::mkm_client::MKMClient;


pub struct ResourceHandler {
    db_conn: Connection,
    mkm_client: MKMClient
}



impl ResourceHandler {

    pub fn new(token_path: &str, db_path: &str) -> Result<ResourceHandler, ResourceError> {
        info!("creating new resource handler");

        let path = Path::new(db_path);

        let conn = Connection::open(path)?;
        let mkm_client = MKMClient::new(token_path)?;

        let rh = ResourceHandler {
            db_conn: conn,
            mkm_client: mkm_client
        };

        Ok(rh)
    }

    pub fn download_card_data(&mut self, card_name: &str) {

    }



}
