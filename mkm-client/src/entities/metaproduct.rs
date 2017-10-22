use serde_json;

use entities::entity::Entity;
use entities::localization::Localization;
use entities::product::Product;
use entities::link::Link;
use entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metaproduct {
    metaproduct: MetaproductFields,
    product: Vec<Product>,
    links: Vec<Link>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetaproductFields {
    id_metaproduct: u32,
    en_name: String,
    loc_name: String,
    localization: Vec<Localization>,
    image: String
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
        &self.product
    }

    pub fn get_links(&self) -> &[Link] {
        &self.links
    }
}

impl MetaproductFields {

    pub fn get_id(&self) -> u32 {
        self.id_metaproduct
    }

    pub fn get_name_en(&self) -> &str {
        &self.en_name
    }

    pub fn get_name_loc(&self) -> &str {
        &self.loc_name
    }

    pub fn get_localizations(&self) -> &[Localization] {
        &self.localization
    }

    pub fn get_image_url(&self) -> &str {
        &self.image
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
