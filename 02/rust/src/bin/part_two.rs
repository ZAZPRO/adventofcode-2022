use std::error::Error;

// Enum that describes match result.
enum Outcome {
    Win,
    Draw,
    Lose,
}

// Trait for outcome to calculate result score.
impl Outcome {
    pub fn get_score(&self) -> u32 {
        match &self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

// Possible Items that players can throw.
#[derive(Clone)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

impl Item {
    // Value of each Item.
    pub fn get_score(&self) -> u32 {
        match &self {
            Item::Rock => 1,
            Item::Paper => 2,
            Item::Scissors => 3,
        }
    }

    // Returns an Item that we must use to get desired outcome using current Item.
    pub fn get_item_from_outcome(&self, outcome: &Outcome) -> Self {
        match (&self, &outcome) {
            (Item::Rock, Outcome::Win) => Item::Paper,
            (Item::Rock, Outcome::Lose) => Item::Scissors,
            (Item::Paper, Outcome::Win) => Item::Scissors,
            (Item::Paper, Outcome::Lose) => Item::Rock,
            (Item::Scissors, Outcome::Win) => Item::Rock,
            (Item::Scissors, Outcome::Lose) => Item::Paper,
            (_, Outcome::Draw) => self.clone(),
        }
    }
}

// Struct that holds each game Round.
struct Round {
    opponent: Item,
    player: Item,
}

impl Round {
    // Common Rust way to create structs.
    pub fn new(opponent: Item, player: Item) -> Self {
        Self { opponent, player }
    }

    // Calculate outcome of the Round.
    pub fn get_outcome(&self) -> Outcome {
        match (&self.player, &self.opponent) {
            (Item::Rock, Item::Scissors) => Outcome::Win,
            (Item::Rock, Item::Paper) => Outcome::Lose,
            (Item::Paper, Item::Rock) => Outcome::Win,
            (Item::Paper, Item::Scissors) => Outcome::Lose,
            (Item::Scissors, Item::Paper) => Outcome::Win,
            (Item::Scissors, Item::Rock) => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }

    // Calculate score of the round.
    pub fn calc_score(&self) -> u32 {
        &self.player.get_score() + &self.get_outcome().get_score()
    }
}

// Program entry point.
fn main() -> Result<(), Box<dyn Error>> {
    // List of game rounds.
    let mut rounds: Vec<Round> = Vec::new();

    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // For each line in the file.
    for line in file.lines() {
        // Split a line in two values.
        let mut iter = line.split_whitespace();

        // First value is what current opponent is throwing.
        let current_opponent = match iter.next() {
            Some(v) => match v {
                "A" => Item::Rock,
                "B" => Item::Paper,
                "C" => Item::Scissors,
                _ => todo!(),
            },
            None => todo!(),
        };

        // Second value is what outcome we must get.
        let current_outcome = match iter.next() {
            Some(v) => match v {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => todo!(),
            },
            None => todo!(),
        };

        // Get player item with the outcome we need to fulfill.
        let current_player = current_opponent.get_item_from_outcome(&current_outcome);

        // Create a new Round struct with opponent and player values.
        rounds.push(Round::new(current_opponent, current_player));
    }

    // Calculate a final sum by calling a score calculation method of Round struct and summing those up.
    let final_sum: u32 = rounds.into_iter().map(|r| r.calc_score()).sum();
    println!("Final Score: {final_sum}");

    Ok(())
}
