use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::Hash;
use std::time::Instant;

use itertools::Itertools;

type Int = u64;
type Broadcasts = Vec<String>;
type Modules = BTreeMap<String, (Module, Vec<String>)>;
type InputType = (Broadcasts, Modules);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Module {
    FlipFlop(bool),
    Conjunction(BTreeMap<String, bool>),
}

// impl Hash for Module {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         match self {
//             Module::FlipFlop(_) => {}
//             Module::Conjunction(_) => {}
//         }
//
//         // self.id.hash(state);
//         // self.phone.hash(state);
//     }
// }
//
fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day20/input.txt").unwrap();

    let mut broadcasts = Vec::new();

    let mut modules_with_parents_and_dest_and_is_flip_flop =
        HashMap::<String, (Vec<String>, Vec<String>, bool)>::new();

    for line in file.lines() {
        let (mut name, dest) = line.split_once(" -> ").unwrap();

        let destinations = dest.split(", ").map(|s| s.to_string()).collect_vec();

        if name == "broadcaster" {
            broadcasts = destinations.clone();
        } else {
            let (module_type, module_name) = name.split_at(1);

            name = module_name;

            modules_with_parents_and_dest_and_is_flip_flop
                .entry(name.to_string())
                .and_modify(|v| {
                    v.1 = destinations.clone();
                    v.2 = module_type == "%";
                })
                .or_insert((vec![], destinations.clone(), module_type == "%"));
        }

        for dest in &destinations {
            modules_with_parents_and_dest_and_is_flip_flop
                .entry(dest.to_string())
                .and_modify(|v| v.0.push(name.to_string()))
                .or_insert((vec![name.to_string()], vec![], false));
        }
    }

    let modules: BTreeMap<String, (Module, Vec<String>)> =
        modules_with_parents_and_dest_and_is_flip_flop
            .iter()
            .map(
                |(name, (parents, destinations, is_flip_flop))| -> (String, (Module, Vec<String>)) {
                    if *is_flip_flop {
                        return (
                            name.clone(),
                            (Module::FlipFlop(false), destinations.clone()),
                        );
                    }

                    let states = parents
                        .iter()
                        .map(|parent| (parent.to_owned(), false))
                        .collect();

                    (
                        name.clone(),
                        (Module::Conjunction(states), destinations.clone()),
                    )
                },
            )
            .collect();

    (broadcasts, modules)
}

fn part1((broadcasts, mut modules): InputType) -> Int {
    // dbg!(&modules);

    let mut states = HashSet::<(Int, Int)>::new();
    let mut states_l = HashSet::new();
    let mut states_h = HashSet::new();

    let mut low_count = 0;
    let mut high_count = 0;

    let mut queue = VecDeque::new();

    for i in 0..1000000 {
        let mut l = 0;
        let mut h = 0;
        // Button press
        l += 1;

        queue.extend(
            broadcasts
                .iter()
                .map(|m| ("broadcast".to_string(), m.to_string(), false)),
        );

        while let Some((from, name, pulse)) = queue.pop_front() {
            // dbg!(&from, &pulse, &name);

            if !pulse {
                l += 1;
            } else {
                h += 1;
            }

            modules
                .entry(name.clone())
                .and_modify(|(module, dest)| match module {
                    Module::FlipFlop(is_on) => {
                        if pulse {
                            return;
                        }

                        // Flip the state
                        *is_on ^= true;

                        queue.extend(dest.iter().map(|d| (name.clone(), d.clone(), *is_on)));
                    }
                    Module::Conjunction(states) => {
                        // dbg!(&states);
                        *states.get_mut(&from).unwrap() = pulse;

                        let output = !states.values().all(|b| *b);
                        queue.extend(dest.iter().map(|d| (name.clone(), d.clone(), output)));
                    }
                });

            // println!();
        }

        if states_l.insert(l) {
            println!("Unique low at {}, {:?}", i, &l);
        }

        if states_h.insert(h) {
            println!("Unique high at {}, {:?}", i, &h);
        }

        low_count += l;
        high_count += h;
    }

    dbg!(&low_count, &high_count);

    high_count * low_count
}

#[test]
fn ex1() {
    let broadcasts = vec![String::from("a"), String::from("b"), String::from("c")];
    let modules = BTreeMap::from([
        (
            String::from("a"),
            (Module::FlipFlop(false), vec![String::from("b")]),
        ),
        (
            String::from("b"),
            (Module::FlipFlop(false), vec![String::from("c")]),
        ),
        (
            String::from("c"),
            (Module::FlipFlop(false), vec![String::from("inv")]),
        ),
        (
            String::from("inv"),
            (
                Module::Conjunction(BTreeMap::from([(String::from("c"), false)])),
                vec![String::from("a")],
            ),
        ),
    ]);

    assert_eq!(part1((broadcasts, modules)), 32000000);
}

#[test]
fn ex2() {
    let broadcasts = vec![String::from("a")];
    let modules = BTreeMap::from([
        (
            String::from("a"),
            (
                Module::FlipFlop(false),
                vec![String::from("inv"), String::from("con")],
            ),
        ),
        (
            String::from("inv"),
            (
                Module::Conjunction(BTreeMap::from([(String::from("a"), false)])),
                vec![String::from("b")],
            ),
        ),
        (
            String::from("b"),
            (Module::FlipFlop(false), vec![String::from("con")]),
        ),
        (
            String::from("con"),
            (
                Module::Conjunction(BTreeMap::from([
                    (String::from("a"), false),
                    (String::from("b"), false),
                ])),
                vec![String::from("output")],
            ),
        ),
    ]);

    assert_eq!(part1((broadcasts, modules)), 11687500);
}

fn part2(input: InputType) -> Int {
    0
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

    // assert_eq!(part1, 0);
    // assert_eq!(part2, 0);
}
