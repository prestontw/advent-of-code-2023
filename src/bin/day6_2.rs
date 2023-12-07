fn main() {
    //something else
    let input = include_str!("../../inputs/day6_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    num_ways(times, distances)
}

fn num_ways(time: usize, distance: u64) -> usize {
    // x * (time - x) - distance => -x^2 + time x - distance
    // => a = -1, b = time, c = -distance
    let a = -1.0;
    let b = time as f64;
    let c = -(distance as f64);

    let x1 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let x2 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let right = dbg!(x1.max(x2) - 1.0).ceil();
    let left = dbg!(x1.min(x2) + 1.0).floor();
    ((left as i64)..=(right as i64)).count()
}

#[test]
fn sample() {
    let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
    assert_eq!(run(input), 71503)
}
#[test]
fn test_num_ways() {
    assert_eq!(num_ways(30, 200), 9)
}
