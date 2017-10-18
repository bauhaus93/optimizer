use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;
use client::entities::localization::Localization;
use client::entities::category::Category;
use client::entities::link::Link;
use client::entities::expansion::Expansion;
use client::entities::price_guide::PriceGuide;
use client::entities::reprint::Reprint;

#[derive(Deserialize, Debug, Clone)]
pub struct Product {
    #[serde(rename="idProduct")]
    id: u32,
    #[serde(rename="idMetaproduct")]
    metaproduct_id: u32,
    #[serde(rename="countReprints")]
    reprint_count: u32,
    #[serde(rename="enName")]
    name_en: String,
    #[serde(rename="localization")]
    localization: Vec<Localization>,
    #[serde(rename="category")]
    category: Option<Category>,
    #[serde(rename="website")]
    website: String,
    #[serde(rename="image")]
    image_uri: String,
    #[serde(rename="gameName")]
    game_name: String,
    #[serde(rename="categoryName")]
    category_name: String,
    #[serde(rename="number")]
    number: Option<String>,
    #[serde(rename="rarity")]
    rarity: Option<String>,
    #[serde(rename="expansionName")]
    expansion_name: String,
    #[serde(rename="links")]
    links: Vec<Link>,
    #[serde(rename="expansion")]
    expansion: Option<Expansion>,
    #[serde(rename="priceGuide")]
    price_guide: Option<PriceGuide>,
    #[serde(rename="reprint")]
    reprint: Option<Reprint>
}

impl Entity for Product {
    fn from_json(json: &str) -> Result<Product, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
