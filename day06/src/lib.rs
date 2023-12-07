use anyhow::{anyhow, Result};
use tap::Tap;

pub fn day6_part1(input: &str) -> Result<u32> {
    let rounds = parse_round(input)?;

    let result: usize = rounds
        .iter()
        .map(|round| {
            round
                .get_win_times()
                // .tap(|v| println!("{:?}", v))
                .len()
            // .tap(|l| println!("{}: {}", round.time, l))
        })
        .product();

    Ok(result as u32)
}

pub fn day6_part2() {}

#[derive(Debug, PartialEq)]
struct Round {
    time: u32,
    distance: u32,
}

impl Round {
    fn get_win_times(&self) -> Vec<u32> {
        (0..self.time)
            .map(|t| {
                let speed = t;
                let time = self.time - t;
                (t, speed * time)
            })
            .filter_map(|(t, d)| if d > self.distance { Some(t) } else { None })
            .collect::<_>()
    }
}

fn parse_round(input: &str) -> Result<Vec<Round>> {
    let (time, distance) = input.split_once('\n').ok_or(anyhow!("No newline"))?;
    time.split_ascii_whitespace()
        .skip(1)
        .zip(distance.split_ascii_whitespace().skip(1))
        .map(|(t, d)| {
            let time = t.parse::<u32>()?;
            let distance = d.parse::<u32>()?;
            Ok(Round { time, distance })
        })
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        let input = include_str!("../example");
        assert_eq!(day6_part1(input).unwrap(), 288);
    }

    #[test]
    fn test_day6_part2() {}

    #[test]
    fn test_parse_round() {
        let input = include_str!("../example");
        let expected = vec![
            Round {
                time: 7,
                distance: 9,
            },
            Round {
                time: 15,
                distance: 40,
            },
            Round {
                time: 30,
                distance: 200,
            },
        ];
        assert_eq!(parse_round(input).unwrap(), expected);
    }

    #[test]
    fn test_get_win_times() {
        let round = Round {
            time: 7,
            distance: 9,
        };
        let expected = vec![2, 3, 4, 5];
        assert_eq!(round.get_win_times(), expected);
    }
}
