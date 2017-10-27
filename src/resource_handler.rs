use std::path::Path;
use rusqlite::{ Connection };

use resource_error::ResourceError;
use mkm_client::mkm_client::MKMClient;
use mkm_client::query::query_builder::QueryBuilder;
use mkm_client::entities::metaproduct::Metaproduct;
use mkm_client::entities::product::Product;


pub struct ResourceHandler {
    db_conn: Connection,
    mkm_client: MKMClient
}

impl ResourceHandler {

    pub fn new(token_path: &str, db_path: &str) -> Result<ResourceHandler, ResourceError> {
        info!("creating new resource handler");

        let path = Path::new(db_path);

        let mut conn = Connection::open(path)?;
        let mkm_client = MKMClient::new(token_path)?;

        create_db_tables(&mut conn)?;

        let rh = ResourceHandler {
            db_conn: conn,
            mkm_client: mkm_client
        };

        Ok(rh)
    }

    pub fn download_card_data(&mut self, card_name: &str) {
        info!("downloading card data for {}", card_name);

        match self.download_product_info(card_name) {
            Ok(_) => {},
            Err(e) => {
                error!("{}", e);
                return;
            }
        }
    }

    fn download_product_info(&mut self, card_name: &str) -> Result<(), ResourceError> {
        let metaproduct = self.download_metaproduct(card_name)?;
        let products = metaproduct.get_products();

        self.store_metaproduct(&metaproduct);
        self.store_products(&products);

        Ok(())
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

    fn store_metaproduct(&mut self, metaproduct: &Metaproduct) -> Result<i32, ResourceError> {
        debug!("storing 1 metaproduct");
        let mut stmt = self.db_conn.prepare("INSERT INTO Metaproduct VALUES (?, ?)")?;
        let id = metaproduct.get_metaproduct().get_id();
        let name = metaproduct.get_metaproduct().get_name_loc();
        let count = stmt.execute(&[&id, &name])?;
        debug!("{} metaproducts were stored", count);
        Ok(count)
    }

    fn store_products(&mut self, products: &[Product]) -> Result<i32, ResourceError> {
        debug!("storing {} products", products.len());
        let mut stmt = self.db_conn.prepare("INSERT INTO Product VALUES (?, ?, ?, ?, ?)")?;
        let mut sum = 0;

        for product in products {
            let exp_name = match product.get_expansion_name() {
                Some(name) => name,
                None => {
                    warn!("Product without expansion name found, discarding");
                    continue;
                }
            };

            let rarity = match product.get_rarity() {
                Some(rarity) => rarity,
                None => {
                    warn!("Product without rarity found, discarding");
                    continue;
                }
            };

            sum += stmt.execute(&[
                &product.get_id(),
                &product.get_metaproduct_id(),
                &product.get_game_name(),
                &exp_name,
                &rarity
            ])?;
        }
        debug!("{} products were stored", sum);
        Ok(sum)
    }
}

fn create_db_tables(db_conn: &mut Connection) -> Result<(), ResourceError> {
    debug!("create metaproduct table");
    db_conn.execute("
        CREATE TABLE IF NOT EXISTS Metaproduct(
        id      INTEGER PRIMARY KEY,
        name    TEXT NOT NULL)
    ", &[])?;
    debug!("create product table");
    db_conn.execute("
        CREATE TABLE IF NOT EXISTS Product(
        id              INTEGER PRIMARY KEY,
        metaproductId   INTEGER NOT NULL REFERENCES metaproduct(id) ON DELETE CASCADE,
        gameName        TEXT NOT NULL,
        expansionName   TEXT NOT NULL,
        rarity          TEXT NOT NULL)
    ", &[])?;

    debug!("create user table");
    db_conn.execute("
        CREATE TABLE IF NOT EXISTS User(
        id              INTEGER PRIMARY KEY,
        name            TEXT NOT NULL,
        commercial      INTEGER NOT NULL,
        country         TEXT NOT NULL,
        lossPercentage  TEXT NOT NULL,
        reputation      INTEGER NOT NULL,
        shipsFast       INTEGER NOT NULL,
        onVacation      INTEGER NOT NULL)
    ", &[])?;

    debug!("create article table");
    db_conn.execute("
        CREATE TABLE IF NOT EXISTS Article(
        id              INTEGER PRIMARY KEY,
        sellerId        INTEGER NOT NULL REFERENCES user(id) ON DELETE CASCADE,
        productId       INTEGER NOT NULL REFERENCES product(id) ON DELETE CASCADE,
        comments        TEXT,
        price           INTEGER NOT NULL,
        count           INTEGER NOT NULL,
        condition       TEXT NOT NULL,
        foil            INTEGER NOT NULL,
        signed          INTEGER NOT NULL,
        altered         INTEGER NOT NULL)
    ", &[])?;

    Ok(())
}
