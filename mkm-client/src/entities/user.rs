use serde_json;

use entities::entity::Entity;
use entities::entity_error::EntityError;
use entities::link::Link;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id_user: u32,
    username: String,
    registration_date: String,
    is_commercial: u8,
    is_seller: bool,
    name: Name,
    address: Address,
    phone: Option<String>,
    email: Option<String>,
    vat: Option<String>,
    risk_group: u8,
    reputation: u8,
    ships_fast: u8,
    sell_count: u32,
    sold_items: u32,
    avg_shipping_time: u32,
    on_vacation: bool,
    links: Option<Vec<Link>>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    company: Option<String>,
    first_name: String,
    last_name: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    name: Option<String>,
    extra: Option<String>,
    street: Option<String>,
    zip: Option<String>,
    city: Option<String>,
    country: String
}

impl Entity for User {
    fn from_json(json: &str) -> Result<User, EntityError> {
        Ok(serde_json::from_str(json)?)
    }
}
