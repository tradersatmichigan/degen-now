pub mod api {
    use axum::{http::StatusCode, response::IntoResponse};

    pub struct Error(anyhow::Error);

    impl<E: Into<anyhow::Error>> From<E> for Error {
        fn from(value: E) -> Self {
            Error(value.into())
        }
    }

    impl IntoResponse for Error {
        fn into_response(self) -> axum::response::Response {
            (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
        }
    }
}

pub mod create_game {
    use axum::{extract::State, Json};

    use crate::Manager;

    #[derive(serde::Serialize)]
    pub struct Response {
        game_id: u64,
    }

    pub async fn handle(State(manager) : State<Manager>) -> Json<Response> {
        let game_id = manager.write().await.create_game();
        Json(Response{game_id})
    }
}

// TODO
pub mod join_game {
    pub struct Request {
        username: String,
    }
}
