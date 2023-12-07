use std::{iter::once, str::FromStr};

use anyhow::{anyhow, Ok, Result};

use itertools::Itertools;
use tap::Tap;

pub fn day5_part1(input: &str) -> Result<isize> {
    let mut iter = input.split("\n\n");
    let seeds = iter
        .next()
        .and_then(|s| s.split_once(':'))
        .ok_or(anyhow!("Parse Seeds Fail"))?
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>())
        .collect::<Result<Vec<isize>, _>>()?;

    let tables = iter
        .map(|table| table.parse::<Table>())
        .process_results(|iter| iter.collect::<Vec<Table>>())?;

    tables
        .iter()
        .fold(seeds, |seeds, table| table.map_to_targets(seeds))
        .iter()
        .min()
        .ok_or(anyhow!("No Result"))
        .copied()
}

pub fn day5_part2(input: &str) -> Result<isize> {
    let mut iter = input.split("\n\n");

    let seeds = iter
        .next()
        .and_then(|s| s.split_once(':'))
        .ok_or(anyhow!("Parse Seeds Fail"))?
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>())
        .process_results(|iter| {
            iter.chunks(2)
                .into_iter()
                .flat_map(|mut c| {
                    let start = c.next().unwrap();
                    let range = c.next().unwrap();
                    start..start + range
                })
                .collect::<Vec<isize>>()
        })?
        .tap(|v| println!("{:#?}", v));

    let tables = iter
        .map(|table| table.parse::<Table>())
        .process_results(|iter| iter.collect::<Vec<Table>>())?;

    tables
        .iter()
        .fold(seeds, |seeds, table| table.map_to_targets(seeds))
        .iter()
        .min()
        .ok_or(anyhow!("No Result"))
        .copied()
}

#[derive(Debug, PartialEq)]
struct Mapping {
    source: isize,
    target: isize,
    range: isize,
}

#[derive(Debug, PartialEq)]
struct Table(Vec<Mapping>);

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ').map(|s| s.parse());

        let target = iter.next().ok_or(anyhow!("Parse Target Fail"))??;
        let source = iter.next().ok_or(anyhow!("Parse Source Fail"))??;
        let range = iter.next().ok_or(anyhow!("Parse Range Fail"))??;

        match iter.next() {
            None => Ok(Mapping {
                source,
                target,
                range,
            }),
            _ => Err(anyhow!("Parse Fail : {}", s)),
        }
    }
}

impl FromStr for Table {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let table = s
            .lines()
            .skip(1)
            .map(|line| line.parse::<Mapping>())
            .process_results(|iter| {
                once(Mapping {
                    source: 0,
                    target: 0,
                    range: 0,
                })
                .chain(iter.sorted_by_key(|mapping| mapping.source))
                .rev()
                .collect()
            })?;

        Ok(Table(table))
    }
}

impl Table {
    fn map_to_targets(&self, input: Vec<isize>) -> Vec<isize> {
        let mut input_iter = input.iter().sorted().rev();

        if let Some(mut i) = input_iter.next() {
            self.0.iter().fold(vec![], |mut acc, mapping| {
                while i >= &mapping.source {
                    if *i > mapping.source + mapping.range {
                        acc.push(*i);
                    } else {
                        acc.push(mapping.target + i - mapping.source);
                    }
                    i = input_iter.next().unwrap_or(&isize::MIN);
                }
                acc
            })
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example");

        assert_eq!(day5_part1(input).unwrap(), 35);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../example");

        assert_eq!(day5_part2(input).unwrap(), 46);
    }

    #[test]
    fn test_parse_mapping() {
        let input = "50 98 2";
        let mapping = input.parse::<Mapping>().unwrap();
        assert_eq!(
            mapping,
            Mapping {
                source: 98,
                target: 50,
                range: 2
            }
        );

        let input = "50 98 2 3";
        let mapping = input.parse::<Mapping>();
        assert!(mapping.is_err());

        let input = "50 98";
        let mapping = input.parse::<Mapping>();
        assert!(mapping.is_err());
    }

    #[test]
    fn test_parse_table() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";

        let table = input.parse::<Table>().unwrap();
        assert_eq!(
            table,
            Table(vec![
                Mapping {
                    source: 98,
                    target: 50,
                    range: 2
                },
                Mapping {
                    source: 50,
                    target: 52,
                    range: 48
                },
                Mapping {
                    source: 0,
                    target: 0,
                    range: 0
                },
            ])
        );
    }

    #[test]
    fn test_map_to_targets() {
        let table = "seed-to-soil map:
50 98 2
52 50 48"
            .parse::<Table>()
            .unwrap();

        let input = vec![79, 14, 55, 13];

        assert_eq!(table.map_to_targets(input), vec![81, 57, 14, 13]);
    }
}
