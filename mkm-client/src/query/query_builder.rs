
use query::Query;
use query::game::{ Game, get_game_id };
use query::language::{ Language, get_language_id };
use query::condition::{ Condition, get_condition_string };


pub struct QueryBuilder {
    name: Option<String>,
    exact: Option<bool>,
    game: Option<Game>,
    language: Option<Language>,
    min_condition: Option<Condition>,
    foil: Option<bool>,
    signed: Option<bool>,
    altered: Option<bool>,
    min_available: Option<u32>,
    start: Option<u32>,
    max_results: Option<u32>
}

impl QueryBuilder {

    pub fn new() -> QueryBuilder {
        QueryBuilder {
            name: None,
            exact: None,
            game: None,
            language: None,
            min_condition: None,
            foil: None,
            signed: None,
            altered: None,
            min_available: None,
            start: None,
            max_results: None
        }
    }

    pub fn finalize(self) -> Query {
        let mut query_elements = Vec::new();

        match self.name {
            Some(name) => query_elements.push(("search", name.clone())),
            None => {}
        }

        match self.exact {
            Some(exact) => query_elements.push(("exact", exact.to_string())),
            None => {}
        }

        match self.game {
            Some(game) => query_elements.push(("idGame", get_game_id(game).to_string())),
            None => {}
        }

        match self.language {
            Some(language) => query_elements.push(("idLanguage", get_language_id(language).to_string())),
            None => {}
        }

        match self.min_condition {
            Some(condition) => query_elements.push(("minCondition", get_condition_string(condition).to_owned())),
            None => {}
        }

        match self.foil {
            Some(foil) => query_elements.push(("isFoil", foil.to_string())),
            None => {}
        }

        match self.signed {
            Some(signed) => query_elements.push(("isSigned", signed.to_string())),
            None => {}
        }

        match self.altered {
            Some(altered) => query_elements.push(("isAltered", altered.to_string())),
            None => {}
        }

        match self.min_available {
            Some(min_available) => query_elements.push(("minAvailable", min_available.to_string())),
            None => {}
        }

        match self.start {
            Some(start) => query_elements.push(("start", start.to_string())),
            None => {}
        }

        match self.max_results {
            Some(max_results) => query_elements.push(("maxResults", max_results.to_string())),
            None => {}
        }

        Query {
            elements: query_elements
        }
    }

    pub fn exact(mut self) -> Self {
        self.exact = Some(true);
        self
    }

    pub fn not_exact(mut self) -> Self {
        self.exact = Some(false);
        self
    }

    pub fn game(mut self, game: Game) -> Self {
        self.game = Some(game);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn min_condition(mut self, min_condition: Condition) -> Self {
        self.min_condition = Some(min_condition);
        self
    }

    pub fn foil(mut self) -> Self {
        self.foil = Some(true);
        self
    }

    pub fn not_foil(mut self) -> Self {
        self.foil = Some(false);
        self
    }

    pub fn signed(mut self) -> Self {
        self.signed = Some(true);
        self
    }

    pub fn not_signed(mut self) -> Self {
        self.signed = Some(false);
        self
    }

    pub fn altered(mut self) -> Self {
        self.altered = Some(true);
        self
    }

    pub fn not_altered(mut self) -> Self {
        self.altered = Some(false);
        self
    }

    pub fn min_available(mut self, count: u32) -> Self {
        self.min_available = Some(count);
        self
    }

    pub fn start(mut self, start: u32) -> Self {
        self.start = Some(start);
        self
    }

    pub fn max_results(mut self, max_results: u32) -> Self {
        self.max_results = Some(max_results);
        self
    }
}
