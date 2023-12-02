use std::str::FromStr;

pub fn day2_part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| Game::from_str(line).ok())
        .filter(|game| is_passible(game))
        .map(|game| game.id)
        .sum()
}

pub fn day2_part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| Game::from_str(line).ok())
        .map(|game| get_power(game))
        .sum()
}

#[derive(Debug)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

#[derive(Debug)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32,
}

impl CubeSet {
    fn new() -> CubeSet {
        CubeSet {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let (head, tail) = s.split_once(":").ok_or(())?;

        let id = head.split_once(" ").and_then(|h| h.1.parse().ok()).ok_or(())?;

        let cub_set = tail
            .split(";")
            .map(|set| {
                set.split(",").fold(Ok(CubeSet::new()), |acc, item| {
                    if let Ok(acc) = acc {
                        match item.trim() {
                            s if s.ends_with("red") => Ok(CubeSet {
                                red: s.split_once(" ").unwrap().0.parse().unwrap(),
                                ..acc
                            }),
                            s if s.ends_with("blue") => Ok(CubeSet {
                                blue: s.split_once(" ").unwrap().0.parse().unwrap(),
                                ..acc
                            }),
                            s if s.ends_with("green") => Ok(CubeSet {
                                green: s.split_once(" ").unwrap().0.parse().unwrap(),
                                ..acc
                            }),
                            _ => Err(()),
                        }
                    } else {
                        acc
                    }
                })
            })
            .collect::<Result<Vec<CubeSet>, ()>>()?;

        Ok(Game {
            id,
            cube_sets: cub_set,
        })
    }
}

// only 12 red cubes, 13 green cubes, and 14 blue cubes?
fn is_passible(game: &Game) -> bool {
    game.cube_sets
        .iter()
        .all(|set| set.red <= 12 && set.blue <= 14 && set.green <= 13)
}

fn get_power(game: Game) -> u32 {
    let (red,blue,green) = game.cube_sets.iter().fold((0,0,0),|acc, set| {
        (
            acc.0.max(set.red),
            acc.1.max(set.blue),
            acc.2.max(set.green),
        )
    });

    println!("{} * {} * {} = {} ", red, blue, green, red * blue * green);

    red * blue * green
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(day2_part1(input), 8);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("input").unwrap();

        assert_eq!(day2_part1(&input), 2541);
    }

    #[test]
    fn part2_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(day2_part2(input), 2286);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("input").unwrap();

        assert_eq!(day2_part2(&input), 66016);
    }

    #[test]
    fn test_parse() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = Game::from_str(input).unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.cube_sets.len(), 3);
        assert_eq!(game.cube_sets[0].red, 4);
        assert_eq!(game.cube_sets[0].blue, 3);
        assert_eq!(game.cube_sets[0].green, 0);
        assert_eq!(game.cube_sets[1].red, 1);
        assert_eq!(game.cube_sets[1].blue, 6);
        assert_eq!(game.cube_sets[1].green, 2);
        assert_eq!(game.cube_sets[2].red, 0);
        assert_eq!(game.cube_sets[2].blue, 0);
        assert_eq!(game.cube_sets[2].green, 2);
    }
}