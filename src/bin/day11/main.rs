use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::time::Instant;

type Int = i64;
type InputType = (BinaryHeap<Reverse<Int>>, BinaryHeap<Reverse<Int>>);

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day11/input.txt").unwrap();

    let mut rows = BinaryHeap::new();
    let mut cols = BinaryHeap::new();

    for (row, line) in file.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                rows.push(Reverse(row as Int));
                cols.push(Reverse(col as Int));
            }
        }
    }

    (rows, cols)
}

fn axis_sum(mut nums: BinaryHeap<Reverse<Int>>, factor: Int) -> Int {
    let mut res = 0;
    let mut sum = 0;

    // Numbers are never negative
    let mut prev = Int::MIN;
    let mut i = 0;
    let mut duplicate_nums = 0;

    while let Some(Reverse(mut num)) = nums.pop() {
        if prev == num {
            duplicate_nums += 1;
        }
        prev = num;

        // Account for universe expansion
        num += (num - i + duplicate_nums) * (factor - 1);

        res += num * i - sum;
        sum += num;
        i += 1;
    }

    res
}

fn sum_paths((rows, cols): InputType, factor: Int) -> Int {
    axis_sum(rows, factor) + axis_sum(cols, factor)
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
