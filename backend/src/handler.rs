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

    pub type ApiResult<T> = Result<T, Error>;
}

pub mod create {
    use axum::{extract::State, Json};

    use crate::game::Manager;

    #[derive(serde::Serialize)]
    pub struct Response {
        game_id: String,
    }

    pub async fn handle(manager : State<Manager>) -> Json<Response> {
        let id = manager.create().await;
        Json(Response{
            game_id: id.to_string(),
        })
    }
}
