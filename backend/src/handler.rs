use axum::{http::StatusCode, response::IntoResponse};

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

type ApiResult<T> = Result<axum::Json<T>, AppError>;

pub mod create {
    #[derive(serde::Serialize)]
    pub struct Response {
        game_id: String
    }

    pub async fn handle() -> super::ApiResult<Response> {
        todo!()
    }
}
