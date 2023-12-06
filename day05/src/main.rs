use day05::{day5_part1, day5_part2};

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", day5_part1(input).unwrap());
    println!("Part 2: {}", day5_part2(input).unwrap());
}
