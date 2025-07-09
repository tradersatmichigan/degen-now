use axum::{
    response::IntoResponse, Router, http::StatusCode
};

mod game_manager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn app() -> Router {
    Router::new()
}

/// Wrapper error for anyhow that allows it to be returned as an error to client
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            self.0.to_string()
        )
            .into_response()
    }
}

impl<T> From<T> for AppError
where T: Into<anyhow::Error> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
