use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;
use client::entities::localization::Localization;

#[derive(Deserialize, Debug, Clone)]
pub struct Product {
    id: u32,
    metaproduct_id: u32,
    reprint_count: u32,
    name_en: String,
    localization: Vec<Localization>,
    image_uri: String,
    products: Vec<Product>,
    website: String,


}

impl Entity for Vec<Product> {
    fn from_json(json: &str) -> Result<Vec<Product>, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
