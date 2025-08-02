use std::{collections::BTreeMap, sync::Arc};

use tokio::sync::RwLock;
use tokio::sync::Mutex;

/// container for all active games
#[derive(Clone)]
pub struct Manager(Arc<RwLock<BTreeMap<GameId, Game>>>);

impl Manager {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(BTreeMap::new())))
    }

    pub async fn create(&self) -> GameId {
        let id = GameId::new();
        self.0.write().await.insert(id.clone(), Game::default());
        id
    }

    pub async fn get(&self, game_id: GameId) -> Option<Game> {
        self.0.read().await.get(&game_id).cloned()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GameId(uuid::Uuid);

impl GameId {
    fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl ToString for GameId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone)]
pub struct Game(Arc<Mutex<GameState>>);

impl Default for Game {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(GameState::default())))
    }
}

struct GameState {}

impl Default for GameState {
    fn default() -> Self {
        Self {}
    }
}
