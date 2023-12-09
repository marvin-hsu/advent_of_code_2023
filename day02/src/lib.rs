use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Result};
use itertools::Itertools;

const LIMIT: Limit = Limit {
    red: 12,
    blue: 14,
    green: 13,
};

pub fn day2_part1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| line.parse::<Game>())
        .process_results(|games| {
            games
                .filter(|game| game.is_passible(&LIMIT))
                .map(|game| game.id)
                .sum()
        })
}

pub fn day2_part2(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| line.parse::<Game>())
        .process_results(|games| games.map(|game| game.get_power()).sum())
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32,
}

struct Limit {
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    fn get_power(&self) -> u32 {
        let (red, blue, green) = self.cube_sets.iter().fold((0, 0, 0), |acc, set| {
            (
                acc.0.max(set.red),
                acc.1.max(set.blue),
                acc.2.max(set.green),
            )
        });

        red * blue * green
    }

    fn is_passible(&self, limit: &Limit) -> bool {
        self.cube_sets
            .iter()
            .all(|set| set.red <= limit.red && set.blue <= limit.blue && set.green <= limit.green)
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (head, tail) = s.split_once(':').context("Parse Line Fail.")?;

        let id = head
            .split_once(' ')
            .and_then(|h| h.1.parse().ok())
            .context("Parse Id Fail.")?;

        let cube_sets = tail
            .split(';')
            .map(|set| {
                let map = set
                    .split(',')
                    .filter_map(|cube| {
                        cube.trim()
                            .split_once(' ')
                            .map(|(v, k)| (k, v.parse::<u32>().unwrap_or(0)))
                    })
                    .collect::<HashMap<_, _>>();

                CubeSet {
                    red: *map.get("red").unwrap_or(&0),
                    blue: *map.get("blue").unwrap_or(&0),
                    green: *map.get("green").unwrap_or(&0),
                }
            })
            .collect();

        Ok(Game { id, cube_sets })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example");

        assert_eq!(day2_part1(input).unwrap(), 8);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../example");

        assert_eq!(day2_part2(input).unwrap(), 2286);
    }

    #[test]
    fn test_parse() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(
            input.parse::<Game>().unwrap(),
            Game {
                id: 1,
                cube_sets: vec![
                    CubeSet {
                        red: 4,
                        blue: 3,
                        green: 0
                    },
                    CubeSet {
                        red: 1,
                        blue: 6,
                        green: 2
                    },
                    CubeSet {
                        red: 0,
                        blue: 0,
                        green: 2
                    },
                ],
            }
        );
    }
}
