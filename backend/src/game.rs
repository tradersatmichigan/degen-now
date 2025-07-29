use std::{collections::BTreeMap, sync::{Arc, Mutex}, time::{Duration}};

mod card;

use card::CardSet;

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
        self.games.insert(game_id, Arc::new(Mutex::new(Game::new())));
        game_id
    }

    pub fn get(&self, game_id: u64) -> Option<Arc<Mutex<Game>>> {
        self.games.get(&game_id).cloned()
    }

    pub fn tick(&self, seconds: Duration) {
        for (_, game) in self.games.iter() {
            game.lock().unwrap().tick(seconds);
        }
    }
}

// container for a games state
pub struct Game {
    is_running: bool,
    players: Vec<Player>,
    deck: CardSet,
    board: CardSet,
}

impl Game {
    fn new() -> Self {
        todo!()
    }

    fn tick(&mut self, seconds: Duration) {
        todo!()
    }
}

struct Player {
    buyin: u64,
    current_stack: u64,
    cards: CardSet,
}

/// sequential number generator that wraps back arround to zero
struct IdGenerator(u64);

impl IdGenerator {
    fn new() -> Self {
        Self(0)
    }

    fn get(&mut self) -> u64 {
        let ans = self.0;
        self.0 = ans.wrapping_add(1);
        ans
    }
}
