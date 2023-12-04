use std::{error, str::FromStr};

type Err = Box<dyn error::Error>;

struct Card {
    numbers: Vec<u8>,
    winning_numbers: Vec<u8>,
}
impl FromStr for Card {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once(": ").ok_or("No ': ' delimiter found")?;
        let (numbers, winning_numbers) = s.split_once(" | ").ok_or("No ' | ' delimiter found")?;

        Ok(Self {
            numbers: numbers
                .split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<_, _>>()?,
            winning_numbers: winning_numbers
                .split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}
impl Card {
    pub fn count_winning(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }

    pub fn score(&self) -> u32 {
        let count = self.count_winning();

        if count == 0 {
            0
        } else {
            2u32.pow(count as u32 - 1)
        }
    }
}

pub fn score_winning_numbers(s: &str) -> Result<u32, Err> {
    let cards: Vec<Card> = s
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok(cards.iter().map(|card| card.score()).sum())
}

// PART 2

#[derive(Clone)]
struct CardCopy {
    n: usize,
    count: usize,
}
impl From<Card> for CardCopy {
    fn from(value: Card) -> Self {
        Self {
            n: 1,
            count: value.count_winning(),
        }
    }
}

pub fn count_copies(s: &str) -> Result<usize, Err> {
    let cards: Vec<Card> = s
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let mut card_copies: Vec<CardCopy> = cards.into_iter().map(|card| card.into()).collect();

    for i in 0..card_copies.len() {
        let cc = card_copies[i].clone();
        for offset in 0..cc.count {
            card_copies[i + offset + 1].n += cc.n;
        }
    }

    Ok(card_copies.iter().map(|cc| cc.n).sum())
}
