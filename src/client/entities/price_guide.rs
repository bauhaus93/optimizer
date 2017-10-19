use serde_json;

use client::entities::entity::Entity;
use client::entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct PriceGuide {
    #[serde(rename="SELL")]
    sell: f64,
    #[serde(rename="LOW")]
    low: f64,
    #[serde(rename="LOWEX")]
    low_ex: f64,
    #[serde(rename="LOWFOIL")]
    low_foil: f64,
    #[serde(rename="AVG")]
    avg: f64,
    #[serde(rename="TREND")]
    trend: f64
}

impl PriceGuide {

    pub fn get_sell(&self) -> f64 {
        self.sell
    }

    pub fn get_low(&self) -> f64 {
        self.low
    }

    pub fn get_low_ex(&self) -> f64 {
        self.low_ex
    }

    pub fn get_low_foil(&self) -> f64 {
        self.low_foil
    }

    pub fn get_avg(&self) -> f64 {
        self.avg
    }

    pub fn get_trend(&self) -> f64 {
        self.trend
    }
}

impl Entity for PriceGuide {
    fn from_json(json: &str) -> Result<PriceGuide, EntityError> {
        Ok(try!(serde_json::from_str(json)))
    }
}
