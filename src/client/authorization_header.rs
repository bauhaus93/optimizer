use std::fmt;
use std::str;
use std::time::SystemTime;
use std::time;

use hyper::header::Scheme;
use rand;
use rand::Rng;
use url::percent_encoding::{ utf8_percent_encode, USERINFO_ENCODE_SET, QUERY_ENCODE_SET};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use base64;

use client::authorization_error::AuthorizationError;
use client::token::Token;

#[derive(Clone, Debug)]
pub struct AuthorizationHeader {
    realm: String,
    token: Token,
    timestamp: String,
    nonce: String,
    signature: String
}

const OAUTH_VERSION: &str = "1.0";
const OAUTH_SIGNATURE_METHOD: &str = "HMAC-SHA1";

fn calculate_signature(realm: &str, token: &Token, nonce: &str, timestamp: &str) -> Result<String, AuthorizationError> {
    let base_string = format!("GET&{}&", utf8_percent_encode(realm, USERINFO_ENCODE_SET).to_string());

    let param_string= format!("\
        oauth_consumer_key%3D{}%26\
        oauth_nonce%3D{}%26\
        oauth_signature_method%3D{}%26\
        oauth_timestamp%3D{}%26\
        oauth_token%3D{}%26\
        oauth_version%3D{}",
        token.get_app_token(),
        nonce,
        OAUTH_SIGNATURE_METHOD,
        timestamp,
        token.get_access_token(),
        OAUTH_VERSION
    );

    let base_string = format!("{}{}",
        base_string,
        utf8_percent_encode(&param_string, QUERY_ENCODE_SET).to_string()
    );

    let signing_key = format!("{}&{}",
        utf8_percent_encode(token.get_app_secret(), QUERY_ENCODE_SET).to_string(),
        utf8_percent_encode(token.get_access_secret(), QUERY_ENCODE_SET).to_string()
    );

    info!("signing_key: {}", signing_key);
    info!("base_string: {}", base_string);

    let mut hmac = Hmac::<Sha1>::new(Sha1::new(), signing_key.as_bytes());
    hmac.input(base_string.as_bytes());
    let signature = base64::encode(hmac.result().code());

    Ok(signature)
}



impl AuthorizationHeader {

    pub fn new(token: Token, realm: &str) -> Result<AuthorizationHeader, AuthorizationError> {
        let timestamp = try!(SystemTime::now().duration_since(time::UNIX_EPOCH)).as_secs().to_string();
        let nonce = format!("{:x}", rand::thread_rng().next_u64());
        let signature = try!(calculate_signature(realm, &token, &nonce, &timestamp));

        info!("signature: {}", signature);

        let header = AuthorizationHeader {
            realm: realm.to_owned(),
            token: token,
            timestamp: timestamp,
            nonce: nonce,
            signature: signature
        };
        Ok(header)
    }

}

impl Scheme for AuthorizationHeader {

    fn scheme() -> Option<&'static str> {
        Some("OAuth")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
            realm=\"{}\", \
            oauth_version=\"{}\", \
            oauth_timestamp=\"{}\", \
            oauth_nonce=\"{}\", \
            oauth_consumer_key=\"{}\", \
            oauth_token=\"{}\", \
            oauth_signature_method=\"{}\", \
            oauth_signature\"{}\"",
            self.realm,
            OAUTH_VERSION,
            self.timestamp,
            self.nonce,
            self.token.get_app_token(),
            self.token.get_access_token(),
            OAUTH_SIGNATURE_METHOD,
            self.signature
        )
    }
}

impl str::FromStr for AuthorizationHeader {
    type Err = AuthorizationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = &mut s.split(',');

        let realm = match parts.next() {
            Some(part) => part.to_owned(),
            None => return Err(AuthorizationError::StringParse)
        };

        let app_token = match parts.next() {
            Some(part) => part.to_owned(),
            None => return Err(AuthorizationError::StringParse)
        };

        let app_secret = match parts.next() {
            Some(part) => part.to_owned(),
            None => return Err(AuthorizationError::StringParse)
        };

        let access_token = match parts.next() {
            Some(part) => part.to_owned(),
            None => return Err(AuthorizationError::StringParse)
        };

        let access_secret = match parts.next() {
            Some(part) => part.to_owned(),
            None => return Err(AuthorizationError::StringParse)
        };

        let token = Token::new(&app_token, &app_secret, &access_token, &access_secret);

        let timestamp = try!(SystemTime::now().duration_since(time::UNIX_EPOCH)).as_secs().to_string();
        let nonce = format!("{:x}", rand::thread_rng().next_u64());
        let signature = try!(calculate_signature(&realm, &token, &nonce, &timestamp));

        let header = AuthorizationHeader {
            realm: realm,
            token: token,
            timestamp: timestamp,
            nonce: nonce,
            signature: signature
        };

        Ok(header)
    }
}

impl fmt::Display for AuthorizationHeader {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "realm={}, \
            oauth_consumer_key={}, \
            oauth_nonce={}, \
            oauth_signature_method={}, \
            oauth_timestamp={}, \
            oauth_token={}, \
            oauth_version={}, \
            oauth_signature={}",
            self.realm,
            self.token.get_app_token(),
            self.nonce,
            OAUTH_SIGNATURE_METHOD,
            self.timestamp,
            self.token.get_access_token(),
            OAUTH_VERSION,
            self.signature)

    }
}
