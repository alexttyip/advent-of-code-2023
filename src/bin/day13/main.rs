use std::fs;
use std::time::Instant;

type Int = usize;
type InputType = Vec<(Vec<Int>, Vec<Int>)>;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day13/input.txt").unwrap();

    let mut patterns = Vec::new();

    for pattern_str in file.split("\n\n") {
        let mut rows = vec![];
        let mut cols = vec![];

        for line in pattern_str.lines() {
            let mut row = 0;
            for (i, c) in line.chars().enumerate() {
                let curr = Int::from(c == '#');

                row = (row << 1) + curr;

                if let Some(col) = cols.get_mut(i) {
                    *col = (*col << 2) + curr;
                } else {
                    cols.push(curr);
                }
            }

            rows.push(row);
        }

        patterns.push((rows, cols));
    }

    patterns
}

fn differs_by_single_bit(a: Int, b: Int) -> bool {
    let xor = a ^ b;

    xor > 0 && ((xor & (xor - 1)) == 0)
}

fn scan(v: &Vec<Int>, part1: bool) -> Option<Int> {
    let mut axis = 0;
    let mut delta = 1;
    let mut smudge_found = false;

    while axis < v.len() - 1 {
        let curr_i = axis + 1 - delta;
        let next_i = axis + delta;
        let curr = v[curr_i];
        let next = v[next_i];

        if curr == next {
            if curr_i == 0 || next_i == v.len() - 1 {
                if part1 || smudge_found {
                    return Some(axis + 1);
                }

                if !part1 {
                    axis += 1;
                    delta = 1;
                    smudge_found = false;
                    continue;
                }
            }

            delta += 1;
            continue;
        }

        if !part1 && !smudge_found && differs_by_single_bit(curr, next) {
            if curr_i == 0 || next_i == v.len() - 1 {
                return Some(axis + 1);
            }

            delta += 1;
            smudge_found = true;
            continue;
        }

        axis += 1;
        delta = 1;
        smudge_found = false;
    }

    None
}

fn part1(input: InputType) -> Int {
    let mut ans = 0;

    for (rows, cols) in input.iter() {
        if let Some(i) = scan(rows, true) {
            ans += i * 100;
            continue;
        }

        ans += scan(cols, true).unwrap();
    }

    ans
}

fn part2(input: InputType) -> Int {
    let mut ans = 0;

    for (rows, cols) in input.iter() {
        if let Some(i) = scan(rows, false) {
            ans += i * 100;
            continue;
        }

        ans += scan(cols, false).unwrap();
    }

    ans
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

    println!("--- Day 13 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 26957);
    assert_eq!(part2, 42695);
}
