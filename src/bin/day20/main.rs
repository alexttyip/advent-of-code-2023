use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs;
use std::time::Instant;

use itertools::Itertools;
use num::integer::gcd;

type Int = u64;
type InputType = HashMap<String, Line>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(BTreeMap<String, bool>),
}

impl Module {
    fn pulse(&mut self, sender: String, pulse: bool) -> Option<bool> {
        match self {
            Module::Broadcaster => None,
            Module::FlipFlop(state) => {
                if pulse {
                    return None;
                }

                *state = !*state;
                Some(*state)
            }
            Module::Conjunction(state) => {
                *state.entry(sender).or_insert(false) = pulse;

                Some(!state.values().all(|v| *v))
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ModuleTypes {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Line {
    module_type: ModuleTypes,
    children: Vec<String>,
}

fn read_input() -> InputType {
    let file = fs::read_to_string("/input.txt").unwrap();

    let mut modules = HashMap::new();

    for line in file.lines() {
        let (mut name, dest) = line.split_once(" -> ").unwrap();

        let children = dest.split(", ").map(|s| s.to_string()).collect_vec();

        let mut module_type = ModuleTypes::Broadcaster;

        if name != "broadcaster" {
            let (c, module_name) = name.split_at(1);

            module_type = match c {
                "%" => ModuleTypes::FlipFlop,
                "&" => ModuleTypes::Conjunction,
                &_ => panic!("Module type not recognised"),
            };

            name = module_name;
        }

        modules.insert(
            name.to_string(),
            Line {
                module_type,
                children,
            },
        );
    }

    modules
}

fn part1(modules: InputType) -> Int {
    let mut parents_by_child: HashMap<String, Vec<String>> = HashMap::new();

    for (k, v) in modules.iter() {
        v.children.iter().for_each(|child| {
            parents_by_child
                .entry(child.clone())
                .or_default()
                .push(k.clone());
        })
    }

    let mut state: HashMap<_, _> = HashMap::new();

    let mut l: u64 = 0;
    let mut h = 0;

    for _ in 0..1000 {
        l += 1; // Button click

        let mut queue = VecDeque::from([("broadcaster".to_string(), false)]);

        while let Some((head, pulse)) = queue.pop_front() {
            let children = modules.get(&head).unwrap().children.clone();

            for child in children {
                if pulse {
                    h += 1;
                } else {
                    l += 1;
                }

                let Some(v) = modules.get(&child) else {
                    continue;
                };

                if let Some(new_pulse) = state
                    .entry(child.clone())
                    .or_insert_with(|| match v.module_type {
                        ModuleTypes::Broadcaster => Module::Broadcaster,
                        ModuleTypes::FlipFlop => Module::FlipFlop(false),
                        ModuleTypes::Conjunction => Module::Conjunction(BTreeMap::from_iter(
                            parents_by_child
                                .get(&child)
                                .unwrap()
                                .iter()
                                .map(|parent| (parent.clone(), false)),
                        )),
                    })
                    .pulse(head.to_string(), pulse)
                {
                    queue.push_back((child, new_pulse));
                };
            }
        }
    }

    l * h
}

fn part2(modules: InputType) -> Int {
    let mut parents_by_child: HashMap<String, Vec<String>> = HashMap::new();

    for (k, v) in modules.iter() {
        v.children.iter().for_each(|child| {
            parents_by_child
                .entry(child.clone())
                .or_default()
                .push(k.clone());
        })
    }

    let mut state: HashMap<_, _> = HashMap::new();

    let mut parents = HashMap::from(
        [("tx", 0), ("dd", 0), ("nz", 0), ("ph", 0)].map(|(k, v)| (k.to_string(), v)),
    );

    for i in 0.. {
        let mut queue = VecDeque::from([("broadcaster".to_string(), false)]);

        while let Some((head, pulse)) = queue.pop_front() {
            let children = modules.get(&head).unwrap().children.clone();

            if pulse {
                if let Some(rx_parent) = parents.get_mut(&head) {
                    *rx_parent = i + 1;
                }
            }

            for child in children {
                let Some(v) = modules.get(&child) else {
                    continue;
                };

                if let Some(new_pulse) = state
                    .entry(child.clone())
                    .or_insert_with(|| match v.module_type {
                        ModuleTypes::Broadcaster => Module::Broadcaster,
                        ModuleTypes::FlipFlop => Module::FlipFlop(false),
                        ModuleTypes::Conjunction => Module::Conjunction(BTreeMap::from_iter(
                            parents_by_child
                                .get(&child)
                                .unwrap()
                                .iter()
                                .map(|parent| (parent.clone(), false)),
                        )),
                    })
                    .pulse(head.to_string(), pulse)
                {
                    queue.push_back((child, new_pulse));
                };
            }
        }

        if parents.values().all(|v| *v > 0) {
            return parents.values().fold(1, |acc, v| (v * acc) / gcd(*v, acc));
        }
    }

    panic!();
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

    println!("--- Day 20 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 869395600);
    assert_eq!(part2, 232605773145467);
}
