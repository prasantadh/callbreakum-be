use axum::{
    extract::{self, State},
    response::Json,
    routing::post,
    Router,
};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use model::Game;
use redis::AsyncCommands;
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
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    {
        // ping the database before starting
        let mut conn = pool.get().await.unwrap();
        conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
        let result: String = conn.get("foo").await.unwrap();
        assert_eq!(result, "bar");
    }

    // build our application with a route
    let app = Router::new()
        .route("/new", post(newhandler))
        .route("/join", post(joinhandler))
        .with_state(pool);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

type ConnectionPool = Pool<RedisConnectionManager>;
async fn newhandler(
    State(pool): State<ConnectionPool>,
    extract::Json(payload): extract::Json<IncomingMessage>,
) -> Json<OutgoingMessage> {
    // check that the player is valid
    // TODO need to check this is a correct user token
    // for now simply enforce 6 char username that is not playing any other game
    if payload.player.len() < 6 {
        return Json(OutgoingMessage {
            status: "Failure".to_string(),
            data: "player name must be six characters".to_string(),
        });
    }

    // check that the requesting player has no active game
    // TODO do I need to lock anything here?
    let mut conn = pool.get().await.unwrap();
    // will likely need to lock on the player key here
    let gameid: Option<String> = conn.get(payload.player.to_string()).await.unwrap();
    if gameid != None {
        return Json(OutgoingMessage {
            status: "Failure".to_string(),
            data: "player is in another game".to_string(),
        });
    }

    let game = Game::new();
    let _: () = conn
        .set(payload.player.to_string(), game.id.to_string())
        .await
        .unwrap();

    let _: () = conn
        .set(game.id.to_string(), serde_json::to_string(&game).unwrap())
        .await
        .unwrap();
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
