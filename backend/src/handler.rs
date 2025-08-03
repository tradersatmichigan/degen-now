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
    use axum::{Json, extract::State};

    use crate::game::Manager;

    #[derive(serde::Serialize)]
    pub struct Response {
        game_id: String,
    }

    pub async fn handle(manager: State<Manager>) -> Json<Response> {
        let id = manager.create().await;
        Json(Response {
            game_id: id.to_string(),
        })
    }
}

pub mod join {
    use anyhow::anyhow;
    use axum::{
        Json,
        extract::{Path, State},
    };
    use axum_extra::extract::{cookie::Cookie, SignedCookieJar};

    use crate::{
        game::Manager,
        handler::api::ApiResult,
    };

    #[derive(serde::Deserialize)]
    pub struct Request {
        name: String,
    }

    pub async fn handle(
        State(manager): State<Manager>,
        Path(game_id): Path<String>,
        jar: SignedCookieJar,
        Json(request): Json<Request>,
    ) -> ApiResult<SignedCookieJar> {
        let game = 
            manager.get(&game_id.parse()?).await.ok_or(anyhow!("Game doesn't exist"))?;
        game.join(&request.name).await?;
        Ok(jar.add(Cookie::new(game_id, request.name)))
    }
}
