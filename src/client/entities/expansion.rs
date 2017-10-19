use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Expansion {
    #[serde(rename="idExpansion")]
    id: u32,
    #[serde(rename="enName")]
    name_en: String,
    #[serde(rename="expansionIcon")]
    icon_id: u32
}

impl Entity for Expansion {
    fn from_json(json: &str) -> Result<Expansion, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
