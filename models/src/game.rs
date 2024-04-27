enum Card {}

struct Player {
    id: u8, // at some point this will have to be UUID
}

struct Hand {
    cards: Vec<Card>,
}

struct Trick {
    cards: Vec<Card>,
}

struct Round {
    hands: Vec<Hand>,   // indexed by player
    tricks: Vec<Trick>, // indexed by trick number
    calls: Vec<u8>,     // indexed by player
    breaks: Vec<u8>,    // indexed by player
}

struct Game {
    id: u8,
    players: Vec<Player>,
    rounds: Vec<Round>,
}
