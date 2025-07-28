use axum::Router;

mod handler;

#[tokio::main]
async fn main() {

    // Add api routes here
    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
