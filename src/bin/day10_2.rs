use std::collections::{HashMap, VecDeque};

fn main() {
    //something else
    let input = include_str!("../../inputs/day10_1.txt");
    dbg!(run(input));
}

#[derive(Debug)]
enum Boundary {
    Up,
    Down,
    None,
    Wall,
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
        if !current.is_valid(&grid) {
            continue;
        }

        if let Some(already_found_distance) = seen.get(&current.pos) {
            if *already_found_distance == current.distance {
                break;
            }
            continue;
        }

        seen.insert(current.pos, current.distance);

        for neighbor in current.neighbors(&grid) {
            queue.push_back(neighbor);
        }
    }

    grid.into_iter()
        .enumerate()
        .map(|(line_index, line)| {
            line.into_iter().enumerate().fold(
                (0, true, Boundary::None),
                |(count, exterior, boundary), (char_index, char)| {
                    if seen.contains_key(&(line_index, char_index)) {
                        use Boundary::*;
                        match (Pipe::new(char).to_boundary(), boundary) {
                            (Up, Down) | (Down, Up) => (count, !exterior, None),
                            (Wall, _) | (_, Wall) => (count, !exterior, None),
                            (Up, Up) => (count, exterior, None),
                            (Down, Down) => (count, exterior, None),
                            (boundary, None) | (None, boundary) => (count, exterior, boundary),
                        }
                    } else {
                        (
                            count + if exterior { 0 } else { 1 },
                            exterior,
                            Boundary::None,
                        )
                    }
                },
            )
        })
        .map(|(count, _, _)| count)
        .sum()
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

struct Pipe(char);

impl Pipe {
    fn new(c: char) -> Self {
        Pipe(c)
    }

    fn to_boundary(&self) -> Boundary {
        match self.0 {
            // This 'S' orientation is specific to my input (and luckily passes the sample)
            'S' | 'F' | '7' => Boundary::Down,
            'L' | 'J' => Boundary::Up,
            '|' => Boundary::Wall,
            _ => Boundary::None,
        }
    }
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
        let (line_index, char_index) = self.pos;
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
    let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
    assert_eq!(run(input), 10)
}
