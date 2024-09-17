fn main() {
    //something else
    let input = include_str!("../../inputs/day1_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> i32 {
    let list = input.lines().map(|s| {
        let numbers = s;
        let first = first_num(numbers);
        let last = last_num(numbers);
        first * 10 + last
    });
    let list = list.collect::<Vec<_>>();
    list.into_iter().sum()
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
    let founds = needles
        .into_iter()
        .filter_map(|needle| s.find(needle.0).map(|position| (needle.1, position)));
    founds.min_by_key(|(_, pos)| *pos).unwrap().0
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
        .filter_map(|needle| s.rfind(needle.0).map(|position| (needle.1, position)))
        .collect::<Vec<_>>();
    founds.into_iter().max_by_key(|(_, pos)| *pos).unwrap().0
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
