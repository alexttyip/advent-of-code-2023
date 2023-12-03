use itertools::Itertools;
use std::fs;
use std::time::Instant;

type Int = u32;

// Order: RGB
type InputType = Vec<Vec<(Int, Int, Int)>>;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day02/input.txt").unwrap();

    let mut games: InputType = Vec::with_capacity(100);

    for line in file.trim().lines() {
        let mut game = Vec::with_capacity(10);
        let mut set = (0, 0, 0);

        for mut chunk in &line
            .split(&[' ', ':', ','][..])
            .filter(|s| !s.is_empty())
            .skip(2)
            .chunks(2)
        {
            let n = chunk.next().and_then(|s| s.parse::<Int>().ok()).unwrap();
            let color = chunk.next().unwrap();

            if color.starts_with("red") {
                set.0 = n;
            } else if color.starts_with("green") {
                set.1 = n;
            } else if color.starts_with("blue") {
                set.2 = n;
            } else {
                panic!()
            }

            if color.ends_with(';') {
                game.push(set);
                set = (0, 0, 0);
            }
        }

        game.push(set);
        games.push(game);
    }

    games
}

const R: Int = 12;
const G: Int = 13;
const B: Int = 14;

fn part1(input: InputType) -> Int {
    input
        .iter()
        .enumerate()
        .map(|(i, game)| {
            if game.iter().all(|&(r, g, b)| r <= R && g <= G && b <= B) {
                i as Int + 1
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: InputType) -> Int {
    input
        .iter()
        .map(|game| {
            let (r, g, b) = game.iter().fold((0, 0, 0), |(r, g, b), &(rr, gg, bb)| {
                (r.max(rr), g.max(gg), b.max(bb))
            });

            r * g * b
        })
        .sum()
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

    println!("--- Day 02 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 2256);
    assert_eq!(part2, 74229);
}
