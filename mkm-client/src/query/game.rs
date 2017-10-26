
pub enum Game {
    MAGIC,
    YUGIOH
}

pub fn get_game_id(game: Game) -> u32 {
    match game {
        Game::MAGIC => 1,
        Game::YUGIOH => 3
    }
}
