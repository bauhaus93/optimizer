pub mod language;
pub mod condition;
pub mod game;
pub mod query_builder;

use std::fmt;

use url::percent_encoding::{ utf8_percent_encode, QUERY_ENCODE_SET };

pub struct Query {
    elements: Vec<(&'static str, String)>
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = {
            let mut output = String::new();

            self.elements.iter().for_each(| ref pair |
                output.push_str(&format!("{} = {}, ", pair.0, pair.1))
            );
            output.pop();
            output.pop();
            output
        };

        write!(f, "{}", output)
    }
}

impl Query {

    pub fn get_elements(&self) -> &[(&'static str, String)] {
        &self.elements
    }

    pub fn create_query_string(&self) -> String {
        let mut query_string = String::new();
        if !self.elements.is_empty() {
            query_string.push_str("?");
            self.elements.iter().for_each(| ref pair | {
                query_string.push_str(&format!("{}={}&", pair.0, utf8_percent_encode(&pair.1, QUERY_ENCODE_SET)))
            });
            query_string.pop();
        }
        query_string
    }
}
