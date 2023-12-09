use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn day4_part1(input: &str) -> Result<usize> {
    input
        .lines()
        .map(|line| line.parse::<ScratchCard>())
        .process_results(|cards| cards.map(|card| card.get_points()).sum())
}

pub fn day4_part2(input: &str) -> Result<usize> {
    let cards: Vec<ScratchCard> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_>>()?;

    let cards_len = cards.len();

    let result = cards
        .iter()
        .fold(HashMap::new(), |mut map, card| {
            let count = card.count_matching_numbers();
            let instances = map.entry(card.id).or_insert(0);
            *instances += 1;

            if count > 0 {
                let instances = *instances;

                for id in (card.id + 1)..=(card.id + count).min(cards_len) {
                    *map.entry(id).or_insert(0) += instances;
                }
            }
            map
        })
        .values()
        .sum();

    Ok(result)
}

/// Replace HashMap with VecDeque
#[allow(unused_variables)]
pub fn day4_part2_v2(input: &str) -> usize {
    todo!()
}

#[derive(Debug, PartialEq, Clone)]
struct ScratchCard {
    id: usize,
    wining_numbers: HashSet<usize>,
    scratch_numbers: HashSet<usize>,
}

impl ScratchCard {
    fn count_matching_numbers(&self) -> usize {
        self.wining_numbers
            .intersection(&self.scratch_numbers)
            .count()
    }

    fn get_points(&self) -> usize {
        self.count_matching_numbers()
            .checked_add_signed(-1)
            .map(|pow| 2_usize.pow(pow as u32))
            .unwrap_or(0)
    }
}

impl FromStr for ScratchCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (head, tail) = s.split_once(':').context("Parse Line Fail.")?;

        let id: usize = head
            .split_once(' ')
            .and_then(|(_, id)| id.trim().parse().ok())
            .context("Parse Id Fail.")?;

        let (head, tail) = tail.split_once('|').context("Parse Numbers Fail.")?;

        let wining_numbers = head
            .split_ascii_whitespace()
            .map(|n| n.trim().parse())
            .process_results(|iter| iter.collect::<HashSet<_>>())?;

        let scratch_numbers = tail
            .split_ascii_whitespace()
            .map(|n| n.trim().parse())
            .process_results(|iter| iter.collect::<HashSet<_>>())?;

        Ok(Self {
            id,
            wining_numbers,
            scratch_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example");

        assert_eq!(day4_part1(input).unwrap(), 13);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../example");

        assert_eq!(day4_part2(input).unwrap(), 30);
    }

    #[test]
    fn test_parse_scratch_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let card = input.parse::<ScratchCard>().unwrap();

        assert_eq!(
            card,
            ScratchCard {
                id: 1,
                wining_numbers: vec![41, 48, 83, 86, 17].into_iter().collect(),
                scratch_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect(),
            }
        );
    }
}
