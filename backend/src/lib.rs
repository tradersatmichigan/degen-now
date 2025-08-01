use axum::{extract::FromRef, routing::post, Router};
use axum_extra::extract::cookie::Key;

mod game;
mod handler;

use game::Manager;

pub fn app() -> Router {
    let state = AppState::new();

    Router::new()
        .route("/create", post(handler::create::handle))
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
            manager: game::Manager::new()
        }
    }
}
