pub mod api {
    use axum::{http::StatusCode, response::IntoResponse};

    pub enum Response {
        Ok(String),
        Err(anyhow::Error),
    }

    impl IntoResponse for Response {
        fn into_response(self) -> axum::response::Response {
            match self {
                Response::Ok(response) => (StatusCode::OK, response).into_response(),
                Response::Err(err) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                }
            }
        }
    }
}
