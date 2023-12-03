use std::collections::VecDeque;
use std::fs;
use std::mem::swap;
use std::time::Instant;

type Int = u32;
type InputType = ([[Option<Int>; N]; N], Vec<(usize, usize, bool)>);

const N: usize = 140;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day03/input.txt").unwrap();

    let mut grid = [[None; N]; N];
    let mut symbols = Vec::new();

    for (y, line) in file.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                grid[y][x] = c.to_digit(10);
                continue;
            }

            if c != '.' {
                symbols.push((y, x, c == '*'));
            }
        }
    }

    (grid, symbols)
}

fn get_adjacent_numbers(
    grid: &[[Option<Int>; N]; N],
    (y, x): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut numbers = vec![];

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let Ok(xx) = usize::try_from(x as isize + dx) else {
                continue;
            };
            let Ok(yy) = usize::try_from(y as isize + dy) else {
                continue;
            };
            if xx >= N || yy >= N {
                continue;
            }

            if grid[yy][xx].is_some() {
                numbers.push((yy, xx));
            }
        }
    }

    numbers
}

fn part1((mut grid, symbols): InputType) -> Int {
    let mut queue = VecDeque::new();
    let mut numbers = [[None; N]; N];

    for (y, x, _) in symbols {
        queue.extend(get_adjacent_numbers(&grid, (y, x)))
    }

    while let Some((y, x)) = queue.pop_front() {
        if numbers[y][x].is_some() || grid[y][x].is_none() {
            continue;
        }

        swap(&mut grid[y][x], &mut numbers[y][x]);

        if x > 0 {
            queue.push_back((y, x - 1));
        }

        if x < N - 1 {
            queue.push_back((y, x + 1));
        }
    }

    let mut sum = 0;
    for row in numbers {
        let mut curr_num = 0;

        for cell in row {
            if let Some(n) = cell {
                curr_num *= 10;
                curr_num += n;
            } else {
                sum += curr_num;
                curr_num = 0;
            }
        }

        sum += curr_num;
    }

    sum
}

fn part2((mut grid, mut symbols): InputType) -> Int {
    let mut sum = 0;

    symbols.retain(|&(_, _, is_star)| is_star);

    for (y, x, _) in symbols {
        let mut queue = VecDeque::from(get_adjacent_numbers(&grid, (y, x)));

        if queue.len() <= 1 {
            continue;
        }

        let mut numbers = [[None; N]; 3];

        while let Some((yy, xx)) = queue.pop_front() {
            if numbers[yy + 1 - y][xx].is_some() || grid[yy][xx].is_none() {
                continue;
            }

            swap(&mut grid[yy][xx], &mut numbers[yy + 1 - y][xx]);

            if xx > 0 {
                queue.push_back((yy, xx - 1));
            }

            if xx < N - 1 {
                queue.push_back((yy, xx + 1));
            }
        }

        let mut product = 1;
        let mut num_count = 0;

        for row in numbers {
            let mut curr_num: Option<Int> = None;

            for cell in row.iter().chain([&None]) {
                if let Some(n) = cell {
                    curr_num = curr_num.map(|num| num * 10 + n).or(Some(*n));
                } else if let Some(n) = curr_num {
                    product *= n;
                    curr_num = None;
                    num_count += 1;
                }
            }
        }

        if num_count > 1 {
            sum += product;
        }
    }

    sum
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

    println!("--- Day 03 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 538046);
    assert_eq!(part2, 81709807);
}
