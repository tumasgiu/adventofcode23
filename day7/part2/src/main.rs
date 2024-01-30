mod card;
mod hand;
mod hand_kind;

use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;
use card::Card;
use hand::Hand;
use hand_kind::HandKind;

fn main() {
    let hands = parse_file("../input.txt").unwrap();

    println!("Part 2 Answer: {}", total_winnings(hands))
}

fn total_winnings(mut hands: Vec<Hand>) -> usize {
    hands.sort_by(|a, b| a.cmp(b));

    let mut total_winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = i + 1;
        total_winnings += rank * hand.bid
    }

    total_winnings
}

fn parse_file(filename: &str) -> Result<Vec<Hand>, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut hands = Vec::new();
    for line in reader.lines() {
        let line = line?;
        match line.parse::<Hand>() {
            Ok(hand) => hands.push(hand),
            Err(_e) => continue,
        }
    }

    Ok(hands)
}


#[test]
fn test_hand() {
    let expected_hands = vec![
        Hand {
            cards: vec![
                Card::Three,
                Card::Two,
                Card::T,
                Card::Three,
                Card::K,
            ],
            bid: 765,
        },
        Hand {
            cards: vec![
                Card::T,
                Card::Five,
                Card::Five,
                Card::J,
                Card::Five,
            ],
            bid: 684,
        },
        Hand {
            cards: vec![
                Card::K,
                Card::K,
                Card::Six,
                Card::Seven,
                Card::Seven,
            ],
            bid: 28,
        },
        Hand {
            cards: vec![
                Card::K,
                Card::T,
                Card::J,
                Card::J,
                Card::T,
            ],
            bid: 220,
        },
        Hand {
            cards: vec![
                Card::Q,
                Card::Q,
                Card::Q,
                Card::J,
                Card::A,
            ],
            bid: 483,
        }
    ];

    let hands = parse_file("../test-input.txt").unwrap();

    assert_eq!(expected_hands, hands);

    assert_eq!(expected_hands[0].hand_kind(), HandKind::OnePair);
    assert_eq!(expected_hands[1].hand_kind(), HandKind::FourOfAKind);
    assert_eq!(expected_hands[2].hand_kind(), HandKind::TwoPair);
    assert_eq!(expected_hands[3].hand_kind(), HandKind::FourOfAKind);
    assert_eq!(expected_hands[4].hand_kind(), HandKind::FourOfAKind);
    assert_eq!(Hand::from_str("AAAAA 123").unwrap().hand_kind(), HandKind::FiveOfAKind);
    assert_eq!(Hand::from_str("22333 123").unwrap().hand_kind(), HandKind::FullHouse);
    assert_eq!(Hand::from_str("JJ4JJ 123").unwrap().hand_kind(), HandKind::FiveOfAKind);
    assert_eq!(Hand::from_str("33633 123").unwrap().hand_kind(), HandKind::FourOfAKind);
    assert_eq!(Hand::from_str("2749A 123").unwrap().hand_kind(), HandKind::HighCard);
    assert_eq!(Hand::from_str("24226 123").unwrap().hand_kind(), HandKind::ThreeOfAKind);


    assert!(HandKind::FiveOfAKind > HandKind::FourOfAKind);
    assert!(HandKind::FourOfAKind > HandKind::FullHouse);
    assert!(HandKind::FullHouse > HandKind::ThreeOfAKind);
    assert!(HandKind::ThreeOfAKind > HandKind::TwoPair);
    assert!(HandKind::TwoPair > HandKind::OnePair);
    assert!(HandKind::OnePair > HandKind::HighCard);

    assert!(Card::A > Card::K);
    assert!(Card::K > Card::Q);
    assert!(Card::Q > Card::T);
    assert!(Card::T > Card::Nine);
    assert!(Card::Nine > Card::Eight);
    assert!(Card::Eight > Card::Seven);
    assert!(Card::Seven > Card::Six);
    assert!(Card::Six > Card::Five);
    assert!(Card::Five > Card::Four);
    assert!(Card::Four > Card::Three);
    assert!(Card::Three > Card::Two);
    assert!(Card::Two > Card::J);

    assert!(Hand::from_str("K749A 123").unwrap() > Hand::from_str("3749A 123").unwrap());
    assert!(Hand::from_str("3849A 123").unwrap() > Hand::from_str("3749A 123").unwrap());
    assert!(Hand::from_str("3769A 123").unwrap() > Hand::from_str("3749A 123").unwrap());
    assert!(Hand::from_str("374AA 123").unwrap() > Hand::from_str("3749A 123").unwrap());

    assert_eq!(total_winnings(hands), 5905)
}