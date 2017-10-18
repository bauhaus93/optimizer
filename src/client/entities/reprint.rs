use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Reprint {
    #[serde(rename="idProduct")]
    product_id: u32,
    #[serde(rename="expansion")]
    expansion_name: String,
    #[serde(rename="expansionIcon")]
    icon_url: String
}

impl Entity for Reprint {
    fn from_json(json: &str) -> Result<Reprint, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
