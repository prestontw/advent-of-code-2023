use std::collections::{HashMap, VecDeque};

fn main() {
    //something else
    let input = include_str!("../../inputs/day10_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> usize {
    let mut starting_pos = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .map(|(char_index, char)| {
                    if char == 'S' {
                        starting_pos = (line_index, char_index);
                    }
                    char
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut seen = HashMap::new();
    seen.insert(starting_pos, 0);
    let starting_pos = Pos {
        pos: starting_pos,
        distance: 0,
        came_from: Dir::West,
    };
    let mut queue = starting_pos
        .neighbors(&grid)
        .into_iter()
        .filter(|p| p.is_valid(&grid))
        .collect::<VecDeque<_>>();

    while let Some(current) = queue.pop_front() {
        dbg!(&current);
        if !current.is_valid(&grid) {
            continue;
        }

        if let Some(already_found_distance) = seen.get(&current.pos) {
            if *already_found_distance == current.distance {
                return current.distance;
            }
            continue;
        }

        seen.insert(current.pos, current.distance);

        for neighbor in current.neighbors(&grid) {
            queue.push_back(neighbor);
        }
    }
    3
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Pos {
    pos: (usize, usize),
    came_from: Dir,
    distance: usize,
}

impl Pos {
    fn is_valid(&self, grid: &[Vec<char>]) -> bool {
        use Dir::*;
        grid.get(self.pos.0)
            .and_then(|line| line.get(self.pos.1))
            .is_some_and(|c| match (c, self.came_from) {
                ('.', _) => false,
                ('|', North | South) => true,
                ('-', East | West) => true,
                ('L', North | East) => true,
                ('J', North | West) => true,
                ('7', South | West) => true,
                ('F', South | East) => true,
                _ => false,
            })
    }

    fn neighbors(&self, grid: &[Vec<char>]) -> Vec<Self> {
        use Dir::*;
        let next = self.distance + 1;
        let (line_index, char_index) = dbg!(self.pos);
        let down = Pos {
            pos: (line_index + 1, char_index),
            came_from: North,
            distance: next,
        };
        let right = Pos {
            pos: (line_index, char_index + 1),
            came_from: West,
            distance: next,
        };

        if grid[line_index][char_index] == 'S' {
            let mut ret = vec![down, right];
            if line_index > 0 {
                ret.push(Pos {
                    pos: (line_index - 1, char_index),
                    distance: next,
                    came_from: South,
                });
            }
            if char_index > 0 {
                ret.push(Pos {
                    pos: (line_index, char_index - 1),
                    distance: next,
                    came_from: East,
                });
            }
            return ret;
        }

        let mut up = None;
        if line_index > 0 {
            up = Some(Pos {
                pos: (line_index - 1, char_index),
                distance: next,
                came_from: South,
            })
        };

        let mut left = None;
        if char_index > 0 {
            left = Some(Pos {
                pos: (line_index, char_index - 1),
                distance: next,
                came_from: East,
            })
        };

        match (grid[line_index][char_index], self.came_from) {
            ('|', North) => vec![down],
            ('|', South) => vec![up.unwrap()],
            ('-', West) => vec![right],
            ('-', East) => vec![left.unwrap()],
            ('L', North) => vec![right],
            ('L', East) => vec![up.unwrap()],
            ('J', West) => vec![up.unwrap()],
            ('J', North) => vec![left.unwrap()],
            ('7', West) => vec![down],
            ('7', South) => vec![left.unwrap()],
            ('F', South) => vec![right],
            ('F', East) => vec![down],
            _ => vec![],
        }
    }
}

#[test]
fn sample() {
    let input = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;
    assert_eq!(run(input), 8)
}
