use std::str;
use std::time::SystemTime;
use std::time;

use rand;
use rand::Rng;
use url::percent_encoding::{ utf8_percent_encode, USERINFO_ENCODE_SET, QUERY_ENCODE_SET };
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use base64;

use client::authorization_error::AuthorizationError;
use client::entities::token::Token;

const OAUTH_VERSION: &str = "1.0";
const OAUTH_SIGNATURE_METHOD: &str = "HMAC-SHA1";


pub fn create_authorization_header(method: &str, realm: &str, query: &Vec<(&str, &str)>, token: &Token) -> Result<String, AuthorizationError> {
    let timestamp = try!(SystemTime::now().duration_since(time::UNIX_EPOCH)).as_secs().to_string();
    let nonce = rand::thread_rng().gen_ascii_chars().take(32).collect::<String>().to_lowercase();
    let signature = try!(calculate_signature(method, realm, query, &token, &nonce, &timestamp));

    let header = format!("Authorization: \
        OAuth \
        realm=\"{}\", \
        oauth_version=\"{}\", \
        oauth_timestamp=\"{}\", \
        oauth_nonce=\"{}\", \
        oauth_consumer_key=\"{}\", \
        oauth_token=\"{}\", \
        oauth_signature_method=\"{}\", \
        oauth_signature=\"{}\"",
        realm,
        OAUTH_VERSION,
        timestamp,
        nonce,
        token.get_app_token(),
        token.get_access_token(),
        OAUTH_SIGNATURE_METHOD,
        signature
    );
    Ok(header)
}

fn calculate_signature(method: &str, realm: &str, query: &Vec<(&str, &str)>, token: &Token, nonce: &str, timestamp: &str) -> Result<String, AuthorizationError> {
    let base_string = format!("{}&{}&", method, utf8_percent_encode(realm, USERINFO_ENCODE_SET).to_string());
    let mut params: Vec<(&str, &str)> = query.clone();
    params.push(("oauth_consumer_key", token.get_app_token()));
    params.push(("oauth_nonce", nonce));
    params.push(("oauth_signature_method", OAUTH_SIGNATURE_METHOD));
    params.push(("oauth_timestamp", timestamp));
    params.push(("oauth_token", token.get_access_token()));
    params.push(("oauth_version", OAUTH_VERSION));
    params.sort();

    let mut param_string = String::new();

    params.iter().for_each(| &(key, value) | {
        param_string.push_str(&format!("{}%3D{}%26", key, value));
    });
    param_string.pop(); param_string.pop(); param_string.pop();

    info!("param_string: {}", param_string);

    /*let param_string= format!("\
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
    );*/

    let base_string = format!("{}{}",
        base_string,
        utf8_percent_encode(&param_string, QUERY_ENCODE_SET).to_string()
    );

    let signing_key = format!("{}&{}",
        utf8_percent_encode(token.get_app_secret(), QUERY_ENCODE_SET).to_string(),
        utf8_percent_encode(token.get_access_secret(), QUERY_ENCODE_SET).to_string()
    );

    let mut hmac = Hmac::<Sha1>::new(Sha1::new(), signing_key.as_bytes());
    hmac.input(base_string.as_bytes());
    let signature = base64::encode(hmac.result().code());

    Ok(signature)
}
