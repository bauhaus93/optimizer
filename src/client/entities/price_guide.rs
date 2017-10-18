use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct PriceGuide {
    #[serde(rename="SELL")]
    sell: f64,
    #[serde(rename="LOW")]
    low: f64,
    #[serde(rename="LOWEX+")]
    low_ex_plus: f64,
    #[serde(rename="LOWFOIL")]
    low_foil: f64,
    #[serde(rename="AVG")]
    avg: f64,
    #[serde(rename="TREND")]
    trend: f64
}

impl Entity for PriceGuide {
    fn from_json(json: &str) -> Result<PriceGuide, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
