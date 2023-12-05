use std::{collections::HashSet, sync::OnceLock};

use advent_2023::extract_values;
use regex::Regex;

fn main() {
    //something else
    let input = include_str!("../../inputs/day4_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let groups = extract_values(line_regex(), line);
            let winning_numbers = groups[1].split_whitespace().collect::<HashSet<&str>>();
            let our_numbers = groups[2].split_whitespace();

            let num_winners = our_numbers
                .filter(|number| winning_numbers.contains(number))
                .count();
            if num_winners == 0 {
                0
            } else {
                2_i32.pow(num_winners as u32 - 1)
            }
        })
        .sum()
}

fn line_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"Card *(\d+): (.*) \| (.*)").unwrap())
}

#[test]
fn sample() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(run(input), 13)
}
