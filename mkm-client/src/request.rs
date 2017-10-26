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

use authorization_error::AuthorizationError;
use token::Token;
use query::Query;

const OAUTH_VERSION: &str = "1.0";
const OAUTH_SIGNATURE_METHOD: &str = "HMAC-SHA1";

pub struct Request<'a>{
    method: &'a str,
    uri: String,
    query: Query,
    oauth_header: String
}

impl<'a> Request<'a> {

    pub fn new(method: &'a str, path: &str, query: Query, token: &Token) -> Result<Request<'a>, AuthorizationError> {
        info!("creating request for {}, query: {}", path, query);
        let realm = format!("https://www.mkmapi.eu/ws/v2.0/output.json/{}", path);
        let uri = realm.clone() + &query.create_query_string();

        let oauth_header = create_authorization_header(method, &realm, &query, &token)?;

        let request = Request {
            method: method,
            uri: uri,
            query: query,
            oauth_header: oauth_header
        };

        Ok(request)
    }

    pub fn get_method(&self) -> &'a str {
        self.method
    }

    pub fn get_uri(&self) -> &str {
        &self.uri
    }

    pub fn get_oauth_header(&self) -> &str {
        &self.oauth_header
    }

}

fn create_authorization_header(method: &str, realm: &str, query: &Query, token: &Token) -> Result<String, AuthorizationError> {
    let timestamp = SystemTime::now().duration_since(time::UNIX_EPOCH)?.as_secs().to_string();
    let nonce = rand::thread_rng().gen_ascii_chars().take(32).collect::<String>().to_lowercase();
    let signature = calculate_signature(method, realm, query, token, &nonce, &timestamp)?;

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

fn calculate_signature(method: &str, realm: &str, query: &Query, token: &Token, nonce: &str, timestamp: &str) -> Result<String, AuthorizationError> {
    let base_string = format!("{}&{}&", method, utf8_percent_encode(realm, USERINFO_ENCODE_SET).to_string());
    let mut params: Vec<(&str, String)> = query.get_elements().to_vec();
    params.push(("oauth_consumer_key", token.get_app_token().to_owned()));
    params.push(("oauth_nonce", nonce.to_owned()));
    params.push(("oauth_signature_method", OAUTH_SIGNATURE_METHOD.to_owned()));
    params.push(("oauth_timestamp", timestamp.to_owned()));
    params.push(("oauth_token", token.get_access_token().to_owned()));
    params.push(("oauth_version", OAUTH_VERSION.to_owned()));
    params.sort();

    let mut param_string = String::new();

    params.iter().for_each(| &(key, ref value) | {
        param_string.push_str(&format!("{}%3D{}%26", key, value));
    });
    param_string.pop(); param_string.pop(); param_string.pop();

    debug!("oauth param_string: {}", param_string);

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
