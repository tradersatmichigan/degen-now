use std::sync::Arc;

use axum::{extract::FromRef, Router};
use axum_extra::extract::cookie::Key;
use tokio::sync::RwLock;

mod game;
mod handler;

pub fn app() -> Router {
    let state = AppState::new();

    Router::new()
        .with_state(state)
}

type Manager = Arc<RwLock<game::Manager>>;

#[derive(Clone)]
struct AppState {
    cookie_key: Key,
    manager: Manager,
}

impl FromRef<AppState> for Key {
    fn from_ref(input: &AppState) -> Self {
        input.cookie_key.clone()
    }
}

impl FromRef<AppState> for Manager {
    fn from_ref(input: &AppState) -> Self {
        input.manager.clone()
    }
}

impl AppState {
    fn new() -> Self {
        todo!()
    }
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
