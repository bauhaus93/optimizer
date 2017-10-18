use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Localization {
    name: String,
    language_id: u32,
    language_name: String
}

impl Entity for Vec<Localization> {
    fn from_json(json: &str) -> Result<Vec<Localization>, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
