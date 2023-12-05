use std::sync::OnceLock;

use advent_2023::{blank_lines, extract_values};
use regex::Regex;

fn main() {
    //something else
    let input = include_str!("../../inputs/day5_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> usize {
    let chunks = blank_lines(input);
    let (seeds, maps) = chunks.split_at(1);
    let seeds = seeds[0][0]
        .split_whitespace()
        .skip(1)
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let maps = maps.iter().map(|block| Map::new(block)).collect::<Vec<_>>();
    seeds
        .into_iter()
        .map(|seed_val| translate(seed_val, &maps))
        .min()
        .unwrap()
}

fn translate(seed_val: usize, maps: &[Map]) -> usize {
    let mut item_type = "seed";
    let mut value = seed_val;

    while item_type != "location" {
        let current_map = maps.iter().find(|map| map.from == item_type).unwrap();
        item_type = &current_map.to;
        value = current_map.translate(value);
    }
    value
}

fn map_line() -> &'static regex::Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"(.*)-to-(.*) map:").unwrap())
}

struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Map {
    fn new(block: &[&str]) -> Self {
        let (header, body) = block.split_at(1);
        let header = extract_values(map_line(), header[0]);
        let from = header[0].to_owned();
        let to = header[1].to_owned();

        Self {
            from,
            to,
            ranges: body.iter().map(|line| Range::new(line)).collect(),
        }
    }
    fn translate(&self, source_number: usize) -> usize {
        self.ranges
            .iter()
            .filter_map(|range| range.translate(source_number))
            .next()
            .unwrap_or(source_number)
    }
}

struct Range {
    source_start: usize,
    dest_start: usize,
    range_length: usize,
}

impl Range {
    fn new(line: &str) -> Self {
        let mut info = line.split_whitespace().filter_map(|num| num.parse().ok());
        let dest_start = info.next().unwrap();
        let source_start = info.next().unwrap();
        let range_length = info.next().unwrap();

        Self {
            source_start,
            dest_start,
            range_length,
        }
    }

    fn translate(&self, source_number: usize) -> Option<usize> {
        if source_number >= self.source_start
            && self.source_start + self.range_length - 1 >= source_number
        {
            Some(source_number - self.source_start + self.dest_start)
        } else {
            None
        }
    }
}

#[test]
fn sample() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(run(input), 35)
}
