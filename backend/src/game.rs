use std::str::FromStr;
use std::{collections::BTreeMap, sync::Arc};

use tokio::sync::Mutex;
use tokio::sync::RwLock;

mod card;

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
}

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
        let mut game = self.0.lock().await;
        
        if game.players.len() == 0 {
            game.players.insert(name.into(), Player::owner());
            Ok(())
        } else if game.players.contains_key(name) {
            anyhow::bail!("Name is already taken");
        } else {
            game.players.insert(name.into(), Player::player());
            Ok(())
        }
    }

    pub async fn buyin(&self, name: &str, amount: u64) {
        let mut game = self.0.lock().await;
        let player = game.players.get_mut(name).unwrap();

        if player.is_owner {
            player.total_buyin += amount as i64;
        } else {
            todo!()
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(GameState::default())))
    }
}

struct GameState {
    players: BTreeMap<String, Player>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            players: BTreeMap::new(),
        }
    }
}

struct Player {
    is_owner: bool,
    total_buyin: i64,
}

impl Player {
    fn owner() -> Self {
        Self {
            is_owner: true,
            total_buyin: 0,
        }
    }

    fn player() -> Self {
        Self {
            is_owner: false,
            total_buyin: 0,
        }
    }
}
