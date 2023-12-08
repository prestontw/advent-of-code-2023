use std::{cmp::Ordering, collections::HashSet, str::FromStr};

use advent_2023::counts;

fn main() {
    //something else
    let input = include_str!("../../inputs/day7_1.txt");
    dbg!(run(input));
}

fn run(input: &str) -> usize {
    let mut list = input
        .lines()
        .map(|s| {
            let mut chunks = s.split_whitespace();
            let hand = chunks
                .next()
                .unwrap()
                .chars()
                .filter_map(|s| Value::from_str(&s.to_string()).ok())
                .collect::<Vec<Value>>();
            let hand = Hand::new(hand);
            let bid = chunks.next().unwrap().parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    list.sort_unstable_by_key(|(h, _)| h.clone());
    dbg!(&list);
    list.into_iter()
        .enumerate()
        .map(|(index, (_, bid))| (index + 1) * bid)
        .sum()
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Hand {
    hand_type: HandType,
    values: Vec<Value>,
}

impl Hand {
    fn new(hand: Vec<Value>) -> Self {
        let counts = counts(counts(&hand).values().copied());
        let counts_counts = counts;
        let hand_type = if counts_counts == [(5, 1)].into() {
            HandType::Five
        } else if counts_counts == [(4, 1), (1, 1)].into() {
            HandType::Four
        } else if counts_counts == [(3, 1), (2, 1)].into() {
            HandType::FullHouse
        } else if counts_counts == [(3, 1), (1, 2)].into() {
            HandType::Three
        } else if counts_counts == [(2, 2), (1, 1)].into() {
            HandType::Two
        } else if counts_counts == [(2, 1), (1, 3)].into() {
            HandType::One
        } else {
            HandType::High
        };
        Self {
            hand_type,
            values: hand,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.values.iter().cmp(&other.values),
            x => x,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Clone, Eq, PartialOrd, Ord, Hash, Debug)]
enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Value::*;
        match s {
            "2" => Ok(Two),
            "3" => Ok(Three),
            "4" => Ok(Four),
            "5" => Ok(Five),
            "6" => Ok(Six),
            "7" => Ok(Seven),
            "8" => Ok(Eight),
            "9" => Ok(Nine),
            "T" => Ok(Ten),
            "J" => Ok(Jack),
            "Q" => Ok(Queen),
            "K" => Ok(King),
            "A" => Ok(Ace),
            c => unreachable!("weird card: {}", c),
        }
    }
}

#[derive(PartialEq, Clone, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    High,
    One,
    Two,
    Three,
    FullHouse,
    Four,
    Five,
}

#[test]
fn sample() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert!(HandType::Five > HandType::Four);
    assert_eq!(run(input), 6440)
}
