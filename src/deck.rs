use rand;
use rand::Rng;

use genetic::sequence::{ Encodeable, Decodeable };
use genetic::fitness::Fitness;
use genetic::mutable::Mutable;

#[derive(Clone)]
pub struct Article {
    id_article: u32,
    count: u32,
    price: u32
}

pub struct CardWanted {
    id_metaproduct: u32,
    count_wanted: u32,
    possible_articles: Vec<Article>
}

pub struct DeckWanted {
    cards: Vec<CardWanted>
}

pub struct DeckSelection {
    selection: Vec<Article>,
    fitness: u32
}

impl Article {

    pub fn get_total_price(&self) -> u32 {
        self.count * self.price
    }
}

impl CardWanted {

    pub fn get_random_article(&self) -> &Article {
        let index = rand::thread_rng().gen::<usize>() % self.possible_articles.len();
        &self.possible_articles[index]
    }
}



impl DeckWanted {

    pub fn get_cards(&self) -> &[CardWanted] {
        &self.cards
    }

    pub fn create_random_solution(&self) ->

}

impl DeckSelection {

    pub fn new(wanted: &DeckWanted) -> DeckSelection {

        let mut card_selection: Vec<Article> = Vec::new();
        let mut fitness: u32 = 0;

        for card in wanted.get_cards() {
            let article = card.get_random_article();
            fitness += article.get_total_price();
            card_selection.push(article.clone());
        }

        DeckSelection {
            selection: card_selection,
            fitness: fitness
        }
    }

}

impl Fitness for DeckSelection {
    fn get_fitness(&self) -> i32 {
        self.fitness as i32
    }
}

impl Mutable for Sequence<DeckSelection, Article> {
    fn mutate(&mut self) {
        for e in &mut self.get_encoding_mut().iter_mut() {
            if random(100) < 20 {
                *e = (*e + 1) % 10;
            }
        }
    }
}
