use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = usize;
type InputType = Vec<String>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day15/input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.to_string())
        .collect_vec()
}

fn get_box_n(label: &str) -> Int {
    let mut curr = 0;

    for c in label.chars() {
        curr += c as Int;
        curr *= 17;
        curr %= 256;
    }

    curr
}

fn part1(input: InputType) -> Int {
    input.iter().fold(0, |acc, s| acc + get_box_n(s) as Int)
}

fn part2(input: InputType) -> Int {
    let mut boxes = vec![Vec::<(String, Int)>::new(); 256];

    for s in input {
        if let Some((label, fl)) = s.split_once('=') {
            let b = get_box_n(label);

            let label = label.to_string();
            let fl = fl.parse::<Int>().unwrap();

            if let Some((i, _)) = boxes[b].iter().find_position(|(ll, _)| ll == &label) {
                boxes[b][i] = (label, fl);
                continue;
            }

            boxes[b].push((label, fl));
            continue;
        }

        let label = &s[..s.len() - 1];
        let b = get_box_n(label);

        boxes[b].retain(|(ll, _)| ll != label);
    }

    boxes.iter().enumerate().fold(0, |sum, (i, bb)| {
        bb.iter().enumerate().fold(0, |acc, (j, (_, fl))| {
            acc + (i + 1) as Int * (j + 1) as Int * *fl
        }) + sum
    })
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

    println!("--- Day 15 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 511416);
    assert_eq!(part2, 290779);
}
