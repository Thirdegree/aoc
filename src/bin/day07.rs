#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use std::{cmp::Ordering, collections::HashMap};
// For part 1, switch the Part value in the hand constructer, and move J below between Q and T

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    A,
    K,
    Q,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
    J,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::_9,
            '8' => Self::_8,
            '7' => Self::_7,
            '6' => Self::_6,
            '5' => Self::_5,
            '4' => Self::_4,
            '3' => Self::_3,
            '2' => Self::_2,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Strength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq, Debug)]
#[allow(dead_code)]
enum Part {
    Part1,
    Part2
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    part: Part
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        Self {
            cards: value.chars().map(Into::into).collect(),
            part: Part::Part2
        }
    }
}

impl Hand {
    fn strength(&self) -> Strength {
        let mut card_counts_by_type = HashMap::new();
        self.cards
            .iter()
            .for_each(|c| *card_counts_by_type.entry(c).or_insert(0) += 1);
        let n_js = if self.part == Part::Part2 {
            card_counts_by_type.remove(&Card::J).unwrap_or(0)
        } else {
            0
        };
        let n_card_types = card_counts_by_type.len();
        match n_card_types {
            0 | 1 => Strength::FiveOfAKind,
            2 if card_counts_by_type.values().any(|&v| (v + n_js) == 4) => Strength::FourOfAKind,
            2 => Strength::FullHouse,
            3 if card_counts_by_type.values().any(|&v| (v + n_js) == 3) => Strength::ThreeOfAKind,
            3 => Strength::TwoPair,
            4 => Strength::OnePair,
            _ => Strength::HighCard,
        }
    }
    fn compare_eq_strength(&self, other: &Self) -> Ordering {
        self.cards
            .iter()
            .zip(&other.cards)
            .find_map(|(own_card, other_card)| match own_card.cmp(other_card) {
                v @ (Ordering::Greater | Ordering::Less) => Some(v),
                Ordering::Equal => None,
            })
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength().cmp(&other.strength()) {
            v @ (Ordering::Greater | Ordering::Less) => v,
            Ordering::Equal => self.compare_eq_strength(other),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut hands: Vec<(Hand, u32)> = aoc_2023::include_data!(day7)
        .lines()
        .map(|line| {
            let (hand, value) = line.split_once(' ').unwrap();
            (hand.into(), value.parse().unwrap())
        })
        .collect();

    hands.sort_by(|v1, v2| v1.0.cmp(&v2.0).reverse());
    println!(
        "Day 7 result: {}",
        hands
            .iter()
            .enumerate()
            .map(|(rank, (_, bet))| Ok(u32::try_from(rank + 1)? * bet))
            .sum::<anyhow::Result<u32>>()?
    );
    Ok(())
}
