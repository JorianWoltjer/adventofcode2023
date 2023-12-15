use std::error;

type Err = Box<dyn error::Error>;

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum Card {
    Joker,
    Two,
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
impl Card {
    fn from_char(c: char, joker: bool) -> Option<Card> {
        Some(match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => {
                if joker {
                    Card::Joker
                } else {
                    Card::Jack
                }
            }
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => return None,
        })
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl HandKind {
    fn get_counts(cards: &[Card; 5]) -> [usize; 14] {
        let mut counts = [0; 14];
        for card in cards {
            counts[*card as usize] += 1;
        }
        counts.sort();
        counts.reverse();
        counts
    }

    fn get_counts_joker(cards: &[Card; 5]) -> [usize; 14] {
        let mut counts = [0; 14];
        let mut jokers = 0;
        for card in cards {
            if *card == Card::Joker {
                jokers += 1;
            } else {
                counts[*card as usize] += 1;
            }
        }
        counts.sort();
        counts.reverse();
        counts[0] += jokers; // It is always best to add jokers to the highest count
        counts
    }

    fn from_counts(counts: [usize; 14]) -> HandKind {
        match counts {
            [5, ..] => HandKind::FiveOfAKind,
            [4, 1, ..] => HandKind::FourOfAKind,
            [3, 2, ..] => HandKind::FullHouse,
            [3, ..] => HandKind::ThreeOfAKind,
            [2, 2, ..] => HandKind::TwoPair,
            [2, ..] => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Hand {
    kind: HandKind,
    cards: [Card; 5],
    bid: u32,
}
impl Hand {
    fn from_line(s: &str, joker: bool) -> Result<Self, Err> {
        let (cards, bid) = s.split_once(' ').ok_or("Cannot find ' ' delimiter")?;
        let cards = cards
            .chars()
            .map(|c| Card::from_char(c, joker).ok_or("Invalid card"))
            .collect::<Result<Vec<_>, _>>()?;

        let cards = cards.try_into().map_err(|_| "Invalid number of cards")?;
        let kind = HandKind::from_counts(if joker {
            HandKind::get_counts_joker(&cards)
        } else {
            HandKind::get_counts(&cards)
        });
        let bid = bid.parse()?;

        Ok(Self { cards, kind, bid })
    }
}

pub fn order_hands(s: &str) -> Result<u32, Err> {
    let mut hands: Vec<Hand> = s
        .lines()
        .map(|line| Hand::from_line(line, false))
        .collect::<Result<_, _>>()?;

    hands.sort();

    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum())
}

// PART 2

pub fn order_hands_joker(s: &str) -> Result<u32, Err> {
    let mut hands: Vec<Hand> = s
        .lines()
        .map(|line| Hand::from_line(line, true))
        .collect::<Result<_, _>>()?;

    hands.sort();

    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum())
}
