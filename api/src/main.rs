use axum::{
    extract::{self, State},
    response::Json,
    routing::post,
    Router,
};
use model::Game;
use redis::{Commands, JsonCommands};
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
    let client = redis::Client::open("redis://localhost").unwrap();

    {
        // TODO create an index on the database to search player name
        // ping the database before starting
        let mut conn = client.get_connection().unwrap();
        let _: () = conn.set("foo", "bar").unwrap();
        let result: String = conn.get("foo").unwrap();
        assert_eq!(result, "bar");
    }

    // build our application with a route
    let app = Router::new()
        .route("/new", post(newhandler))
        .route("/join", post(joinhandler))
        .route("/call", post(callhandler))
        .with_state(client);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn newhandler(
    State(client): State<redis::Client>,
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

    // within a transaction, see if the player has an active game
    // if not then create a game, add current player then move on
    let mut conn = client.get_connection().unwrap();
    let mut game = Game::new();
    let v: u32 = redis::transaction(&mut conn, &[payload.player.to_string()], |con, pipe| {
        let count: Option<String> = con.get(&payload.player).unwrap();
        return match count {
            None => {
                game.add_player(payload.player.to_string()).unwrap();
                pipe.set(payload.player.to_string(), game.id.to_string())
                    .json_set(game.id.to_string(), "$", &game)
                    .unwrap()
                    .query::<()>(con)
                    .unwrap();
                Ok(Some(0))
            }
            _ => Ok(Some(1)),
        };
    })
    .unwrap();

    match v {
        0 => Json(OutgoingMessage {
            status: "Success".to_string(),
            data: game.id.to_string(),
        }),
        _ => Json(OutgoingMessage {
            status: "Failure".to_string(),
            data: "there was an error processing your request".to_string(),
        }),
    }
}

async fn callhandler(
    State(client): State<redis::Client>,
    extract::Json(payload): extract::Json<IncomingMessage>,
) -> Json<OutgoingMessage> {
    // now do this inside a transaction
    // return 0 if no error, or another value if error
    let mut conn = client.get_connection().unwrap();
    let game: String = conn.json_get(&payload.game, "$").unwrap();
    let game: Vec<Game> = serde_json::from_str(game.as_str()).unwrap();
    println!("game: {:?}", game[0]);

    Json(OutgoingMessage {
        status: "success".to_string(),
        data: payload.data,
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

    // match game.add_player() {
    //     Ok(v) => Json(OutgoingMessage {
    //         status: "success".to_string(),
    //         data: v.to_string(),
    //     }),
    //     Err(s) => Json(OutgoingMessage {
    //         status: "failure".to_string(),
    //         data: s.to_string(),
    //     }),
    // }
    Json(OutgoingMessage {
        status: "success".to_string(),
        data: "under construction".to_string(),
    })
}
