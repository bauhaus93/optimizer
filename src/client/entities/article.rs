use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;
use client::entities::language::Language;
use client::entities::product::ProductShort;
use client::entities::user::User;
use client::entities::link::Link;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
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
    last_edited: String,
    condition: String,
    is_foil: bool,
    is_signed: bool,
    is_altered: bool,
    is_playset: bool,
    is_first_ed: bool,
    links: Vec<Link>
}

#[derive(Deserialize, Debug, Clone)]
struct Articles {
    #[serde(rename="article")]
    articles: Vec<Article>
}

impl Articles {
    pub fn consume(self) -> Vec<Article> {
        self.articles
    }
}

impl Entity for Vec<Article> {
    fn from_json(json: &str) -> Result<Vec<Article>, EntityError> {
        let articles: Articles = try!(serde_json::from_str(json));
        Ok(articles.consume())
    }
}
