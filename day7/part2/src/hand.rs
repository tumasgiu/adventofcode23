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
        let mut counts_by_cards: HashMap<&Card, usize> = HashMap::new();

        for card in &self.cards {
            let count = *counts_by_cards.get(card).unwrap_or_else(|| &0usize) + 1;
            counts_by_cards.insert(card, count);
        }

        counts_by_cards = match counts_by_cards.get(&Card::J) {
            Some(count) => {
                let mut card_counts: Vec<(&&Card, &usize)> = counts_by_cards.iter().filter(|e| e.0 != &&Card::J).collect();
                // highest count then highest strength
                card_counts.sort_by(|a, b| {
                    match b.1.cmp(&a.1) {
                        Ordering::Equal => b.0.cmp(&a.0),
                        o => o,
                    }
                });
                let mut new_cc = counts_by_cards.clone();
                new_cc.remove(&Card::J);
                let mut card: &Card;
                let mut new_count: usize;
                match card_counts.len() {
                    0 => {
                        card = &Card::A;
                        new_count = *count;
                    },
                    _ => {
                        card = card_counts[0].0;
                        new_count = counts_by_cards.get(card).unwrap() + count
                    },
                };
                new_cc.insert(card, new_count);
                new_cc
            }
            None => counts_by_cards
        };

        let mut counts: Vec<&usize> = counts_by_cards.values().collect();
        counts.sort_by(|a, b| b.cmp(a));

        match counts.len() {
            1 => HandKind::FiveOfAKind,
            2 => {
                if counts == vec![&4usize, &1usize] {
                    return HandKind::FourOfAKind
                }
                if counts == vec![&3usize, &2usize] {
                    return HandKind::FullHouse
                }
                panic!("unexpected!")
            },
            3 => {
                if counts == vec![&3usize, &1usize, &1usize] {
                    return HandKind::ThreeOfAKind
                }
                if counts == vec![&2usize, &2usize, &1usize] {
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
