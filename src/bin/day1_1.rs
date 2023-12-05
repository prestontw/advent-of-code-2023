fn main() {
    //something else
    let input = include_str!("../../inputs/day1_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> usize {
    let list = input.lines().map(|s| {
        let numbers = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
        let first = numbers.chars().next().unwrap();
        let last = numbers.chars().last().unwrap();
        format!("{}{}", first, last).parse().unwrap()
    });
    let list = list.collect::<Vec<_>>();
    list.iter().sum()
}

#[test]
fn sample() {
    let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    assert_eq!(run(input), 142)
}
