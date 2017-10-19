use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Localization {
    id_language: String,
    language_name: String,
    name: String
}

impl Entity for Vec<Localization> {
    fn from_json(json: &str) -> Result<Vec<Localization>, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
