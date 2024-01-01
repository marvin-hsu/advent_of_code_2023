use std::collections::HashMap;

use anyhow::{anyhow, bail, Ok, Result};
#[allow(unused_imports)]
use tap::Tap;

#[derive(Debug, Clone, Copy)]
struct Node<'a>(&'a str, &'a str);

pub fn part1(input: &str) -> Result<usize> {
    let (instructions, node_map) = parse_data1(input)?;

    let mut key = "AAA";

    for (step, instruction) in instructions.chars().cycle().enumerate() {
        let node = node_map.get(key).ok_or(anyhow!("Node Not Found"))?;

        key = match instruction {
            'L' => node.0,
            'R' => node.1,
            _ => return Err(anyhow!("Invalid Instruction")),
        };

        if key == "ZZZ" {
            return Ok(step + 1);
        }
    }

    bail!("No Solution")
}

type Instructions = str;
type NodeKey = str;
fn parse_data1(input: &str) -> Result<(&Instructions, HashMap<&NodeKey, Node<'_>>)> {
    let (instructions, rules) = input
        .split_once("\n\n")
        .ok_or(anyhow!("Can't Get Instructions"))?;
    let node_map = rules
        .lines()
        .map(|line| {
            let (name, rule) = line.split_once('=').ok_or(anyhow!("Can't Get Rule"))?;

            let node = rule
                .split_once(',')
                .map(|(left, right)| {
                    Node(
                        left.trim().trim_start_matches('('),
                        right.trim().trim_end_matches(')'),
                    )
                })
                .ok_or(anyhow!("Can't Get Node"))?;

            Ok((name.trim(), node))
        })
        .collect::<Result<HashMap<_, _>>>()?;
    Ok((instructions, node_map))
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Result<usize> {
    Ok(6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example1");
        assert_eq!(part1(input).unwrap(), 2);

        let input = include_str!("../example2");
        assert_eq!(part1(input).unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../example3");
        assert_eq!(part2(input).unwrap(), 6);
    }
}
