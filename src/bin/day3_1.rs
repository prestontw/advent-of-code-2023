fn main() {
    //something else
    let input = include_str!("../../inputs/day3_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut line_index = 0;
    let mut total = 0;
    while line_index < grid.len() {
        let mut char_index = 0;
        let mut running_total = 0;
        let mut adjacent_symbol = false;
        while char_index < grid[line_index].len() {
            match grid[line_index][char_index] {
                c if c.is_numeric() => {
                    let num = c.to_digit(10).unwrap();
                    adjacent_symbol = adjacent_symbol || is_adjacent(line_index, char_index, &grid);
                    running_total = running_total * 10 + num;
                }
                _ => {
                    if adjacent_symbol {
                        total += dbg!(running_total);
                    }
                    running_total = 0;
                    adjacent_symbol = false;
                }
            }
            char_index += 1;
        }
        // Cleaning the remaining is a common pattern. I forgot it this year!
        if adjacent_symbol {
            total += running_total;
        }
        line_index += 1;
    }
    total as usize
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

#[test]
fn sample() {
    let input = r#"467..114..
...=......
..35..633.
......#...
617*......
.....%.58.
..592.....
......755.
...$./....
.664.598.."#;
    assert_eq!(run(input), 4361)
}
