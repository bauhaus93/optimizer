use serde_json;

use entities::entity::Entity;
use entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
