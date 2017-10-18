use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Link {
    #[serde(rename="rel")]
    rel: String,
    #[serde(rename="href")]
    href: String,
    #[serde(rename="method")]
    method: String
}

impl Entity for Link {
    fn from_json(json: &str) -> Result<Link, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
