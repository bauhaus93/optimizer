use serde_json;

use entities::entity::Entity;
use entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reprint {
    id_product: u32,
    expansion: String,
    exp_icon: u32
}

impl Entity for Reprint {
    fn from_json(json: &str) -> Result<Reprint, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
