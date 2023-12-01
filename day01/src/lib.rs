use std::str::FromStr;

pub fn day1_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();

            let first = nums.first().map(|n| n * 10);
            let last = nums.last();

            (first.unwrap() + last.unwrap()) as usize
        })
        .sum()
}

pub fn day1_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;

            let mut start = 0;
            let mut end = 1;

            while !(start == line.len() && end > line.len()) {
                if let Ok(n) = line[start..end].parse::<Number>() {
                    if first == 0 {
                        first = n as i32;
                    }

                    last = n as i32;

                    start += 1;
                    end = start + 1;
                } else if end - start < 6 && end < line.len(){
                    end += 1;
                } else {
                    start += 1;
                    end = start + 1;
                }
            }

            (first * 10 + last) as usize
        })
        .sum()
}

#[derive(Clone, Copy)]
enum Number {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "one" => Ok(Number::One),
            "2" | "two" => Ok(Number::Two),
            "3" | "three" => Ok(Number::Three),
            "4" | "four" => Ok(Number::Four),
            "5" | "five" => Ok(Number::Five),
            "6" | "six" => Ok(Number::Six),
            "7" | "seven" => Ok(Number::Seven),
            "8" | "eight" => Ok(Number::Eight),
            "9" | "nine" => Ok(Number::Nine),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(day1_part1(input), 142);
    }

    #[test]
    fn part1() {
        let file_path = "input";
        let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");

        let answer = day1_part1(&content);
        print!("Answer: {}", answer)
    }

    #[test]
    fn part2_example() {
        let input = 
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(day1_part2(input), 281);
    }

    #[test]
    fn part2() {
        let file_path = "input";
        let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");

        let answer = day1_part2(&content);
        print!("Answer: {}", answer)
    }
}
