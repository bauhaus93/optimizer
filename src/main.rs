#[macro_use]
extern crate log;
extern crate optimizer;
extern crate mkm_client;

use optimizer::logger;
use mkm_client::mkm_client::MKMClient;

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

    let articles = match client.get_articles(295893, 0, 1) {
        Ok(a) => a,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };


}
