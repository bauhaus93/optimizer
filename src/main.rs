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

    let mps = match client.find_metaproducts("Tarmogoyf", true, None, None) {
        Ok(mp) => mp,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    for mp in mps {
        info!("metaproduct: name = {}, id = {}", mp.get_metaproduct().get_name_en(), mp.get_metaproduct().get_id());
    }


}
