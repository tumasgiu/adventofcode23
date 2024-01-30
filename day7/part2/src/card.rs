use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::A => write!(f, "{}", "A"),
            Card::K => write!(f, "{}", "K"),
            Card::Q => write!(f, "{}", "Q"),
            Card::T => write!(f, "{}", "T"),
            Card::Nine => write!(f, "{}", "9"),
            Card::Eight => write!(f, "{}", "8"),
            Card::Seven => write!(f, "{}", "7"),
            Card::Six => write!(f, "{}", "6"),
            Card::Five => write!(f, "{}", "5"),
            Card::Four => write!(f, "{}", "4"),
            Card::Three => write!(f, "{}", "3"),
            Card::Two => write!(f, "{}", "2"),
            Card::J => write!(f, "{}", "J"),
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "T" => Ok(Card::T),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            "J" => Ok(Card::J),
            _ => Err(()),
        }
    }
}

impl Card {
    fn strength(&self) -> u64 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::T => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 3,
            Card::Three => 2,
            Card::Two => 1,
            Card::J => 0,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}
