use anyhow::{Context, Ok, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<usize> {
    let ground: Ground = input.try_into()?;

    let binding = ground.first_step_choice();
    let (first_position, second_positon) = binding
        .iter()
        .collect_tuple()
        .context("No initial step found")?;
    let mut first_position = *first_position;
    let mut second_positon = *second_positon;

    let mut steps = 1;

    while first_position.x != second_positon.x || first_position.y != second_positon.y {
        steps += 1;
        first_position = ground
            .next_step(&first_position)
            .context("No next step found")?;
        second_positon = ground
            .next_step(&second_positon)
            .context("No next step found")?;
    }

    Ok(steps)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    direction: Direction,
    x: usize,
    y: usize,
}

struct Ground {
    map: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Ground {
    fn first_step_choice(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        let (x, y) = self.start;

        if y > 0 {
            match self.map[y - 1][x] {
                '|' => positions.push(Position {
                    direction: Direction::Up,
                    x,
                    y: y - 1,
                }),
                'F' => positions.push(Position {
                    direction: Direction::Right,
                    x,
                    y: y - 1,
                }),
                '7' => positions.push(Position {
                    direction: Direction::Left,
                    x,
                    y: y - 1,
                }),
                _ => (),
            }
        }

        if y < self.map.len() - 1 {
            match self.map[y + 1][x] {
                '|' => positions.push(Position {
                    direction: Direction::Down,
                    x,
                    y: y + 1,
                }),
                'J' => positions.push(Position {
                    direction: Direction::Left,
                    x,
                    y: y + 1,
                }),
                'L' => positions.push(Position {
                    direction: Direction::Right,
                    x,
                    y: y + 1,
                }),
                _ => (),
            }
        }

        if x > 0 {
            match self.map[y][x - 1] {
                '-' => positions.push(Position {
                    direction: Direction::Left,
                    x: x - 1,
                    y,
                }),
                'L' => positions.push(Position {
                    direction: Direction::Up,
                    x: x - 1,
                    y,
                }),
                'F' => positions.push(Position {
                    direction: Direction::Down,
                    x: x - 1,
                    y,
                }),
                _ => (),
            }
        }

        if x < self.map[0].len() - 1 {
            match self.map[y][x + 1] {
                '-' => positions.push(Position {
                    direction: Direction::Right,
                    x: x + 1,
                    y,
                }),
                'J' => positions.push(Position {
                    direction: Direction::Up,
                    x: x + 1,
                    y,
                }),
                '7' => positions.push(Position {
                    direction: Direction::Down,
                    x: x + 1,
                    y,
                }),
                _ => (),
            }
        }

        positions
    }

    fn next_step(&self, position: &Position) -> Option<Position> {
        let (x, y) = (position.x, position.y);
        match position.direction {
            Direction::Up if y > 0 => match self.map[y - 1][x] {
                '|' => Some(Position {
                    direction: Direction::Up,
                    x,
                    y: y - 1,
                }),
                'F' => Some(Position {
                    direction: Direction::Right,
                    x,
                    y: y - 1,
                }),
                '7' => Some(Position {
                    direction: Direction::Left,
                    x,
                    y: y - 1,
                }),
                _ => None,
            },
            Direction::Down if y < self.map.len() => match self.map[y + 1][x] {
                '|' => Some(Position {
                    direction: Direction::Down,
                    x,
                    y: y + 1,
                }),
                'J' => Some(Position {
                    direction: Direction::Left,
                    x,
                    y: y + 1,
                }),
                'L' => Some(Position {
                    direction: Direction::Right,
                    x,
                    y: y + 1,
                }),
                _ => None,
            },
            Direction::Left if x > 0 => match self.map[y][x - 1] {
                '-' => Some(Position {
                    direction: Direction::Left,
                    x: x - 1,
                    y,
                }),
                'L' => Some(Position {
                    direction: Direction::Up,
                    x: x - 1,
                    y,
                }),
                'F' => Some(Position {
                    direction: Direction::Down,
                    x: x - 1,
                    y,
                }),
                _ => None,
            },
            Direction::Right if x < self.map[0].len() => match self.map[y][x + 1] {
                '-' => Some(Position {
                    direction: Direction::Right,
                    x: x + 1,
                    y,
                }),
                'J' => Some(Position {
                    direction: Direction::Up,
                    x: x + 1,
                    y,
                }),
                '7' => Some(Position {
                    direction: Direction::Down,
                    x: x + 1,
                    y,
                }),
                _ => None,
            },
            _ => None,
        }
    }
}

impl TryFrom<&str> for Ground {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let map = value
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let start = map
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter().enumerate().find_map(
                    |(x, c)| {
                        if *c == 'S' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .context("No starting position found")?;

        Ok(Self { map, start })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = include_str!("../example1");
        assert_eq!(super::part1(input).unwrap(), 4);

        let input = include_str!("../example2");
        assert_eq!(super::part1(input).unwrap(), 8);
    }
}
