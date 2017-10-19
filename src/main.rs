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

    let p = match client.get_product(295893) {
        Ok(p) => p,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("product: name = {}, expansion_name = , avg_price: {}", p.get_name_en(), p.get_price_guide().unwrap().get_avg());

}
