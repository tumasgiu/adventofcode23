use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for HandKind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for HandKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HandKind {
    fn strength(&self) -> usize {
        match self {
            HandKind::FiveOfAKind => 7,
            HandKind::FourOfAKind => 6,
            HandKind::FullHouse => 5,
            HandKind::ThreeOfAKind => 4,
            HandKind::TwoPair => 3,
            HandKind::OnePair => 2,
            HandKind::HighCard => 1,
        }
    }
}
