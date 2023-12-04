use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

type Int = usize;
type InputType = Vec<Int>;

fn count_matches_for_line(line: &str) -> Int {
    let (lhs, rhs): (&str, &str) = line
        .split_terminator(&[':', '|'][..])
        .skip(1)
        .collect_tuple()
        .unwrap();

    let lhs_set = lhs.split_whitespace().collect::<HashSet<&str>>();
    let rhs_set = rhs.split_whitespace().collect::<HashSet<&str>>();

    lhs_set.intersection(&rhs_set).count()
}

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day04/input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(count_matches_for_line)
        .collect()
}

fn part1(input: InputType) -> Int {
    input
        .iter()
        .map(|&matches| if matches >= 1 { 1 << (matches - 1) } else { 0 })
        .sum()
}

fn part2(input: InputType) -> Int {
    input
        .iter()
        .enumerate()
        .fold(vec![1; input.len()], |mut acc, (card, card_winnings)| {
            for i in 0..*card_winnings {
                acc[card + i + 1] += acc[card];
            }

            acc
        })
        .iter()
        .sum()
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

    println!("--- Day 04 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 28750);
    assert_eq!(part2, 10212704);
}
