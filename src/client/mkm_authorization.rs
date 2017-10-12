use std::fmt;
use std::str::FromStr;

use hyper::header::Scheme;

#[derive(Clone, Debug)]
pub struct MKMAuthorization {
    realm: String,
    consumer_key: String,
    token: String,
}

impl Scheme for MKMAuthorization {

    fn scheme() -> Option<&'static str> {
        Some("MKM Authorization")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }

}

impl FromStr for MKMAuthorization {

    fn from_str(s: &str) -> Result<Self, Self::Err> {

    }
}
