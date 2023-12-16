use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = usize;
type InputType = [[char; N]; N];

const N: usize = 110;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day16/input.txt").unwrap();

    let mut grid = [['.'; N]; N];

    for (i, row) in file.lines().enumerate() {
        for (j, c) in row.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    grid
}

fn get_new_rc(row: Int, col: Int, direction: Int) -> Option<(Int, Int)> {
    match direction {
        0 => row.checked_sub(1).map(|r| (r, col)),
        1 if col + 1 < N => Some((row, col + 1)),
        2 if row + 1 < N => Some((row + 1, col)),
        3 => col.checked_sub(1).map(|c| (row, c)),
        _ => None,
    }
}

fn step(
    grid: &InputType,
    history: &mut HashSet<(Int, Int, Int)>,
    row: Int,
    col: Int,
    direction: Int,
) {
    if !history.insert((row, col, direction)) {
        return;
    }

    match grid[row][col] {
        '.' => {
            let Some((r, c)) = get_new_rc(row, col, direction) else {
                return;
            };

            step(grid, history, r, c, direction)
        }
        '\\' => {
            let d = 3 - direction;
            let Some((r, c)) = get_new_rc(row, col, d) else {
                return;
            };

            step(grid, history, r, c, d)
        }
        '/' => {
            let d = (5 - direction) % 4;
            let Some((r, c)) = get_new_rc(row, col, d) else {
                return;
            };

            step(grid, history, r, c, d)
        }
        '|' => {
            // Beam moving vertically
            if direction % 2 == 0 {
                let Some((r, c)) = get_new_rc(row, col, direction) else {
                    return;
                };

                return step(grid, history, r, c, direction);
            }

            for d in [0, 2] {
                let Some((r, c)) = get_new_rc(row, col, d) else {
                    continue;
                };

                step(grid, history, r, c, d);
            }
        }
        '-' => {
            // Beam moving horizontally
            if direction % 2 == 1 {
                let Some((r, c)) = get_new_rc(row, col, direction) else {
                    return;
                };

                return step(grid, history, r, c, direction);
            }

            for d in [1, 3] {
                let Some((r, c)) = get_new_rc(row, col, d) else {
                    continue;
                };

                step(grid, history, r, c, d);
            }
        }
        _ => panic!(),
    }
}

fn count_unique_squares(history: &HashSet<(Int, Int, Int)>) -> Int {
    history.iter().unique_by(|(r, c, _)| (r, c)).count()
}

fn part1(input: InputType) -> Int {
    let mut history = HashSet::new();

    step(&input, &mut history, 0, 0, 1);

    count_unique_squares(&history)
}

fn part2(input: InputType) -> Int {
    let mut combinations = Vec::with_capacity(N * 4);

    for r in 0..N {
        for c in 0..N {
            if r == 0 {
                combinations.push((r, c, 2));
            }

            if c == 0 {
                combinations.push((r, c, 1));
            }

            if r == N - 1 {
                combinations.push((r, c, 0));
            }

            if c == N - 1 {
                combinations.push((r, c, 3));
            }
        }
    }

    let mut history = HashSet::new();

    combinations
        .iter()
        .map(|(r, c, d)| {
            history.clear();
            step(&input, &mut history, *r, *c, *d);
            count_unique_squares(&history)
        })
        .max()
        .unwrap()
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let part1 = part1(input);
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 16 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 8389);
    assert_eq!(part2, 8564);
}
