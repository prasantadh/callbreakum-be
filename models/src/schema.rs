use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RoundHand {
    round: i32,
    hand: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RoundScore {
    round: i32,
    score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    id: i32,
    hands: Vec<RoundHand>,
    scores: Vec<RoundScore>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerScore {
    player: i32,
    score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerCard {
    player: i32,
    card: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Trick {
    id: i32,
    starter: i32,
    cards: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Round {
    id: i32,
    starter: i32,
    tricks: Vec<String>,
    scores: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    players: Vec<Player>,
    rounds: Vec<Round>,
}

#[cfg(test)]
mod tests {

    use std::vec;

    use crate::schema::Schema;

    #[test]
    fn new_works() {
        let _game = Schema {
            players: vec![],
            rounds: vec![],
        };
        // println!("{:?} {:?} {:?}", game, game.players, game.rounds);
    }

    #[test]
    fn deserialize_works_on_empty_doc() -> Result<(), serde_json::Error> {
        let data = r#"
        {
            "players" : [],
            "rounds" : []
        }
        "#;
        let _v: Schema = serde_json::from_str(data)?;
        Ok(())
    }
}
