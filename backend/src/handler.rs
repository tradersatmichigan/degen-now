pub mod api {
    use axum::{response::IntoResponse, Json};
    use serde_json::Value;

    pub struct Ok(Json<Value>);

    impl<R: serde::Serialize> From<R> for Ok {
        fn from(value: R) -> Self {
            Ok(Json(serde_json::to_value(&value).unwrap()))
        }
    }

    impl IntoResponse for Ok {
        fn into_response(self) -> axum::response::Response {
            self.0.into_response()
        }
    }

    pub struct Error(anyhow::Error);

    impl<E: Into<anyhow::Error>> From<E> for Error {
        fn from(value: E) -> Self {
            Error(value.into())
        }
    }

    impl IntoResponse for Error {
        fn into_response(self) -> axum::response::Response {
            todo!()
        }
    }

    pub type Response = Result<Ok, Error>;
}

pub mod create_game {
    use axum::extract::State;

    use crate::Manager;

    use super::api;

    #[derive(serde::Serialize)]
    struct Response {
        game_id: u64,
    }

    pub async fn handle(State(manager) : State<Manager>) -> api::Response {
        let game_id = manager.write().await.create_game();
        Ok(Response{game_id}.into())
    }
}
