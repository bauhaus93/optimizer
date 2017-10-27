#[macro_use]
extern crate log;
extern crate optimizer;
extern crate mkm_client;

use optimizer::logger;
use mkm_client::mkm_client::MKMClient;
use mkm_client::query::query_builder::QueryBuilder;
use optimizer::resource_handler::ResourceHandler;

pub fn main() {
    match logger::init() {
        Ok(_) => {},
        Err(e) => {
            println!("Could not init logger: {}", e);
            return;
        }
    }

    /*let mut client = match MKMClient::new("app_token.json") {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    let query = QueryBuilder::new()
        .name("Tarmogoyf")
        .finalize();

    let mp = client.find_metaproducts(query);*/

    let mut rh = match ResourceHandler::new("app_token.json", "test.db") {
        Ok(rh) => rh,
        Err(e) => {
            error!("{}", e);
            return
        }
    };

    rh.download_card_data("Tarmogoyf");


}
