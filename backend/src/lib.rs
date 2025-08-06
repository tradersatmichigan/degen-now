use std::time::Duration;

use axum::{Router, extract::FromRef, routing::post};
use axum_extra::extract::cookie::Key;

mod game;
mod handler;

use game::Manager;

pub fn app() -> Router {
    let state = AppState::new();

    let manager = state.manager.clone();
    tokio::spawn(async move {
        loop {
            manager.tick().await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    Router::new()
        .route("/create", post(handler::create::handle))
        .route("/{game_id}/join", post(handler::join::handle))
        .route("/{game_id}/buyin", post(handler::buyin::handle))
        .route("/{game_id}/act", post(handler::act::handle))
        .with_state(state)
}

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
        Self {
            cookie_key: Key::generate(),
            manager: game::Manager::new(),
        }
    }
}
