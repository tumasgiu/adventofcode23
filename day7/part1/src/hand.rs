use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::{Display, Formatter};
use crate::card::Card;
use crate::hand_kind::HandKind;

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_kind().cmp(&other.hand_kind()) {
            Ordering::Equal => {
                let mut i = 0;
                loop {
                    let co = self.cards[i].cmp(&other.cards[i]);
                    if co != Ordering::Equal {
                        return co
                    }
                    i = i + 1;
                    if i == 5 {
                        break
                    }
                }
                Ordering::Equal
            }
            o => o
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Hand {
    pub fn hand_kind(&self) -> HandKind {
        let mut counts: HashMap<&Card, usize> = HashMap::new();
        for card in &self.cards {
            counts.insert(card, *counts.get(card).unwrap_or_else(|| &0usize) + 1);
        }

        let mut vals: Vec<&usize> = counts.values().collect();
        vals.sort_by(|a, b| b.cmp(a));

        match counts.len() {
            1 => HandKind::FiveOfAKind,
            2 => {
                if vals == vec![&4usize, &1usize] {
                    return HandKind::FourOfAKind
                }
                if vals == vec![&3usize, &2usize] {
                    return HandKind::FullHouse
                }
                panic!("unexpected!")
            },
            3 => {
                if vals == vec![&3usize, &1usize, &1usize] {
                    return HandKind::ThreeOfAKind
                }
                if vals == vec![&2usize, &2usize, &1usize] {
                    return HandKind::TwoPair
                }
                panic!("unexpected!")
            },
            4 => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(for c in &self.cards {
            match f.write_fmt(format_args!("{}", c)) {
                Ok(_) => {}
                Err(e) => { return Err(e) }
            }
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Err {
    MalformedInput,
    MalformedHand,
}

impl FromStr for Hand {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        if split.len() != 2 {
            return Err(Err::MalformedInput);
        }
        if split[0].len() != 5 {
            return Err(Err::MalformedHand);
        }
        let bid = match split[1].parse() {
            Ok(v) => v,
            _ => return Err(Err::MalformedInput),
        };

        let cards = split[0].chars()
            .map(|c| c.to_string())
            .map(|s| Card::from_str(s.as_str()))
            .map(|c| c.unwrap())
            .collect::<Vec<Card>>();
        Ok(Hand{
            cards,
            bid,
        })
    }
}
