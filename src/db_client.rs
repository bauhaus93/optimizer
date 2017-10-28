use std::path::Path;

use rusqlite::{ Connection, Transaction, TransactionBehavior };

use resource_error::ResourceError;
use mkm_client::entities::metaproduct::Metaproduct;
use mkm_client::entities::product::Product;
use mkm_client::entities::article::Article;
use mkm_client::entities::user::User;

pub struct DBClient {
    connection: Connection
}

impl DBClient {

    pub fn new(db_path: &str) -> Result<DBClient, ResourceError> {
        debug!("creating new db client, db_path = {}", db_path);

        let path = Path::new(db_path);

        let mut conn = Connection::open(path)?;

        create_db_tables(&mut conn)?;

        let db_client = DBClient {
            connection: conn
        };

        Ok(db_client)
    }

    pub fn save_metaproduct(&mut self, metaproduct: &Metaproduct) -> Result<i32, ResourceError> {
        debug!("saving 1 metaproduct");
        let mut stmt = self.connection.prepare("INSERT OR REPLACE INTO Metaproduct VALUES (?, ?)")?;
        let id = metaproduct.get_metaproduct().get_id();
        let name = metaproduct.get_metaproduct().get_name_loc();
        let count = stmt.execute(&[&id, &name])?;
        Ok(count)
    }

    pub fn save_products(&mut self, products: &[Product]) -> Result<i32, ResourceError> {
        debug!("saving {} products", products.len());
        let tx = Transaction::new(&mut self.connection, TransactionBehavior::Deferred)?;
        let mut sum = 0;
        {
        let mut stmt = tx.prepare("INSERT OR REPLACE INTO Product VALUES (?, ?, ?, ?, ?)")?;

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
        }
        tx.commit()?;
        Ok(sum)
    }

    pub fn save_users(&mut self, users: &[User]) -> Result<i32, ResourceError> {
        debug!("saving {} users", users.len());
        let tx = Transaction::new(&mut self.connection, TransactionBehavior::Deferred)?;
        let mut sum = 0;
        {
        let mut stmt = tx.prepare("INSERT OR REPLACE INTO User VALUES (?, ?, ?, ?, ?, ?, ?, ?)")?;


        for user in users {
            sum += stmt.execute(&[
                &user.get_id(),
                &user.get_username(),
                &user.get_commercial_status(),
                &user.get_address().get_country(),
                &user.get_risk_group(),
                &user.get_reputation(),
                &user.get_ships_fast(),
                &user.get_on_vacation()
            ])?;
        }
        }
        tx.commit()?;
        Ok(sum)
    }

    pub fn save_articles(&mut self, articles: &[Article]) -> Result<i32, ResourceError> {
        debug!("saving {} articles", articles.len());
        let tx = Transaction::new(&mut self.connection, TransactionBehavior::Deferred)?;
        let mut sum = 0;
        {
        let mut stmt = tx.prepare("INSERT OR REPLACE INTO Article VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;

        for article in articles {
            let price: u32 = (article.get_price() * 100.0) as u32;
            let condition = match article.get_condition() {
                Some(cond) => cond,
                None => {
                    warn!("Article without condition found, discarding");
                    continue;
                }
            };
            let foil = match article.is_foil() {
                Some(foil) => foil as u8,
                None => {
                    warn!("Article without foil status found, discarding");
                    continue;
                }
            };
            let signed = match article.is_signed() {
                Some(signed) => signed as u8,
                None => {
                    warn!("Article without signed status found, discarding");
                    continue;
                }
            };
            let altered = match article.is_altered() {
                Some(altered) => altered as u8,
                None => {
                    warn!("Article without altered status found, discarding");
                    continue;
                }
            };

            sum += stmt.execute(&[
                &article.get_id(),
                &article.get_seller().get_id(),
                &article.get_product_id(),
                &article.get_comments(),
                &price,
                &article.get_count(),
                &condition,
                &foil,
                &signed,
                &altered
            ])?;
        }
        }
        tx.commit()?;
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
        username        TEXT NOT NULL,
        commercial      INTEGER NOT NULL,
        country         TEXT NOT NULL,
        riskGroup       INTEGER NOT NULL,
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
        comments        TEXT NOT NULL,
        price           INTEGER NOT NULL,
        count           INTEGER NOT NULL,
        condition       TEXT NOT NULL,
        foil            INTEGER NOT NULL,
        signed          INTEGER NOT NULL,
        altered         INTEGER NOT NULL)
    ", &[])?;

    Ok(())
}
