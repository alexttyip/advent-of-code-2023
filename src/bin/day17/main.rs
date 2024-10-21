use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

type Int = usize;
type InputType = [[Int; N]; N];
const N: Int = 141;
type CostType = HashMap<(Int, Int, isize, isize), Int>;
type QueueType = VecDeque<(Int, Int, isize, isize, Int)>;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day17/input.txt").unwrap();

    let mut grid = [[0; N]; N];

    file.trim().lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            grid[i][j] = c.to_string().parse().unwrap();
        })
    });

    grid
}

fn update_cost_and_append_to_queue(
    costs: &mut CostType,
    y: Int,
    x: Int,
    dy: isize,
    dx: isize,
    new_cost: Int,
    queue: &mut QueueType,
) {
    costs
        .entry((y, x, dy, dx))
        .and_modify(|prev_cost| {
            if new_cost < *prev_cost {
                queue.push_back((y, x, dy, dx, new_cost));
                *prev_cost = new_cost;
            }
        })
        .or_insert_with(|| {
            queue.push_back((y, x, dy, dx, new_cost));
            new_cost
        });
}

fn get_answer_from_costs(costs: CostType) -> Int {
    costs.iter().fold(Int::MAX, |acc, (&(y, x, _, _), c)| {
        if y == N - 1 && x == N - 1 {
            acc.min(*c)
        } else {
            acc
        }
    })
}

fn part1(input: InputType) -> Int {
    let mut costs = HashMap::<(Int, Int, isize, isize), Int>::new();
    costs.insert((0, 0, 0, 0), 0);

    let mut queue = VecDeque::<(Int, Int, isize, isize, Int)>::from([(0, 0, 0, 0, 0)]);

    while let Some((y, x, dy, dx, c)) = queue.pop_front() {
        if y > 0 && (-2..=0).contains(&dy) {
            let yy = y - 1;
            let new_dy = dy - 1;
            let new_cost = c + input[yy][x];

            update_cost_and_append_to_queue(&mut costs, yy, x, new_dy, 0, new_cost, &mut queue);
        }

        if y < N - 1 && (0..=2).contains(&dy) {
            let yy = y + 1;
            let new_dy = dy + 1;
            let new_cost = c + input[yy][x];

            update_cost_and_append_to_queue(&mut costs, yy, x, new_dy, 0, new_cost, &mut queue);
        }

        if x > 0 && (-2..=0).contains(&dx) {
            let xx = x - 1;
            let new_dx = dx - 1;
            let new_cost = c + input[y][xx];

            update_cost_and_append_to_queue(&mut costs, y, xx, 0, new_dx, new_cost, &mut queue);
        }

        if x < N - 1 && (0..=2).contains(&dx) {
            let xx = x + 1;
            let new_dx = dx + 1;
            let new_cost = c + input[y][xx];

            update_cost_and_append_to_queue(&mut costs, y, xx, 0, new_dx, new_cost, &mut queue);
        }
    }

    get_answer_from_costs(costs)
}

fn part2(input: InputType) -> Int {
    let mut costs = HashMap::<(Int, Int, isize, isize), Int>::new();
    costs.insert((0, 0, 0, 0), 0);

    let mut queue = VecDeque::<(Int, Int, isize, isize, Int)>::from([(0, 0, 0, 0, 0)]);

    while let Some((y, x, dy, dx, c)) = queue.pop_front() {
        if dy == 0 {
            if y > 3 {
                let yy = y - 4;
                let new_dy = -4;
                let new_cost = c + (1..=4).fold(0, |acc, i| acc + input[y - i][x]);

                update_cost_and_append_to_queue(&mut costs, yy, x, new_dy, 0, new_cost, &mut queue);
            }
        } else if (-9..0).contains(&dy) && y > 0 {
            let yy = y - 1;
            let new_dy = dy - 1;
            let new_cost = c + input[yy][x];

            update_cost_and_append_to_queue(&mut costs, yy, x, new_dy, 0, new_cost, &mut queue);
        }

        if dy == 0 {
            if y < N - 4 {
                let yy = y + 4;
                let new_dy = 4;
                let new_cost = c + (1..=4).fold(0, |acc, i| acc + input[y + i][x]);

                update_cost_and_append_to_queue(&mut costs, yy, x, new_dy, 0, new_cost, &mut queue);
            }
        } else if (0..10).contains(&dy) && y < N - 1 {
            let yy = y + 1;
            let new_dy = dy + 1;
            let new_cost = c + input[yy][x];

            update_cost_and_append_to_queue(&mut costs, yy, x, new_dy, 0, new_cost, &mut queue);
        }

        if dx == 0 {
            if x > 3 {
                let xx = x - 4;
                let new_dx = -4;
                let new_cost = c + (1..=4).fold(0, |acc, i| acc + input[y][x - i]);

                update_cost_and_append_to_queue(&mut costs, y, xx, 0, new_dx, new_cost, &mut queue);
            }
        } else if (-9..0).contains(&dx) && x > 0 {
            let xx = x - 1;
            let new_dx = dx - 1;
            let new_cost = c + input[y][xx];

            update_cost_and_append_to_queue(&mut costs, y, xx, 0, new_dx, new_cost, &mut queue);
        }

        if dx == 0 {
            if x < N - 4 {
                let xx = x + 4;
                let new_dx = 4;
                let new_cost = c + (1..=4).fold(0, |acc, i| acc + input[y][x + i]);

                update_cost_and_append_to_queue(&mut costs, y, xx, 0, new_dx, new_cost, &mut queue);
            }
        } else if (0..10).contains(&dx) && x < N - 1 {
            let xx = x + 1;
            let new_dx = dx + 1;
            let new_cost = c + input[y][xx];

            update_cost_and_append_to_queue(&mut costs, y, xx, 0, new_dx, new_cost, &mut queue);
        }
    }

    get_answer_from_costs(costs)
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

    println!("--- Day 17 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 855);
    assert_eq!(part2, 980);
}
