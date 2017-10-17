#[macro_use]
extern crate log;
extern crate optimizer;

use optimizer::logger;
use optimizer::client::connection::Connection;

pub fn main() {
    match logger::init() {
        Ok(_) => {},
        Err(e) => {
            println!("Could not init logger: {}", e);
            return;
        }
    }

    let mut conn = match Connection::new("app_token.json") {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    match conn.request("GET", "/ws/v2.0/account") {
        Ok(_) => {},
        Err(e) => {
            error!("{}", e);
            return
        }
    }

}
