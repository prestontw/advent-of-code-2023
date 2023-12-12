use std::collections::HashSet;

use advent_2023::{manhattan_distance, manhattan_distance2d};

fn main() {
    //something else
    let input = include_str!("../../inputs/day11_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> i64 {
    let expanded = input;
    let mut galaxies = expanded
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(char_index, char)| {
                    (char == '#').then_some((line_index as i64, char_index as i64))
                })
        })
        .collect::<Vec<_>>();

    // expand at this moment
    let empty_rows = input
        .lines()
        .enumerate()
        .filter_map(|(line_index, line)| line.chars().all(|c| c == '.').then_some(line_index))
        .collect::<Vec<_>>();
    let initial_width = input.lines().next().unwrap().len();
    let empty_columns = (0..initial_width)
        .filter(|potential_column| {
            !galaxies
                .iter()
                .any(|(_, char_index)| *char_index == *potential_column as i64)
        })
        .collect::<Vec<_>>();

    galaxies.iter_mut().for_each(|(line_index, char_index)| {
        *line_index += empty_rows
            .iter()
            .filter(|&&row_index| row_index < *line_index as usize)
            .count() as i64;
        *char_index += empty_columns
            .iter()
            .filter(|&&column_index| column_index < *char_index as usize)
            .count() as i64;
    });

    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            sum += manhattan_distance2d(*galaxy, *other_galaxy);
        }
    }
    sum
}

#[test]
fn sample() {
    let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    assert_eq!(run(input), 374)
}
