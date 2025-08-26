use std::str::FromStr;
use std::{collections::BTreeMap, sync::Arc};

use tokio::sync::Mutex;
use tokio::sync::RwLock;

mod card;
mod rules;

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

    pub async fn get(&self, game_id: &GameId) -> Option<Game> {
        self.0.read().await.get(game_id).cloned()
    }

    /// update time in all games to force actions like
    /// auto fold, card dealing and other timed events
    pub async fn tick(&self) {
        let mp = self.0.read().await;

        for (_, game) in mp.iter() {
            game.tick().await;
        }
    }
}

/// used as key to identify game
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GameId(uuid::Uuid);

impl GameId {
    fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl FromStr for GameId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        uuid::Uuid::parse_str(s).map(GameId)
    }
}

impl ToString for GameId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone)]
pub struct Game(Arc<Mutex<GameState>>);

impl Game {
    pub async fn join(&self, name: &str) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn buyin(&self, name: &str, amount: u64) {
        todo!()
    }

    pub async fn act(&self, name: &str, action: Action) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn tick(&self) {}
}

impl Default for Game {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(GameState::default())))
    }
}

#[derive(serde::Deserialize)]
pub enum Action {
    Bet(u64),
    Check,
    Fold,
}

struct GameState {}

impl GameState {

}

impl Default for GameState {
    fn default() -> Self {
        Self {}
    }
}
