use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    #[serde(rename="idCategory")]
    id: u32,
    #[serde(rename="categoryName")]
    name: String,
}

impl Entity for Vec<Category> {
    fn from_json(json: &str) -> Result<Vec<Category>, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
