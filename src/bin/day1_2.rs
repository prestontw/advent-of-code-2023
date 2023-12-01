fn main() {
    //something else
    let input = include_str!("../../inputs/day1_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> usize {
    let list = input.lines().map(|s| {
        let numbers = s;
        let first = first_num(&numbers);
        let last = last_num(&numbers);
        format!("{}{}", first, last).parse().unwrap()
    });
    let list = list.collect::<Vec<_>>();
    dbg!(&list);
    list.iter().sum()
}

fn first_num(s: &str) -> i32 {
    let needles: Vec<(&str, i32)> = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let founds = needles.into_iter().filter_map(|needle| {
        if let Some(position) = s.find(needle.0) {
            Some((needle.1, position))
        } else {
            None
        }
    });
    founds.min_by_key(|(value, pos)| *pos).unwrap().0
}

fn last_num(s: &str) -> i32 {
    let needles: Vec<(&str, i32)> = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let founds = needles
        .into_iter()
        .filter_map(|needle| {
            if let Some(position) = s.rfind(needle.0) {
                Some((needle.1, position))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    dbg!(s, &founds);
    founds
        .into_iter()
        .max_by_key(|(value, pos)| *pos)
        .unwrap()
        .0
}

#[test]
fn sample() {
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    assert_eq!(run(input), 281)
}
