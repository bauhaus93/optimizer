
pub fn calculate_article_fitness(deck: &[Article]) -> u32 {
    let mut sum = 0;
    deck.iter().for_each(| ref e | sum += e.get_total_price());
    sum
}

#[derive(Clone)]
pub struct Article {
    id_article: u32,
    count: u32,
    price: u32
}

impl Article {

    pub fn get_total_price(&self) -> u32 {
        self.count * self.price
    }
}

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        self.id_article == other.id_article
    }
}
