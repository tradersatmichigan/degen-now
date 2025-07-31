use std::sync::Arc;

use axum::{extract::{FromRef, FromRequestParts, Path}, http::{request::Parts, StatusCode}, routing::post, RequestPartsExt, Router};
use tokio::sync::{RwLock, Mutex};

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

// request extractor so that any api with game_id in 
// path can receive the game struct without checking
struct ExtractGame(Arc<Mutex<game::Game>>);

impl<S: Send + Sync> FromRequestParts<S> for ExtractGame 
where Manager: FromRef<S>
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) 
        -> Result<Self,Self::Rejection> {
        let manager = Manager::from_ref(state);
        let Path(game_id) = parts.extract::<Path<u64>>().await
            .map_err(|_| (StatusCode::BAD_REQUEST, "missing game id"))?;

        match manager.read().await.get(game_id) {
            Some(game) => Ok(ExtractGame(game)),
            None => Err((StatusCode::BAD_REQUEST, "no game exists with this id")),
        }
    }
}
