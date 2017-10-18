use serde_json;

use client::entities::entity::Entity;
use client::entities::localization::Localization;
use client::entities::product::Product;
use client::entities::link::Link;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename="metaproduct")]
pub struct Metaproduct {
    #[serde(rename="idMetaproduct")]
    id: u32,
    #[serde(rename="enName")]
    name_en: String,
    #[serde(rename="locName")]
    name_loc: String,
    #[serde(rename="localization")]
    localization: Vec<Localization>,
    #[serde(rename="image")]
    image_url: String,
    #[serde(rename="product")]
    products: Vec<Product>,
    #[serde(rename="links")]
    links: Vec<Link>
}

impl Entity for Metaproduct {
    fn from_json(json: &str) -> Result<Metaproduct, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}

impl Entity for Vec<Metaproduct> {
    fn from_json(json: &str) -> Result<Vec<Metaproduct>, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
