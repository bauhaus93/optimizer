use resource_error::ResourceError;
use db_client::DBClient;
use mkm_client::mkm_client::MKMClient;
use mkm_client::query::query_builder::QueryBuilder;
use mkm_client::entities::metaproduct::Metaproduct;
use mkm_client::entities::article::Article;
use mkm_client::entities::user::User;


pub struct ResourceHandler {
    db_client: DBClient,
    mkm_client: MKMClient
}

impl ResourceHandler {

    pub fn new(token_path: &str, db_path: &str) -> Result<ResourceHandler, ResourceError> {
        info!("creating new resource handler");

        let db_client = DBClient::new(db_path)?;
        let mkm_client = MKMClient::new(token_path)?;


        let rh = ResourceHandler {
            db_client: db_client,
            mkm_client: mkm_client
        };

        Ok(rh)
    }

    pub fn download_card_data(&mut self, card_name: &str) -> Result<i32, ResourceError>   {
        info!("downloading and saving card data for {}", card_name);

        let metaproduct = self.download_metaproduct(card_name)?;
        let products = metaproduct.get_products();
        let mut articles: Vec<Article> = Vec::new();
        let mut seller: Vec<User> = Vec::new();

        for product in products {
            let mut arts = match self.download_articles(product.get_id()) {
                Ok(a) => a,
                Err(e) => {
                    error!("{}", e);
                    continue;
                }
            };
            for a in &arts {
                if !seller.contains(a.get_seller()) {
                    seller.push(a.get_seller().clone());
                }
            }
            articles.append(&mut arts);
        }

        let mut sum = 0;
        sum += self.db_client.save_metaproduct(&metaproduct)?;
        sum += self.db_client.save_products(&products)?;
        sum += self.db_client.save_users(&seller)?;
        sum += self.db_client.save_articles(&articles)?;

        info!("changed {} rows in db", sum);

        Ok(sum)
    }

    fn download_metaproduct(&mut self, card_name: &str) -> Result<Metaproduct, ResourceError> {
        debug!("downloading metaproduct/product data...");
        let query = QueryBuilder::new()
            .name(card_name)
            .exact(true)
            .finalize();

        let mut metaproducts = self.mkm_client.find_metaproducts(query)?;
        if metaproducts.len() > 1 {
            warn!("Found {} metaproducts, only the first metaproduct will be used", metaproducts.len());
        }
        Ok(metaproducts.remove(0))
    }

    fn download_articles(&mut self, product_id: u32) -> Result<Vec<Article>, ResourceError> {
        debug!("downloading articles...");
        let query = QueryBuilder::new()
            .start(0)
            .max_results(10)
            .finalize();

        let articles = self.mkm_client.get_articles(product_id, query)?;

        Ok(articles)
    }


}
