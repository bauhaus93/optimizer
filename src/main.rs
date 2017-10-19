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

    let ps = match client.find_products("Tarmogoyf", true, None, None, 0, 100) {
        Ok(p) => p,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    for p in ps {
        info!("product: name = {}, expansion_name = {}", p.get_name_en(), p.get_expansion_name());
    }


}
