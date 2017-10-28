use serde_json;

use entities::entity::Entity;
use entities::entity_error::EntityError;
use entities::language::Language;
use entities::product::ProductShort;
use entities::user::User;
use entities::link::Link;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    id_article: u32,
    id_product: u32,
    language: Language,
    comments: String,
    price: f64,
    count: u32,
    in_shopping_cart: bool,
    product: Option<ProductShort>,
    seller: User,
    last_edited: Option<String>,
    condition: Option<String>,
    is_foil: Option<bool>,
    is_signed: Option<bool>,
    is_altered: Option<bool>,
    is_playset: Option<bool>,
    is_first_ed: Option<bool>,
    links: Vec<Link>
}

impl Article {

    pub fn get_id(&self) -> u32 {
        self.id_article
    }

    pub fn get_product_id(&self) -> u32 {
        self.id_product
    }

    pub fn get_language(&self) -> &Language {
        &self.language
    }

    pub fn get_comments(&self) -> &str {
        &self.comments
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn get_in_shopping_cart(&self) -> bool {
        self.in_shopping_cart
    }

    pub fn get_seller(&self) -> &User {
        &self.seller
    }

    pub fn get_condition(&self) -> Option<&str> {
        match self.condition {
            Some(ref cond) => Some(cond),
            None => None
        }
    }

    pub fn is_foil(&self) -> Option<bool> {
        self.is_foil
    }

    pub fn is_signed(&self) -> Option<bool> {
        self.is_signed
    }

    pub fn is_altered(&self) -> Option<bool> {
        self.is_altered
    }

}

#[derive(Deserialize, Debug, Clone)]
struct Articles {
    #[serde(rename="article")]
    articles: Vec<Article>,
    links: Vec<Link>
}

impl Articles {
    pub fn consume(self) -> Vec<Article> {
        self.articles
    }
}

impl Entity for Vec<Article> {
    fn from_json(json: &str) -> Result<Vec<Article>, EntityError> {
        let articles: Articles = serde_json::from_str(json)?;
        Ok(articles.consume())
    }
}
