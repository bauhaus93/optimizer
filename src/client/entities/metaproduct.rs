
use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Metaproduct {

}

impl Entity for Vec<Metaproduct> {
    fn from_json(json: &str) -> Result<Vec<Metaproduct>, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
