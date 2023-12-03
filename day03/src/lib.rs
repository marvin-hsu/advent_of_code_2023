use regex::Regex;

pub fn day3_part1(input: &str) -> usize {
    let (numbers, symbols) = prepare_data(input);

    numbers
        .iter()
        .filter(|number| number.is_adjacent_to_symbol(&symbols))
        .map(|number| number.value)
        .sum()
}

pub fn day3_part2(input: &str) -> usize {
    let (numbers, symbols) = prepare_data(input);

    symbols
        .into_iter()
        .filter(|symbol| symbol.0.eq(&'*'))
        .filter_map(|symbol| match symbol.get_neighbours(&numbers).as_slice() {
            [first, second] => Some(first.value * second.value),
            _ => None,
        })
        .sum()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Symbol(char, usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Number {
    value: usize,
    x: usize,
    y: usize,
    length: usize,
}

impl Number {
    fn is_adjacent_to_symbol(self, symbols: &[Symbol]) -> bool {
        let x_start = self.x.saturating_add_signed(-1);
        let x_end = self.x + self.length;
        let y_start = self.y.saturating_add_signed(-1);
        let y_end = self.y + 1;

        symbols
            .iter()
            .any(|Symbol(_, x, y)| *x >= x_start && *x <= x_end && *y >= y_start && *y <= y_end)
    }
}

impl Symbol {
    fn get_neighbours(self, numbers: &[Number]) -> Vec<&Number> {
        numbers
            .iter()
            .filter(|Number { x, y, length, .. }| {
                self.1 >= x.saturating_add_signed(-1)
                    && self.1 <= x + length
                    && self.2 >= y.saturating_add_signed(-1)
                    && self.2 <= y + 1
            })
            .collect()
    }
}

fn prepare_data(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let re = Regex::new(r"^\d+").unwrap();

    input
        .lines()
        .enumerate()
        .fold((vec![], vec![]), |(mut numbers, mut symbols), (y, line)| {
            let mut x = 0;

            while x < line.len() {
                if let Some(m) = re.find(&line[x..]) {
                    numbers.push(Number {
                        value: m.as_str().parse().unwrap(),
                        x,
                        y,
                        length: m.len(),
                    });

                    x += m.len();
                } else if let Some(c) = line[x..].chars().next().filter(|c| !c.eq(&'.')) {
                    symbols.push(Symbol(c, x, y));

                    x += 1;
                } else {
                    x += 1;
                }
            }

            (numbers, symbols)
        })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(day3_part1(input), 4361);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("input").unwrap();

        assert_eq!(day3_part1(&input), 520135);
    }

    #[test]
    fn part2_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(day3_part2(input), 467835);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("input").unwrap();

        assert_eq!(day3_part2(&input), 72514855);
    }

    #[test]
    fn test_prepare_data() {
        let input = "467..114..
...*......";
        let (numbers, symbols) = prepare_data(input);

        assert_eq!(
            numbers,
            vec![
                Number {
                    value: 467,
                    x: 0,
                    y: 0,
                    length: 3
                },
                Number {
                    value: 114,
                    x: 5,
                    y: 0,
                    length: 3
                }
            ]
        );

        assert_eq!(symbols, vec![Symbol('*', 3, 1)]);
    }
}
