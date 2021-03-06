use std::fmt;

use serde_json;

use entities::entity::Entity;
use entities::entity_error::EntityError;

#[derive(Deserialize, Debug, Clone)]
pub struct Token {
    app_token: String,
    app_secret: String,
    access_token: String,
    access_secret: String,
}

impl Token {

    pub fn get_app_token(&self) -> &str {
        &self.app_token
    }

    pub fn get_app_secret(&self) -> &str {
        &self.app_secret
    }

    pub fn get_access_token(&self) -> &str {
        &self.access_token
    }

    pub fn get_access_secret(&self) -> &str {
        &self.access_secret
    }

}

impl fmt::Display for Token {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "app_token: {}, app_secret: {}, access_token: {}, access_secret: {}",
            self.app_token,
            self.app_secret,
            self.access_token,
            self.access_secret)
    }
}

impl Entity for Token {
    fn from_json(json: &str) -> Result<Token, EntityError> {
        let token = serde_json::from_str(json)?;
        Ok(token)
    }
}
