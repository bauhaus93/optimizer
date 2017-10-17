
use client::entities::entity_error::EntityError;

pub trait Entity
where Self: Sized {
    fn from_json(json: &str) -> Result<Self, EntityError>;
}
