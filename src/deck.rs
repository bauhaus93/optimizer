use rand;
use rand::Rng;

use genetic::problem::Problem;
use genetic::solution::Solution;
use article::{ Article, calculate_article_fitness };

pub struct CardWanted {
    id_metaproduct: u32,
    count_wanted: u32,
    possible_articles: Vec<Article>
}

pub struct DeckWanted {
    cards: Vec<CardWanted>
}

impl CardWanted {
    pub fn get_random_article(&self) -> &Article {
        assert!(self.possible_articles.len() != 0);
        let index = rand::thread_rng().gen::<usize>() % self.possible_articles.len();
        &self.possible_articles[index]
    }
}

impl DeckWanted {
    pub fn get_wanted_cards(&self) -> &[CardWanted] {
        &self.cards
    }
}

impl Problem<Vec<Article>> for DeckWanted {

    fn create_random_solution(&self) -> Solution<Vec<Article>> {
        let mut card_selection: Vec<Article> = Vec::new();
        let mut fitness: u32 = 0;

        for card in self.get_wanted_cards() {
            let article = card.get_random_article();
            fitness += article.get_total_price();
            card_selection.push(article.clone());
        }
        Solution::new(card_selection, fitness)
    }

    fn create_child_solution(&self, parents: (&Solution<Vec<Article>>, &Solution<Vec<Article>>)) -> Solution<Vec<Article>> {
        let selection_a = parents.0.get_encoding();
        let selection_b = parents.1.get_encoding();
        assert!(selection_a.len() == selection_b.len());

        let split_point: usize = 1 + random(selection_a.len() as u32 - 2) as usize;

        let mut child_selection = Vec::new();
        child_selection.extend_from_slice(&selection_a[..split_point]);
        child_selection.extend_from_slice(&selection_b[split_point..]);

        let fitness = calculate_article_fitness(&child_selection);

        Solution::new(child_selection, fitness)
    }

    fn mutate_solution(&self, solution: Solution<Vec<Article>>) -> Solution<Vec<Article>> {
        let mut selection = solution.consume();

        for (article, possible_articles) in selection.iter_mut().zip(self.get_wanted_cards().iter()) {
            if random(100) < 20 {
                *article = possible_articles.get_random_article().clone();
            }
        }
        let fitness = calculate_article_fitness(&selection);

        Solution::new(selection, fitness)
    }
}

fn random(max: u32) -> u32 {
    rand::thread_rng().gen::<u32>() % max
}
