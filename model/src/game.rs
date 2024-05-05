use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
enum Card {}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    id: String, // at some point this will have to be UUID
}

#[derive(Debug, Serialize, Deserialize)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Trick {
    cards: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Round {
    hands: Vec<Hand>,   // indexed by player
    tricks: Vec<Trick>, // indexed by trick number
    calls: Vec<u8>,     // indexed by player
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(serialize_with = "uuid::serde::simple::serialize")]
    pub id: Uuid,
    players: Vec<Player>,
    rounds: Vec<Round>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: Uuid::new_v4(),
            players: vec![],
            rounds: vec![],
        }
    }

    pub fn add_player(&mut self, id: String) -> Result<usize, &'static str> {
        // TODO: possibly implement shuffling players
        match self.players.len() {
            0..=3 => {
                self.players.push(Player { id });
                Ok(self.players.len())
            }
            _ => Err("table already full!"),
        }
    }

    pub fn call(&mut self, player: &String, call: u8) -> Result<(), &'static str> {
        let round = self.rounds.last();
        return match round {
            None => {
                println!("no active round in play");
                Err("no active round in play")
            }
            Some(v) => {
                let round_index = self.rounds.len() - 1;
                let turn = (round_index + v.calls.len()) % 4;
                if &self.players[turn].id == player {
                    self.rounds[round_index].calls.push(call);
                    Ok(())
                } else {
                    println!("it is not currently the player's turn");
                    Err("attepted out of turn play")
                }
            }
        };
    }
}
