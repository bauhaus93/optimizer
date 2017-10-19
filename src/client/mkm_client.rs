use client::client_error::ClientError;
use client::connection::Connection;

use client::entities::entity::Entity;
use client::entities::metaproduct::Metaproduct;
use client::entities::product::Product;

pub struct MKMClient {
    connection: Connection
}

impl MKMClient {

    pub fn new(token_path: &str) -> Result<MKMClient, ClientError> {

        let connection = try!(Connection::new(token_path));

        let client = MKMClient {
            connection: connection
        };

        Ok(client)
    }

    pub fn find_metaproducts(&mut self, search: &str, exact: bool, game_id: Option<u32>, language_id: Option<u32>) -> Result<Vec<Metaproduct>, ClientError> {
        info!("find metaproducts: search = {}, exact = {}, game_id = {:?}, language_id = {:?}",
            search,
            exact,
            game_id,
            language_id
        );

        let exact_str = exact.to_string();
        let game_id_str = match game_id {
            Some(id) => Some(id.to_string()),
            None => None
        };
        let language_id_str = match language_id {
            Some(id) => Some(id.to_string()),
            None => None
        };

        let mut query: Vec<(&str, &str)> = Vec::new();

        query.push(("search", search));
        query.push(("exact", &exact_str));

        match game_id_str {
            Some(ref id) => query.push(("idGame", id)),
            None => {}
        }

        match language_id_str {
            Some(ref id) => query.push(("idLanguage", id)),
            None => {}
        }

        let json_str = try!(self.connection.request("GET", "metaproducts/find", &query));
        let metaproducts = try!(Vec::<Metaproduct>::from_json(&json_str));

        info!("parsed {} metaproducts", metaproducts.len());

        Ok(metaproducts)
    }

    pub fn find_products(&mut self, search: &str, exact: bool, game_id: Option<u32>, language_id: Option<u32>, start: u32, max_results: u32) -> Result<Vec<Product>, ClientError> {
        info!("find products: search = {}, exact = {}, game_id = {:?}, language_id = {:?}, start = {}, max_results = {}",
            search,
            exact,
            game_id,
            language_id,
            start,
            max_results
        );

        let exact_str = exact.to_string();
        let game_id_str = match game_id {
            Some(id) => Some(id.to_string()),
            None => None
        };
        let language_id_str = match language_id {
            Some(id) => Some(id.to_string()),
            None => None
        };
        let start_str = start.to_string();
        let max_results_str = max_results.to_string();

        let mut query: Vec<(&str, &str)> = Vec::new();

        query.push(("search", search));
        query.push(("exact", &exact_str));
        query.push(("start", &start_str));
        query.push(("max_results", &max_results_str));

        match game_id_str {
            Some(ref id) => query.push(("idGame", id)),
            None => {}
        }

        match language_id_str {
            Some(ref id) => query.push(("idLanguage", id)),
            None => {}
        }

        let json_str = try!(self.connection.request("GET", "products/find", &query));
        info!("{}", json_str);
        let products = try!(Vec::<Product>::from_json(&json_str));

        info!("parsed {} products", products.len());

        Ok(products)
    }

    pub fn get_product(&mut self, product_id: u32) -> Result<Product, ClientError> {
        info!("get product: id = {}", product_id);

        let query: Vec<(&str, &str)> = Vec::new();

        let uri = format!("products/{}", product_id);

        let json_str = try!(self.connection.request("GET", &uri, &query));
        info!("{}", json_str);
        let products = try!(Product::from_json(&json_str));

        info!("parsed 1 product");

        Ok(products)
    }

}
