use std::fs::File;
use std::io::Read;
use std::str;

use curl::easy;

use client_error::ClientError;

use request::Request;
use token::Token;
use query::Query;

use entities::entity::Entity;
use entities::metaproduct::Metaproduct;
use entities::product::Product;
use entities::article::Article;

pub struct MKMClient {
    handle: easy::Easy,
    token: Token
}

impl MKMClient {

    pub fn new(token_path: &str) -> Result<MKMClient, ClientError> {
        info!("creating new client");

        info!("creating curl handle");
        let handle = easy::Easy::new();

        info!("retrieving app token from file \"{}\"", token_path);
        let mut file = File::open(token_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let token = Token::from_json(&content)?;

        let client = MKMClient {
            handle: handle,
            token
        };

        Ok(client)
    }

    fn request(&mut self, rq: Request) -> Result<String, ClientError> {

        let mut list = easy::List::new();
        list.append(rq.get_oauth_header())?;
        self.handle.url(rq.get_uri())?;
        self.handle.http_headers(list)?;

        let mut buffer = Vec::new();
        {
            let mut transfer = self.handle.transfer();
            transfer.write_function(| data | {
                buffer.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }

        let response_code = self.handle.response_code()?;

        match response_code {
            200 | 206 => {  //206 = partial content, is returned when limiting search results
                info!("response code {}, read {} bytes", response_code, buffer.len());
                Ok(str::from_utf8(&buffer)?.to_string())
            },
            207 => {    //207 = no content
                info!("response code {}, no content", response_code);
                Ok(String::new())
            },
            _ => Err(ClientError::BadResponse(response_code))
        }
    }

    pub fn find_metaproducts(&mut self, query: Query) -> Result<Vec<Metaproduct>, ClientError> {
        let rq = Request::new("GET", "metaproducts/find", query, &self.token)?;

        let json_str = self.request(rq)?;
        let metaproducts = Vec::<Metaproduct>::from_json(&json_str)?;
        info!("parsed {} metaproducts", metaproducts.len());

        Ok(metaproducts)
    }

    pub fn find_products(&mut self, query: Query) -> Result<Vec<Product>, ClientError> {
        let rq = Request::new("GET", "products/find", query, &self.token)?;

        let json_str = self.request(rq)?;
        let products = Vec::<Product>::from_json(&json_str)?;
        info!("parsed {} products", products.len());

        Ok(products)
    }

    pub fn get_product(&mut self, product_id: u32, query: Query) -> Result<Product, ClientError> {
        let realm = format!("products/{}", product_id);
        let rq = Request::new("GET", &realm, query, &self.token)?;

        let json_str = self.request(rq)?;
        let product = Product::from_json(&json_str)?;
        info!("parsed 1 product");

        Ok(product)
    }

    pub fn get_articles(&mut self, article_id: u32, query: Query) -> Result<Vec<Article>, ClientError> {
        let realm = format!("articles/{}", article_id);
        let rq = Request::new("GET", &realm, query, &self.token)?;

        let json_str = self.request(rq)?;
        let articles = Vec::<Article>::from_json(&json_str)?;
        info!("parsed {} articles", articles.len());

        Ok(articles)
    }

}
