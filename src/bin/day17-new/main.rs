use num::Complex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

type Int = isize;
type InputType = HashMap<Complex<Int>, Int>;
const N: Int = 141;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day17/input.txt").unwrap();

    let mut grid = HashMap::new();

    file.trim().lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            grid.insert(
                Complex::new(i as Int, j as Int),
                c.to_string().parse().unwrap(),
            );
        });
    });

    grid
}

fn find_ans(input: InputType, min_steps: Int, max_steps: Int) -> Int {
    let mut queue = BinaryHeap::from([(Reverse(0), 0, 0, 0, 0)]);
    let mut seen = HashSet::new();

    while let Some((Reverse(cost), x, y, px, py)) = queue.pop() {
        if x == N - 1 && y == N - 1 {
            return cost;
        }

        if seen.contains(&(x, y, px, py)) {
            continue;
        }

        seen.insert((x, y, px, py));

        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            if (dx, dy) == (px, py) || (dx, dy) == (-px, -py) {
                continue;
            }

            let mut xx = x;
            let mut yy = y;
            let mut new_cost = cost;

            for i in 1..=max_steps {
                xx += dx;
                yy += dy;

                if let Some(d_cost) = input.get(&Complex::new(xx, yy)) {
                    new_cost += d_cost;

                    if i >= min_steps {
                        queue.push((Reverse(new_cost), xx, yy, dx, dy))
                    }
                }
            }
        }
    }

    panic!("No solution found");
}

fn part1(input: InputType) -> Int {
    find_ans(input, 1, 3)
}

fn part2(input: InputType) -> Int {
    find_ans(input, 4, 10)
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

    println!("--- Day 17 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 855);
    assert_eq!(part2, 980);
}
