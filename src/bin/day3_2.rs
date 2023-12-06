use std::collections::{HashMap, HashSet};

fn main() {
    //something else
    let input = include_str!("../../inputs/day3_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut gear_positions = HashSet::new();
    let mut numbers = HashMap::new();
    for (line_index, line) in grid.iter().enumerate() {
        let mut running_total = 0;
        let mut adjacent_symbol = false;
        let mut starting_position = None;
        for (char_index, character) in line.iter().enumerate() {
            match character {
                c if c.is_numeric() => {
                    let num = c.to_digit(10).unwrap();
                    adjacent_symbol = adjacent_symbol || is_adjacent(line_index, char_index, &grid);
                    running_total = running_total * 10 + num;
                    if starting_position.is_none() {
                        starting_position = Some(char_index);
                    }
                }
                c => {
                    if let Some(left) = starting_position {
                        for char_pos in left..char_index {
                            numbers.insert(
                                (line_index, char_pos),
                                Number {
                                    value: running_total,
                                    starting_point: line_index * line.len() + left,
                                },
                            );
                        }
                    }
                    running_total = 0;
                    adjacent_symbol = false;
                    if *c == '*' {
                        gear_positions.insert((line_index, char_index));
                    }
                    starting_position = None;
                }
            }
        }
        // Cleaning the remaining is a common pattern. I forgot it this year!

        if let Some(left) = starting_position {
            for char_pos in left..(line.len()) {
                numbers.insert(
                    (line_index, char_pos),
                    Number {
                        value: running_total,
                        starting_point: line_index * line.len() + left,
                    },
                );
            }
        }
    }
    gear_positions
        .into_iter()
        .filter_map(|pos| {
            let numbers = adjacent_numbers(pos.0, pos.1, &numbers);
            if numbers.len() != 2 {
                None
            } else {
                Some(numbers[0].value * numbers[1].value)
            }
        })
        .sum()
}

#[derive(Clone)]
struct Number {
    value: u32,
    starting_point: usize,
}

fn is_adjacent(line_index: usize, char_index: usize, grid: &[Vec<char>]) -> bool {
    let line_index = line_index as isize;
    let char_index = char_index as isize;
    let positions = vec![
        (line_index - 1, char_index - 1),
        (line_index - 1, char_index),
        (line_index - 1, char_index + 1),
        (line_index, char_index - 1),
        (line_index, char_index + 1),
        (line_index + 1, char_index - 1),
        (line_index + 1, char_index),
        (line_index + 1, char_index + 1),
    ];
    positions.into_iter().any(|(line_index, char_index)| {
        if line_index < 0 || char_index < 0 {
            return false;
        }
        let line_index = line_index as usize;
        let char_index = char_index as usize;
        if let Some(c) = grid.get(line_index).and_then(|line| line.get(char_index)) {
            *c != '.' && !c.is_numeric()
        } else {
            false
        }
    })
}

fn adjacent_numbers(
    line_index: usize,
    char_index: usize,
    numbers: &HashMap<(usize, usize), Number>,
) -> Vec<Number> {
    let line_index = line_index as isize;
    let char_index = char_index as isize;
    let positions = vec![
        (line_index - 1, char_index - 1),
        (line_index - 1, char_index),
        (line_index - 1, char_index + 1),
        (line_index, char_index - 1),
        (line_index, char_index + 1),
        (line_index + 1, char_index - 1),
        (line_index + 1, char_index),
        (line_index + 1, char_index + 1),
    ];
    let mut nums = positions
        .into_iter()
        .filter_map(|(line_index, char_index)| {
            if line_index < 0 || char_index < 0 {
                return None;
            }
            let line_index = line_index as usize;
            let char_index = char_index as usize;
            numbers.get(&(line_index, char_index))
        })
        .collect::<Vec<_>>();
    nums.sort_unstable_by_key(|num| num.starting_point);
    nums.dedup_by_key(|num| num.starting_point);
    nums.into_iter().cloned().collect()
}

#[test]
fn sample() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(run(input), 467835)
}
