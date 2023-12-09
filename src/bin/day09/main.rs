use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = i32;
type InputType = Vec<Vec<Int>>;

const N: usize = 21;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day09/input.txt").unwrap();

    file.trim()
        .lines()
        .map(|s| {
            s.split_whitespace()
                .filter_map(|s| s.parse::<Int>().ok())
                .collect_vec()
        })
        .collect_vec()
}

fn extrapolate(input: InputType) -> (Int, Int) {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut grid = [[0; N + 2]; N + 2];

    for line in input {
        grid[0][1..N + 1].copy_from_slice(&line);

        for row in 1..N + 1 {
            for col in 1..(N + 1 - row) {
                grid[row][col] = grid[row - 1][col + 1] - grid[row - 1][col];
            }
        }

        for row in (0..N).rev() {
            // Part 1
            grid[row][N - row + 1] = grid[row][N - row] + grid[row + 1][N - row];

            // Part 2
            grid[row][0] = grid[row][1] - grid[row + 1][0];
        }

        part1 += grid[0].last().unwrap();
        part2 += grid[0][0];
    }

    (part1, part2)
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let (part1, part2) = extrapolate(input.clone());
    let part1_elapsed = now.elapsed();

    println!("--- Day 09 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Both parts combined took: {:.2?}", part1_elapsed);

    assert_eq!(part1, 1974913025);
    assert_eq!(part2, 884);
}
