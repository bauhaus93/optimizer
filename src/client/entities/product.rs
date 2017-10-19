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
    image_url: String,
    #[serde(rename="gameName")]
    game_name: String,
    #[serde(rename="categoryName")]
    category_name: String,
    #[serde(rename="number")]
    number: Option<String>,
    #[serde(rename="rarity")]
    rarity: Option<String>,
    #[serde(rename="expansionName")]
    expansion_name: Option<String>,
    #[serde(rename="links")]
    links: Vec<Link>,
    #[serde(rename="expansion")]
    expansion: Option<Expansion>,
    #[serde(rename="priceGuide")]
    price_guide: Option<PriceGuide>,
    #[serde(rename="reprint")]
    reprints: Option<Vec<Reprint>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProductShort {
    #[serde(rename="enName")]
    name_en: String,
    #[serde(rename="locName")]
    name_loc: String,
    #[serde(rename="image")]
    image_url: String,
    #[serde(rename="expansionName")]
    expansion_name: Option<String>,
    #[serde(rename="number")]
    number: Option<String>,
    #[serde(rename="expIcon")]
    icon_id: u32,
    #[serde(rename="rarity")]
    rarity: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
struct Products {
    #[serde(rename="product")]
    products: Vec<Product>
}

#[derive(Deserialize, Debug, Clone)]
struct ProductContainer {
    #[serde(rename="product")]
    product: Product
}

impl Product {

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_metaproduct_id(&self) -> u32 {
        self.metaproduct_id
    }

    pub fn get_reprint_count(&self) -> u32 {
        self.reprint_count
    }

    pub fn get_name_en(&self) -> &str {
        &self.name_en
    }

    pub fn get_localizations(&self) -> &[Localization] {
        &self.localization
    }

    pub fn get_category(&self) -> Option<&Category> {
        match self.category {
            Some(ref cat) => Some(cat),
            None => None
        }
    }

    pub fn get_website(&self) -> &str {
        &self.website
    }

    pub fn get_image_url(&self) -> &str {
        &self.image_url
    }

    pub fn get_game_name(&self) -> &str {
        &self.game_name
    }

    pub fn get_category_name(&self) -> &str {
        &self.category_name
    }

    pub fn get_number(&self) -> Option<&str> {
        match self.number {
            Some(ref num) => Some(num),
            None => None
        }
    }

    pub fn get_rarity(&self) -> Option<&str> {
        match self.rarity {
            Some(ref rarity) => Some(rarity),
            None => None
        }
    }

    pub fn get_expansion_name(&self) -> Option<&str> {
        match self.expansion_name {
            Some(ref name) => Some(name),
            None => None
        }
    }

    pub fn get_links(&self) -> &[Link] {
        &self.links
    }

    pub fn get_expansion(&self) -> Option<&Expansion> {
        match self.expansion {
            Some(ref exp) => Some(exp),
            None => None
        }
    }

    pub fn get_price_guide(&self) -> Option<&PriceGuide> {
        match self.price_guide {
            Some(ref pg) => Some(pg),
            None => None
        }
    }

    pub fn get_reprints(&self) -> Option<&[Reprint]> {
        match self.reprints {
            Some(ref rps) => Some(rps),
            None => None
        }
    }

}

impl Products {
    pub fn consume(self) -> Vec<Product> {
        self.products
    }
}

impl ProductContainer {
    pub fn consume(self) -> Product {
        self.product
    }
}

impl Entity for Product {
    fn from_json(json: &str) -> Result<Product, EntityError> {
        let p: ProductContainer = try!(serde_json::from_str(json));
        Ok(p.consume())
    }
}

impl Entity for Vec<Product> {
    fn from_json(json: &str) -> Result<Vec<Product>, EntityError> {
        let ps: Products = try!(serde_json::from_str(json));
        Ok(ps.consume())
    }
}

impl Entity for ProductShort {
    fn from_json(json: &str) -> Result<ProductShort, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
