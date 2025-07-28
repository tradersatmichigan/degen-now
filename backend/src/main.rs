use axum::{routing::post, Router};

mod handler;
mod game_manager;

#[tokio::main]
async fn main() {

    // Add api routes here
    let app = Router::new()
        .route("/create", post(handler::create::handle));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
