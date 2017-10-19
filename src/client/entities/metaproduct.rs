use serde_json;

use client::entities::entity::Entity;
use client::entities::localization::Localization;
use client::entities::product::Product;
use client::entities::link::Link;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Metaproduct {
    #[serde(rename="metaproduct")]
    metaproduct: MetaproductFields,
    #[serde(rename="product")]
    products: Vec<Product>,
    #[serde(rename="links")]
    links: Vec<Link>
}

#[derive(Deserialize, Debug, Clone)]
pub struct MetaproductFields {
    #[serde(rename="idMetaproduct")]
    id: u32,
    #[serde(rename="enName")]
    name_en: String,
    #[serde(rename="locName")]
    name_loc: String,
    #[serde(rename="localization")]
    localization: Vec<Localization>,
    #[serde(rename="image")]
    image_url: String
}

#[derive(Deserialize, Debug, Clone)]
struct Metaproducts {
    #[serde(rename="metaproduct")]
    metaproducts: Vec<Metaproduct>
}

impl Metaproduct {
    pub fn get_metaproduct(&self) -> &MetaproductFields {
        &self.metaproduct
    }

    pub fn get_products(&self) -> &[Product] {
        &self.products
    }

    pub fn get_links(&self) -> &[Link] {
        &self.links
    }
}

impl MetaproductFields {

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name_en(&self) -> &str {
        &self.name_en
    }

    pub fn get_name_loc(&self) -> &str {
        &self.name_loc
    }

    pub fn get_localizations(&self) -> &[Localization] {
        &self.localization
    }

    pub fn get_image_url(&self) -> &str {
        &self.image_url
    }
}

impl Metaproducts {
    pub fn consume(self) -> Vec<Metaproduct> {
        self.metaproducts
    }
}

impl Entity for Metaproduct {
    fn from_json(json: &str) -> Result<Metaproduct, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}

impl Entity for Vec<Metaproduct> {
    fn from_json(json: &str) -> Result<Vec<Metaproduct>, EntityError> {
        let mps: Metaproducts = try!(serde_json::from_str(json));
        Ok(mps.consume())
    }
}
