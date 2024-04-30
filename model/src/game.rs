use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
enum Card {}

#[derive(Debug, Serialize)]
struct Player {
    id: u8, // at some point this will have to be UUID
}

#[derive(Debug, Serialize)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, Serialize)]
struct Trick {
    cards: Vec<Card>,
}

#[derive(Debug, Serialize)]
struct Round {
    hands: Vec<Hand>,   // indexed by player
    tricks: Vec<Trick>, // indexed by trick number
    calls: Vec<u8>,     // indexed by player
    breaks: Vec<u8>,    // indexed by player
}

#[derive(Debug, Serialize)]
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

    pub fn add_player(&mut self) -> Result<usize, &'static str> {
        // TODO: possibly implement shuffling players
        match self.players.len() {
            0..=3 => {
                self.players.push(Player { id: 0 });
                Ok(self.players.len())
            }
            _ => Err("table already full!"),
        }
    }
}
