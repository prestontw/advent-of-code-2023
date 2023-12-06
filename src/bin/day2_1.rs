use std::sync::OnceLock;

use advent_2023::extract_values;
use regex::Regex;

fn main() {
    //something else
    let input = include_str!("../../inputs/day2_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> usize {
    let list = input.lines().map(parse_line);
    list.filter_map(|game| if game.is_valid() { Some(game.id) } else { None })
        .sum()
}

fn line_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"Game (\d+): (.*)").unwrap())
}

struct Game {
    id: usize,

    shown: Vec<Roll>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.shown.iter().all(|r| r.is_valid())
    }
}

impl Roll {
    fn is_valid(&self) -> bool {
        self.red < 13 && self.green < 14 && self.blue < 15
    }
}

struct Roll {
    green: usize,
    red: usize,
    blue: usize,
}

fn parse_line(input: &str) -> Game {
    let capture_groups = extract_values(line_regex(), input);
    let id = capture_groups[0].parse::<usize>().unwrap();
    let rolls = capture_groups[1].split(';').map(|show| {
        show.split(", ").fold(
            Roll {
                green: 0,
                red: 0,
                blue: 0,
            },
            |roll, description| {
                let mut pieces = description.split_whitespace();
                let count = pieces.next().unwrap().parse().unwrap();
                let color = pieces.next().unwrap();
                match color {
                    "green" => Roll {
                        green: count,
                        ..roll
                    },
                    "red" => Roll { red: count, ..roll },
                    "blue" => Roll {
                        blue: count,
                        ..roll
                    },
                    _ => roll,
                }
            },
        )
    });
    Game {
        id,
        shown: rolls.collect(),
    }
}

#[test]
fn sample() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(run(input), 8)
}
