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
#[serde(rename_all = "snake_case")]
pub struct Product {
    id_product: u32,
    id_metaproduct: u32,
    count_reprints: u32,
    en_name: String,
    localization: Vec<Localization>,
    category: Option<Category>,
    website: String,
    image: String,
    game_name: String,
    category_name: String,
    number: Option<String>,
    rarity: Option<String>,
    expansion_name: Option<String>,
    links: Vec<Link>,
    expansion: Option<Expansion>,
    price_guide: Option<PriceGuide>,
    reprint: Option<Vec<Reprint>>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ProductShort {
    en_name: String,
    loc_name: String,
    image: String,
    expansion_name: Option<String>,
    number: Option<String>,
    exp_icon: u32,
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
        self.id_product
    }

    pub fn get_metaproduct_id(&self) -> u32 {
        self.id_metaproduct
    }

    pub fn get_reprint_count(&self) -> u32 {
        self.count_reprints
    }

    pub fn get_name_en(&self) -> &str {
        &self.en_name
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
        &self.image
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
        match self.reprint {
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
