use std::fs;
use std::time::Instant;

use itertools::Itertools;

use crate::Rock::{Cube, Round};

type Int = usize;
type InputType = [[Option<Rock>; N]; N];

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Rock {
    Round,
    Cube,
}

const N: usize = 100;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day14/input.txt").unwrap();

    let mut rocks = [[None; N]; N];

    for (row, line) in file.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'O' => rocks[row][col] = Some(Round),
                '#' => rocks[row][col] = Some(Cube),
                _ => {}
            }
        }
    }

    rocks
}

fn north(input: &mut InputType) -> Int {
    let mut ans = 0;

    for c in 0..N {
        let mut weight = 0;
        let mut last_space = 0;

        for r in 0..N {
            let rock = input[r][c];

            match rock {
                Some(Round) => {
                    if last_space < r {
                        input[last_space][c] = Some(Round);
                        input[r][c] = None;

                        weight += N - last_space;

                        last_space += 1;
                        continue;
                    }

                    last_space += 1;
                    weight += N - r;
                }
                Some(Cube) => {
                    if last_space <= r {
                        last_space = r + 1;
                    }
                }
                None => {}
            }
        }

        ans += weight;
    }

    ans
}

fn south(input: &mut InputType) {
    for c in 0..N {
        let mut last_space = N - 1;

        for r in (0..N).rev() {
            let rock = input[r][c];

            match rock {
                Some(Round) => {
                    if last_space > r {
                        input[last_space][c] = Some(Round);
                        input[r][c] = None;

                        last_space -= 1;
                        continue;
                    }

                    last_space = last_space.saturating_sub(1);
                }
                Some(Cube) => {
                    if last_space >= r {
                        last_space = r.saturating_sub(1);
                    }
                }
                None => {}
            }
        }
    }
}

fn east(input: &mut InputType) -> Int {
    let mut ans = 0;

    for (r, row) in input.iter_mut().enumerate() {
        let mut weight = 0;
        let mut last_space = N - 1;

        for c in (0..N).rev() {
            let rock = row[c];

            match rock {
                Some(Round) => {
                    weight += N - r;

                    if last_space > c {
                        row[last_space] = Some(Round);
                        row[c] = None;

                        last_space -= 1;
                        continue;
                    }

                    last_space = last_space.saturating_sub(1);
                }
                Some(Cube) => {
                    if last_space >= c {
                        last_space = c.saturating_sub(1);
                    }
                }
                None => {}
            }
        }

        ans += weight;
    }

    ans
}

fn west(input: &mut InputType) {
    for row in input {
        let mut last_space = 0;

        for c in 0..N {
            let rock = row[c];

            match rock {
                Some(Round) => {
                    if last_space < c {
                        row[last_space] = Some(Round);
                        row[c] = None;

                        last_space += 1;
                        continue;
                    }

                    last_space += 1;
                }
                Some(Cube) => {
                    if last_space <= c {
                        last_space = c + 1;
                    }
                }
                None => {}
            }
        }
    }
}

fn part1(mut input: InputType) -> Int {
    north(&mut input)
}

fn part2(mut input: InputType) -> Int {
    let mut memo: Vec<(InputType, InputType, Int)> = Vec::with_capacity(500);
    let mut first_repeat_idx = usize::MAX;

    loop {
        if let Some((a, (_, next, _))) = memo.iter().find_position(|(p, _, _)| p == &input) {
            if a == first_repeat_idx {
                break;
            }

            if a < first_repeat_idx {
                first_repeat_idx = a;
            }

            input = *next;
            continue;
        }

        let prev = input;

        north(&mut input);
        west(&mut input);
        south(&mut input);
        let load = east(&mut input);

        memo.push((prev, input, load));
    }

    // (i - where i starts looping) % period + where a starts looping
    let state = memo
        .get((1000000000 - memo.len() - 1) % (memo.len() - first_repeat_idx) + first_repeat_idx)
        .unwrap();

    state.2
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

    println!("--- Day 14 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 105249);
    assert_eq!(part2, 88680);
}
