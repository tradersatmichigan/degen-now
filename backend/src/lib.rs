use std::sync::Arc;

use axum::{routing::post, Router};
use tokio::sync::RwLock;

mod game;
mod handler;

pub fn app() -> Router {
    let manager = new_game_manager();

    Router::new()
        .route("/create-game", post(handler::create_game::handle))
        .with_state(manager)
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

type Manager = Arc<RwLock<game::Manager>>;

fn new_game_manager() -> Manager {
    Arc::new(RwLock::new(game::Manager::new()))
}
