use std::collections::HashSet;
use std::fs;
use std::time::Instant;

type Int = usize;
type InputType = Vec<String>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day04/input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn count_matches_for_line(line: &str) -> Int {
    let mut set = HashSet::new();
    let mut matches = 0;
    let mut is_lhs = true;

    for num in line.split_whitespace().skip(2) {
        if num == "|" {
            is_lhs = false;
            continue;
        }

        if is_lhs {
            set.insert(num);
            continue;
        }

        if set.contains(&num) {
            matches += 1;
        }
    }

    matches
}

fn part1(input: InputType) -> Int {
    let mut sum = 0;

    for line in input {
        let matches = count_matches_for_line(&line);

        if matches >= 1 {
            sum += 1 << (matches - 1);
        }
    }

    sum
}

fn part2(input: InputType) -> Int {
    let mut count = vec![1; input.len()];

    for (card, line) in input.iter().enumerate() {
        let card_winnings = count_matches_for_line(line);

        for i in 1..=card_winnings {
            count[card + i] += count[card];
        }
    }

    count.iter().sum()
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
