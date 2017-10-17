#[macro_use]
extern crate log;
extern crate optimizer;

use optimizer::logger;
use optimizer::client::mkm_client::MKMClient;

pub fn main() {
    match logger::init() {
        Ok(_) => {},
        Err(e) => {
            println!("Could not init logger: {}", e);
            return;
        }
    }

    let mut client = match MKMClient::new("app_token.json") {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    let mp = match client.find_metaproducts("black", true) {
        Ok(mp) => mp,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };


}
