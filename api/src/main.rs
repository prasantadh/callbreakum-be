use axum::{response::Html, routing::get, Router};
use model::Game;

#[tokio::main]
async fn main() {
    let game: Game = Game::new();

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("{:?}", game);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1> helloworld!<h1>")
}
