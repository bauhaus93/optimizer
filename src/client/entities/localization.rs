use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Localization {
    #[serde(rename="idLanguage")]
    language_id: String,
    #[serde(rename="languageName")]
    language_name: String,
    #[serde(rename="name")]
    name: String
}

impl Entity for Vec<Localization> {
    fn from_json(json: &str) -> Result<Vec<Localization>, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
