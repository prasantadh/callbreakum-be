use axum::{
    extract::{self, State},
    response::Json,
    routing::post,
    Router,
};
use model::Game;
use redis::{Commands, JsonCommands};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    let client = redis::Client::open("redis://cache:6379").unwrap();

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
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn newhandler(
    State(client): State<redis::Client>,
    extract::Json(payload): extract::Json<IncomingMessage>,
) -> Json<OutgoingMessage> {
    // check that the player is valid
    // TODO need to check this is a correct user session token
    // for now simply enforce 6 char username that is not playing any other game
    if payload.player.len() < 6 {
        return Json(OutgoingMessage {
            status: "Failure".to_string(),
            data: "player name must be six characters".to_string(),
        });
    }

    let mut conn = client.get_connection().unwrap();
    // TODO change all the unwraps into internal server error
    redis::cmd("watch").arg(&payload.player).execute(&mut conn);
    let v: Option<String> = conn.get(&payload.player).unwrap();
    match v {
        Some(_) => Json(OutgoingMessage {
            status: "failure".to_string(),
            // TODO once we can verify valid user session tokens,
            // return success with current game id instead of failure
            data: "there was an error processing your request".to_string(),
        }),
        None => {
            let mut game = Game::new();
            game.add_player(&payload.player).unwrap();
            redis::pipe()
                .cmd("multi")
                .json_set(game.id.to_string(), "$", &json!(game))
                .unwrap()
                .set(&payload.player, game.id.to_string())
                .cmd("exec")
                .execute(&mut conn);
            Json(OutgoingMessage {
                status: "success".to_string(),
                data: game.id.to_string(),
            })
        }
    }
}

async fn callhandler(
    State(client): State<redis::Client>,
    extract::Json(payload): extract::Json<IncomingMessage>,
) -> Json<OutgoingMessage> {
    // now do this inside a transaction
    // return 0 if no error, or another value if error

    let mut conn = client.get_connection().unwrap();
    let v: u32 = redis::transaction(
        &mut conn,
        &[payload.player.to_string(), payload.game.to_string()],
        |con, pipe| {
            let game: String = con.json_get(&payload.game, "$")?;
            let mut game: Vec<Game> = serde_json::from_str(game.as_str()).unwrap();
            let game = &mut game[0];
            match game.call(&payload.player, payload.data.parse::<u8>().unwrap()) {
                Ok(_) => {
                    pipe.json_set(game.id.to_string(), "$", &game)
                        .unwrap()
                        .query::<()>(con)
                        .unwrap();
                    // TODO possible crafting JSON OutgoingMessage here can help
                    // provide better feedback to the user of the API
                    Ok(Some(0))
                }
                Err(_) => Ok(Some(1)),
            }
        },
    )
    .unwrap();

    match v {
        0 => Json(OutgoingMessage {
            status: "success".to_string(),
            data: payload.data,
        }),
        _ => Json(OutgoingMessage {
            status: "failure".to_string(),
            data: "there was an error processing your request".to_string(),
        }),
    }
}

async fn joinhandler(
    State(client): State<redis::Client>,
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

    // TODO verify a player session token first
    let mut conn = client.get_connection().unwrap();
    redis::cmd("watch")
        .arg(&payload.player)
        .arg(&payload.game)
        .execute(&mut conn);

    let game: Option<String> = conn.json_get(&payload.game, "$").unwrap();
    if game.is_none() {
        println!("non existent game!");
        return Json(OutgoingMessage {
            status: "failure".to_string(),
            data: "invalid game token".to_string(),
        });
    }
    let game = game.unwrap();
    let mut game: Vec<Game> = serde_json::from_str(game.as_str()).unwrap();
    let mut game = game.remove(0);
    match game.add_player(&payload.player) {
        Err(_) => Json(OutgoingMessage {
            status: "failure".to_string(),
            data: "there was an error adding the player".to_string(),
        }),
        Ok(_) => {
            redis::pipe()
                .cmd("multi")
                .json_set(game.id.to_string(), "$", &json!(game))
                .unwrap()
                .set(&payload.player, game.id.to_string())
                .cmd("exec")
                .execute(&mut conn);
            Json(OutgoingMessage {
                status: "success".to_string(),
                data: "player has been added to the game".to_string(),
            })
        }
    }
}
