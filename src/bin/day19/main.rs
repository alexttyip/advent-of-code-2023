use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = u64;
type Workflows = HashMap<String, Vec<Rule>>;
type Part = [Int; 4];
type InputType = (Workflows, Vec<Part>);

struct Rule {
    category: usize,
    operator: char,
    value: Int,
    destination: String,
}

impl Rule {
    fn otherwise(destination: String) -> Rule {
        Rule {
            category: 0,
            operator: '<',
            value: Int::MAX,
            destination,
        }
    }
}

fn category_to_idx(category: &char) -> usize {
    match category {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!(),
    }
}

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day19/input.txt").unwrap();

    let (workflows_str, parts_str) = file.split_once("\n\n").unwrap();

    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    for line in workflows_str.lines() {
        let (name, rules_str) = line.split_once('{').unwrap();

        let rules = rules_str[..rules_str.len() - 1]
            .split(',')
            .filter_map(|rule_str| {
                if let Some((condition, destination)) = rule_str.split_once(':') {
                    let mut condition = condition.chars();

                    Some(Rule {
                        category: category_to_idx(&condition.next()?),
                        operator: condition.next()?,
                        value: condition.join("").parse::<Int>().ok()?,
                        destination: destination.to_string(),
                    })
                } else {
                    Some(Rule::otherwise(rule_str.to_string()))
                }
            })
            .collect_vec();

        workflows.insert(name.to_string(), rules);
    }

    for line in parts_str.lines() {
        let mut numbers = [0; 4];

        for (i, s) in line[1..line.len() - 1].split(',').enumerate() {
            numbers[i] = s[2..].parse::<Int>().unwrap();
        }

        parts.push(numbers);
    }

    (workflows, parts)
}

fn part1((workflows, parts): &InputType) -> Int {
    let mut ans = 0;
    for part in parts {
        let mut curr_workflow = "in".to_string();

        while &curr_workflow != "A" && &curr_workflow != "R" {
            for Rule {
                category,
                operator,
                value,
                destination,
            } in workflows.get(&curr_workflow).unwrap()
            {
                if (*operator == '<' && part[*category] < *value)
                    || (*operator == '>' && part[*category] > *value)
                {
                    curr_workflow = destination.clone();
                    break;
                }
            }
        }

        if curr_workflow == "A" {
            ans += part.iter().sum::<Int>();
        }
    }

    ans
}

fn part2((workflows, _): InputType) -> Int {
    let mut ans = 0;

    let mut queue = VecDeque::from([("in".to_string(), [1..4001, 1..4001, 1..4001, 1..4001])]);

    while let Some((workflow, mut ranges)) = queue.pop_front() {
        if &workflow == "A" {
            ans += ranges.iter().map(|r| r.end - r.start).product::<Int>();
            continue;
        }

        if &workflow == "R" {
            continue;
        }

        for Rule {
            category,
            operator,
            value,
            destination,
        } in workflows.get(&workflow).unwrap()
        {
            let curr_range = ranges[*category].clone();

            if curr_range.contains(value) {
                let mut matching_arr = ranges.clone();

                if *operator == '<' {
                    matching_arr[*category] = curr_range.start..*value;

                    queue.push_back((destination.clone(), matching_arr));

                    ranges[*category] = *value..curr_range.end;

                    continue;
                }

                matching_arr[*category] = (*value + 1)..curr_range.end;

                queue.push_back((destination.clone(), matching_arr));

                ranges[*category] = curr_range.start..(*value + 1);

                continue;
            }

            if (*operator == '<' && curr_range.end <= *value)
                || (*operator == '>' && curr_range.start >= *value)
            {
                queue.push_back((destination.clone(), ranges.clone()));
            }
        }
    }

    ans
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let part1 = part1(&input);
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 19 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 409898);
    assert_eq!(part2, 113057405770956);
}
