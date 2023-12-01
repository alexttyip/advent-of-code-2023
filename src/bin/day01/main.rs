use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = u32;
type InputType = Vec<String>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day01/input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect_vec()
}

fn part1(input: InputType) -> Int {
    input.iter().fold(0, |acc, curr| {
        let mut digits = curr.chars().filter_map(|c| c.to_digit(10));

        let first = digits.next().unwrap();
        let last = digits.next_back().unwrap_or(first);

        acc + first * 10 + last
    })
}

const SPELT_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part2(input: InputType) -> Int {
    input.iter().fold(0, |sum, line| {
        let mut first_value = None;
        let mut last_value = None;

        'outer: for i in 0..line.len() {
            let left = &line[i..];
            let right = &line[(line.len() - i - 1)..];

            if first_value.is_none() && left.starts_with(|c: char| c.is_numeric()) {
                first_value = left.chars().next().and_then(|c| c.to_digit(10));
            }

            if last_value.is_none() && right.starts_with(|c: char| c.is_numeric()) {
                last_value = right.chars().next().and_then(|c| c.to_digit(10));
            }

            for (k, spelt) in SPELT_DIGITS.iter().enumerate() {
                if first_value.is_some() && last_value.is_some() {
                    break 'outer;
                }

                if first_value.is_none() && left.starts_with(spelt) {
                    first_value = Some(k as Int + 1);
                }

                if last_value.is_none() && right.starts_with(spelt) {
                    last_value = Some(k as Int + 1);
                }
            }
        }

        sum + first_value.unwrap() * 10 + last_value.unwrap()
    })
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

    println!("--- Day 01 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 55002);
    assert_eq!(part2, 55093);
}
