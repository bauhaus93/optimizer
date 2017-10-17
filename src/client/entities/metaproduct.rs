
use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Metaproduct {

}

impl Metaproduct {

    pub fn new() -> Metaproduct {
        Metaproduct {

        }
    }
}

impl Entity for Vec<Metaproduct> {
    fn from_json(json: &str) -> Result<Vec<Metaproduct>, EntityError> {
        let metaproduct = try!(serde_json::from_str(json));
        Ok(metaproduct)
    }

}
