use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use num::Integer;

type Int = u64;
type InputType = (String, HashMap<String, (String, String)>);

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day08/input.txt").unwrap();

    let (instructions, nodes) = file.split_once("\n\n").unwrap();

    let mut map = HashMap::new();

    for node in nodes.lines() {
        let element = &node[0..3];
        let left = &node[7..10];
        let right = &node[12..15];

        map.insert(element.to_string(), (left.to_string(), right.to_string()));
    }

    (instructions.to_string(), map)
}

fn part1((instructions, nodes): InputType) -> Int {
    let mut steps = 0;

    let mut curr = "AAA";

    for instruction in instructions.chars().cycle() {
        let (left, right) = nodes.get(curr).unwrap();

        curr = if instruction == 'L' { left } else { right };

        steps += 1;

        if curr == "ZZZ" {
            break;
        }
    }

    steps
}

fn part2((instructions, nodes): InputType) -> Int {
    let mut steps_lcm = 1;

    let a_nodes = nodes.iter().filter_map(|(element, _)| {
        if element.ends_with('A') {
            Some(element)
        } else {
            None
        }
    });

    for mut curr in a_nodes {
        let mut steps = 0;

        for instruction in instructions.chars().cycle() {
            let (left, right) = nodes.get(curr).unwrap();
            curr = if instruction == 'L' { left } else { right };

            steps += 1;

            if curr.ends_with('Z') {
                steps_lcm = steps_lcm.lcm(&steps);
                break;
            }
        }
    }

    steps_lcm
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

    println!("--- Day 08 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 17873);
    assert_eq!(part2, 15746133679061);
}
