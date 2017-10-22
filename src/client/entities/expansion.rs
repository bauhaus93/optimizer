use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Expansion {
    id_expansion: u32,
    en_name: String,
    expansion_icon: u32
}

impl Entity for Expansion {
    fn from_json(json: &str) -> Result<Expansion, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
