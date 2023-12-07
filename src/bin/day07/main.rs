use std::cmp::Ordering;
use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = u64;
type InputType = Vec<(String, Int)>;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day07/input.txt").unwrap();

    file.trim()
        .lines()
        .filter_map(|s| {
            let (hand, bid) = s.split_once(' ')?;

            Some((hand.to_string(), bid.parse().ok()?))
        })
        .collect_vec()
}

fn get_hand_strength(cards: [u32; 13], part1: bool) -> usize {
    let max_count_rest = cards[..if part1 { 13 } else { 12 }].iter().max().unwrap();
    let joker_count = if part1 { 0 } else { cards[12] };
    let max_count = max_count_rest + joker_count;

    if max_count == 5 {
        return 0;
    }

    if max_count == 4 {
        return 1;
    }

    let twos = cards.iter().filter(|x| x == &&2).count();

    if max_count == 3 {
        if joker_count == 0 && twos > 0 {
            return 2;
        }

        if joker_count == 1 && twos == 2 {
            return 2;
        }

        return 3;
    }

    if max_count == 2 {
        return if twos == 2 { 4 } else { 5 };
    }

    6
}

const CARDS_P1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARDS_P2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn cmp_hands(a: &str, b: &str, part1: bool) -> Ordering {
    let mut a_cards = [0; 13];
    let mut b_cards = [0; 13];

    let mut second_ordering = None;
    for (ac, bc) in a.chars().zip(b.chars()) {
        let (ai, _) = if part1 { CARDS_P1 } else { CARDS_P2 }
            .iter()
            .find_position(|x| **x == ac)
            .unwrap();
        let (bi, _) = if part1 { CARDS_P1 } else { CARDS_P2 }
            .iter()
            .find_position(|x| **x == bc)
            .unwrap();

        a_cards[ai] += 1;
        b_cards[bi] += 1;

        let char_ordering = ai.cmp(&bi);
        if second_ordering.is_none() && char_ordering != Ordering::Equal {
            second_ordering = Some(char_ordering);
        }
    }

    match get_hand_strength(a_cards, part1).cmp(&get_hand_strength(b_cards, part1)) {
        Ordering::Equal => second_ordering.unwrap(),
        other => other,
    }
}

fn part1(input: InputType) -> Int {
    input
        .iter()
        .sorted_unstable_by(|(a, _), (b, _)| cmp_hands(b, a, true))
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as Int + 1) * bid)
}

fn part2(input: InputType) -> Int {
    input
        .iter()
        .sorted_unstable_by(|(a, _), (b, _)| cmp_hands(b, a, false))
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as Int + 1) * bid)
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 07 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 251927063);
    assert_eq!(part2, 255632664);
}
