use itertools::Itertools;
use std::fs;
use std::time::Instant;

type Int = u64;
type InputType = (String, String);

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day06/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_terminator(':').nth(1).unwrap().trim().to_string())
        .collect_tuple()
        .unwrap()
}

fn count_ways(time: Int, distance: Int) -> Int {
    (1..time).fold(0, |ways_count, hold| {
        let travel = (time - hold) * hold;

        if travel > distance {
            ways_count + 1
        } else {
            ways_count
        }
    })
}

fn part1((times_str, distances_str): InputType) -> Int {
    let mut ans = 1;

    for (time, distance) in times_str
        .split_whitespace()
        .zip(distances_str.split_whitespace())
    {
        let time = time.parse().unwrap();
        let distance = distance.parse().unwrap();

        ans *= count_ways(time, distance);
    }

    ans
}

fn part2((times_str, distances_str): InputType) -> Int {
    let time = times_str.replace(' ', "").parse().unwrap();
    let distance = distances_str.replace(' ', "").parse().unwrap();

    count_ways(time, distance)
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

    println!("--- Day 06 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 588588);
    assert_eq!(part2, 34655848);
}
