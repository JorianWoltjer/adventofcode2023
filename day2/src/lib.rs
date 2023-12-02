use std::{error, str::FromStr};

type Err = Box<dyn error::Error>;

#[derive(Default, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}
impl FromStr for Round {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(", ");
        let mut round = Round::default();
        for set in s {
            match set
                .split_once(' ')
                .ok_or("no ' ' delimiter found in Round")?
            {
                (n, "red") => round.red = n.parse()?,
                (n, "green") => round.green = n.parse()?,
                (n, "blue") => round.blue = n.parse()?,
                (_, color) => return Err(format!("invalid color {color:?}").into()),
            }
        }
        Ok(round)
    }
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}
impl FromStr for Game {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rounds) = s
            .split_once(": ")
            .ok_or("no ': ' delimiter found in Game")?;

        let rounds = rounds
            .split("; ")
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self { rounds })
    }
}
impl Game {
    pub fn is_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.rounds
            .iter()
            .all(|r| r.red <= max_red && r.green <= max_green && r.blue <= max_blue)
    }

    // PART 2

    pub fn power(&self) -> u32 {
        let max_red = self.rounds.iter().map(|r| r.red).max().unwrap();
        let max_green = self.rounds.iter().map(|r| r.green).max().unwrap();
        let max_blue = self.rounds.iter().map(|r| r.blue).max().unwrap();

        max_red * max_green * max_blue
    }
}

pub fn count_valid(s: &str, max_red: u32, max_green: u32, max_blue: u32) -> Result<usize, Err> {
    let games: Vec<Game> = s
        .split('\n')
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok(games
        .iter()
        .enumerate()
        .map(|(i, game)| {
            if game.is_valid(max_red, max_green, max_blue) {
                i + 1 // IDs start counting at 1
            } else {
                0
            }
        })
        .sum())
}

// PART 2

pub fn minimum_playable(s: &str) -> Result<u32, Err> {
    let games: Vec<Game> = s
        .split('\n')
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok(games.iter().map(|game| game.power()).sum())
}
