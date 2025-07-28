use poem::{listener::TcpListener, Route, Server};

mod handler;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
