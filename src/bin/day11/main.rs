use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = i64;
type InputType = HashSet<Coord>;
type Coord = (Int, Int);

const N: Int = 140;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day11/input.txt").unwrap();

    let mut points = HashSet::new();

    for (row, line) in file.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                points.insert((row as Int, col as Int));
            }
        }
    }

    points
}

fn manhattan(&(r, c): &Coord, &(rr, cc): &Coord) -> Int {
    (r - rr).abs() + (c - cc).abs()
}

fn sum_paths(input: InputType, factor: Int) -> Int {
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];

    for i in 0..N {
        if input.iter().all(|(r, _)| r != &i) {
            empty_rows.push(i);
        }

        if input.iter().all(|(_, c)| c != &i) {
            empty_cols.push(i);
        }
    }

    input
        .iter()
        .map(|(r, c)| {
            (
                r + empty_rows.partition_point(|rr| rr < r) as Int * (factor - 1),
                c + empty_cols.partition_point(|cc| cc < c) as Int * (factor - 1),
            )
        })
        .combinations(2)
        .fold(0, |acc, comb| {
            acc + manhattan(comb.first().unwrap(), comb.last().unwrap())
        })
}

fn part1(input: InputType) -> Int {
    sum_paths(input, 2)
}

fn part2(input: InputType) -> Int {
    sum_paths(input, 1000000)
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

    println!("--- Day 11 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 10422930);
    assert_eq!(part2, 699909023130);
}
