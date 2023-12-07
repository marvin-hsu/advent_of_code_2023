use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use tap::Tap;

pub fn part1(input: &str) -> Result<usize> {
    let input = input
        .lines()
        .map(|line| {
            line.split_once(' ').and_then(|(head, tail)| {
                let card_type = head.parse::<CardType>().ok()?;
                let bet_number = tail.parse::<usize>().ok()?;
                Some((bet_number, card_type))
            })
        })
        .collect::<Option<Vec<(usize, CardType)>>>()
        .ok_or(anyhow!("Invalid input"))?;

    let result = input
        .iter()
        .sorted_by_key(|card| card.1)
        .enumerate()
        .map(|card| (card.0 + 1) * card.1 .0)
        .sum();

    Ok(result)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Label {
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

impl TryFrom<char> for Label {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Label> {
        match c {
            '2' => Ok(Label::Two),
            '3' => Ok(Label::Three),
            '4' => Ok(Label::Four),
            '5' => Ok(Label::Five),
            '6' => Ok(Label::Six),
            '7' => Ok(Label::Seven),
            '8' => Ok(Label::Eight),
            '9' => Ok(Label::Nine),
            'T' => Ok(Label::Ten),
            'J' => Ok(Label::Jack),
            'Q' => Ok(Label::Queen),
            'K' => Ok(Label::King),
            'A' => Ok(Label::Ace),
            _ => Err(anyhow::anyhow!("Invalid label: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardType {
    HighCard([Label; 5]),
    OnePair([Label; 5]),
    TwoPair([Label; 5]),
    ThreeKind([Label; 5]),
    FullHouse([Label; 5]),
    FourKind([Label; 5]),
    FiveKind([Label; 5]),
}

impl FromStr for CardType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<CardType> {
        let labels: [Label; 5] = s
            .chars()
            .map(|c| Label::try_from(c))
            .collect::<Result<Vec<Label>>>()?
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid card type: {}", s))?;

        let map = labels.iter().fold(HashMap::new(), |mut map, label| {
            *map.entry(label).or_insert(0) += 1;
            map
        });

        if map.iter().any(|(_, &count)| count == 5) {
            Ok(CardType::FiveKind(labels))
        } else if map.iter().any(|(_, &count)| count == 4) {
            Ok(CardType::FourKind(labels))
        } else if map.iter().any(|(_, &count)| count == 3) {
            if map.iter().any(|(_, &count)| count == 2) {
                Ok(CardType::FullHouse(labels))
            } else {
                Ok(CardType::ThreeKind(labels))
            }
        } else if map.iter().filter(|(_, &count)| count == 2).count() == 2 {
            Ok(CardType::TwoPair(labels))
        } else if map.iter().any(|(_, &count)| count == 2) {
            Ok(CardType::OnePair(labels))
        } else {
            Ok(CardType::HighCard(labels))
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::CardType;

    use super::{CardType::*, Label::*};

    #[test]
    fn test_part_1() {
        let input = include_str!("../example");
        assert_eq!(super::part1(input).unwrap(), 6440);
    }

    #[test]
    fn test_compare_label() {
        assert!(Ace > King);
        assert!(King > Queen);
        assert!(Queen > Jack);
        assert!(Jack > Ten);
        assert!(Ten > Nine);
        assert!(Nine > Eight);
        assert!(Eight > Seven);
        assert!(Seven > Six);
        assert!(Six > Five);
        assert!(Five > Four);
        assert!(Four > Three);
        assert!(Three > Two);
    }

    #[test]
    fn test_compare_card_type() {
        // 32T3K 765
        // T55J5 684
        // KK677 28
        // KTJJT 220
        // QQQJA 483

        let input = [
            OnePair([Three, Two, Ten, Three, King]),
            ThreeKind([Ten, Five, Five, Jack, Five]),
            TwoPair([King, King, Six, Seven, Seven]),
            TwoPair([King, Ten, Jack, Jack, Ten]),
            ThreeKind([Queen, Queen, Queen, Jack, Ace]),
        ]
        .iter()
        .enumerate()
        .sorted_by_key(|card| card.1)
        .map(|card| card.0)
        .collect_vec();

        assert_eq!(input, [0, 3, 2, 1, 4]);
    }

    #[test]
    fn test_parse_card_type() {
        assert_eq!(
            "32T3K".parse::<CardType>().unwrap(),
            OnePair([Three, Two, Ten, Three, King])
        );
        assert_eq!(
            "76584".parse::<CardType>().unwrap(),
            HighCard([Seven, Six, Five, Eight, Four])
        );
        assert_eq!(
            "KK677".parse::<CardType>().unwrap(),
            TwoPair([King, King, Six, Seven, Seven])
        );
        assert_eq!(
            "KTJJT".parse::<CardType>().unwrap(),
            TwoPair([King, Ten, Jack, Jack, Ten])
        );
        assert_eq!(
            "QQQJA".parse::<CardType>().unwrap(),
            ThreeKind([Queen, Queen, Queen, Jack, Ace])
        );
    }
}
