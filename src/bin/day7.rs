use std::{cmp::Ordering, collections::HashMap};

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
struct Hand {
    cards: Vec<Card>,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        Self {
            cards: value.chars().map(|c| c.into()).collect(),
        }
    }
}

impl Hand {
    fn strength(&self) -> Strength {
        let mut card_counts_by_type = HashMap::new();
        self.cards
            .iter()
            .for_each(|c| *card_counts_by_type.entry(c).or_insert(0) += 1);
        let n_js = card_counts_by_type.remove(&Card::J).unwrap_or(0);
        let n_card_types = card_counts_by_type.len();
        if n_js == 5 || n_card_types == 1 {
            // There is only 1 card type OTHER THAN Js, so they + Js must be 5 of a kind
            // (or all Js)
            Strength::FiveOfAKind
        } else if n_card_types == 2 && card_counts_by_type.values().any(|&v| (v + n_js) == 4) {
            Strength::FourOfAKind
        } else if n_card_types == 2 {
            Strength::FullHouse
        } else if n_card_types == 3 && card_counts_by_type.values().any(|&v| (v + n_js) == 3) {
            Strength::ThreeOfAKind
        } else if n_card_types == 3 {
            Strength::TwoPair
        } else if n_card_types == 4 {
            Strength::OnePair
        } else {
            Strength::HighCard
        }
    }
    fn compare_eq_strength(&self, other: &Self) -> Ordering {
        for (own_card, other_card) in self.cards.iter().zip(&other.cards) {
            match own_card.cmp(other_card) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            }
        }
        Ordering::Equal
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
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.compare_eq_strength(other),
        }
    }
}

fn main() {
    let mut hands: Vec<(Hand, u32)> = include_str!("../day7/input.txt")
        .lines()
        .map(|line| {
            let (hand, value) = line.split_once(' ').unwrap();
            (hand.into(), value.parse().unwrap())
        })
        .collect();

    // sort and reverse (by virtue of cmp v2 to v1 instead of v1 to v2)
    hands.sort_by(|v1, v2| v2.0.cmp(&v1.0));
    println!(
        "Day 7 result: {}",
        hands
            .iter()
            .enumerate()
            .map(|(rank, (_, bet))| (rank + 1) as u32 * bet)
            .sum::<u32>()
    )
}
