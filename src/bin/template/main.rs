use std::fs;
use std::time::Instant;

type Int = u16;
type InputType = Vec<Int>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day00/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|s| s.parse::<Int>())
        .collect()
}

fn part1(_input: InputType) -> Int {
    0
}

fn part2(_input: InputType) -> Int {
    0
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

    println!("--- Day 00 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    // assert_eq!(part1, 0);
    // assert_eq!(part2, 0);
}
