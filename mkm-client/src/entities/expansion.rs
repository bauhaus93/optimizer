use serde_json;

use entities::entity::Entity;
use entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Expansion {
    id_expansion: u32,
    en_name: String,
    expansion_icon: u32
}

impl Entity for Expansion {
    fn from_json(json: &str) -> Result<Expansion, EntityError> {
        Ok(serde_json::from_str(json)?)
    }
}
