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

impl User {

    pub fn get_id(&self) -> u32 {
        self.id_user
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_commercial_status(&self) -> u8 {
        self.is_commercial
    }

    pub fn get_address(&self) -> &Address {
        &self.address
    }

    pub fn get_risk_group(&self) -> u8 {
        self.risk_group
    }

    pub fn get_reputation(&self) -> u8 {
        self.reputation
    }

    pub fn get_ships_fast(&self) -> u8 {
        self.ships_fast
    }

    pub fn get_on_vacation(&self) -> bool {
        self.on_vacation
    }
}

impl Address {

    pub fn get_country(&self) -> &str {
        &self.country
    }
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.id_user == other.id_user
    }
}

impl Entity for User {
    fn from_json(json: &str) -> Result<User, EntityError> {
        Ok(serde_json::from_str(json)?)
    }
}
