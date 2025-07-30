use std::{collections::BTreeMap, sync::Arc};

use tokio::sync::Mutex;

use crate::IdGenerator;

/// container for all active games
pub struct Manager {
    id_generator: IdGenerator,
    games: BTreeMap<u64, Arc<Mutex<Game>>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            id_generator: IdGenerator::new(),
            games: BTreeMap::new(),
        }
    }

    pub fn create_game(&mut self) -> u64 {
        let game_id = self.id_generator.get();
        self.games
            .insert(game_id, Arc::new(Mutex::new(Game::default())));
        game_id
    }

    pub fn get(&self, game_id: u64) -> Option<Arc<Mutex<Game>>> {
        self.games.get(&game_id).cloned()
    }
}

pub struct Game {}

impl Default for Game {
    fn default() -> Self {
        Self {}
    }
}
