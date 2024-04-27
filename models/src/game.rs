#[derive(Debug)]
enum Card {}

#[derive(Debug)]
struct Player {
    id: u8, // at some point this will have to be UUID
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Trick {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Round {
    hands: Vec<Hand>,   // indexed by player
    tricks: Vec<Trick>, // indexed by trick number
    calls: Vec<u8>,     // indexed by player
    breaks: Vec<u8>,    // indexed by player
}

#[derive(Debug)]
pub struct Game {
    id: u8,
    players: Vec<Player>,
    rounds: Vec<Round>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            players: vec![],
            rounds: vec![],
        }
    }
}
