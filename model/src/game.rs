use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn all() -> Vec<Rank> {
        return vec![Rank::Two, Rank::Three];
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn all() -> Vec<Suit> {
        vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck { cards: vec![] };
        for rank in Rank::all().iter() {
            for suit in Suit::all().iter() {
                deck.cards.push(Card {
                    rank: *rank,
                    suit: *suit,
                })
            }
        }
        // TODO shuffle in such a way that splitting 4-ways is callbreak valid
        deck.cards.shuffle(&mut thread_rng());
        return deck;
    }
}

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

impl Trick {
    fn winner(&self) -> Option<usize> {
        if self.cards.len() != 4 {
            return None;
        }
        let spades_max: Option<usize> = self
            .cards
            .iter()
            .enumerate()
            .filter(|(i, c)| c.suit == Suit::Spades)
            .max_by(|(_, a), (_, b)| a.rank.cmp(&b.rank))
            .map(|(index, _)| index);
        match spades_max {
            Some(v) => Some(v),
            None => Some({
                let starter_suit = self.cards[0].suit;
                self.cards
                    .iter()
                    .enumerate()
                    .filter(|(i, c)| c.suit == starter_suit)
                    .max_by(|(_, a), (_, b)| a.rank.cmp(&b.rank))
                    .unwrap()
                    .0
            }),
        }
    }

    fn next(self) -> usize {
        return self.cards.len();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Round {
    hands: Vec<Hand>,   // indexed by player
    tricks: Vec<Trick>, // indexed by trick number
    calls: Vec<u8>,     // indexed by player
}

impl Round {
    fn new() -> Round {
        return Round {
            hands: vec![],
            tricks: vec![],
            calls: vec![],
        };
    }

    fn trick_winner(&self, index: usize) -> usize {
        if index == 0 {
            return self.tricks[index].winner().unwrap();
        }

        return (self.trick_winner(index - 1) + self.tricks[index].winner().unwrap()) % 4;
    }

    fn next(&self) -> usize {
        if self.calls.len() != 4 {
            return self.calls.len();
        }
        let current_trick_index = self.tricks.len() - 1;
        let starter = self.trick_winner(current_trick_index - 1);
        (starter + self.tricks.last().unwrap().cards.len()) % 4
    }

    fn playcard(&mut self, card: Card) {
        let hand
    }
}

pub enum State {
    Calling,
    Breaking,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(serialize_with = "uuid::serde::urn::serialize")]
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

    pub fn add_player(&mut self, id: &String) -> Result<usize, &'static str> {
        if self.players.iter().any(|p| &p.id == id) {
            return Err("player is already in this game");
        }
        // TODO: possibly implement shuffling players
        match self.players.len() {
            0..=3 => {
                self.players.push(Player { id: id.to_owned() });
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

    pub fn playcard(&mut self, player: &String, card: Card) -> Result<(), &'static str> {
        let next = self.next();
        if self.players[next].id != *player {
            return Err("there was an error playing the card");
        }

        let mut round = self.rounds.last().unwrap();
        match round.playcard(next, card) {
            Err(_) => Err(""),
            Ok(v) => Ok(()),
        }
    }

    fn next(&self) -> usize {
        let current_round_index = self.rounds.len() - 1;
        let current_round = self.rounds.last().unwrap();
        (current_round_index + current_round.next()) % 4
    }

    // Looks like this should be start game
    pub fn start_round(&mut self) -> Result<(), &'static str> {
        if self.rounds.len() == 5 {
            return Err("the game is already over!");
        }
        if self.players.len() != 4 {
            return Err("not enough players to start the round");
        }
        if self.rounds.last().is_none() {
            self.rounds.push(Round {
                tricks: vec![],
                hands: vec![],
                calls: vec![],
            })
        };
        let mut new_round = Round {
            tricks: vec![],
            hands: vec![],
            calls: vec![],
        };
        for _ in 0..4 {
            new_round.hands.push(Hand { cards: vec![] })
        }
        let deck = Deck::new();
        for i in 0..52 {
            new_round.hands[i].cards.push(deck.cards[i]);
        }
        Ok(())
    }
}
