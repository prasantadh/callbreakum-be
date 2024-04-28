use axum::{extract, response::Json, routing::get, routing::post, Router};
use model::Game;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct IncomingMessage {
    game: String,
    player: String,
    data: String,
}

#[derive(Serialize)]
struct OutgoingMessage {
    status: String,
    data: String,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/new", get(newhandler))
        .route("/join", post(joinhandler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn newhandler() -> Json<OutgoingMessage> {
    let game = Game::new();

    // likely need to insert a key on redis here with the game id
    // turn this into a pubsub so the subsequent players can subscribe
    // so in a way, make requests through the API
    // but get updates through the websocket

    Json(OutgoingMessage {
        status: "Success".to_string(),
        data: game.id.to_string(),
    })
}

async fn joinhandler(
    extract::Json(payload): extract::Json<IncomingMessage>,
) -> Json<OutgoingMessage> {
    //TODO find a running game here instead of creating a new game
    // likely read game from redis
    // this request should come over a websocket
    // game is locked on redis, retrieved, deserialied, mutated, saved
    // additionally a new assistant bot for the player is started
    // how will we design impossible mode under this design
    // then the client gets the response
    //
    // eventually, if someone doesn't specify a game but is a valid player,
    // assign them to any game that is looking for a player

    let mut game = Game::new();
    match game.add_player() {
        Ok(v) => Json(OutgoingMessage {
            status: "success".to_string(),
            data: v.to_string(),
        }),
        Err(s) => Json(OutgoingMessage {
            status: "failure".to_string(),
            data: s.to_string(),
        }),
    }
}
