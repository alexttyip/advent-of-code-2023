use std::collections::VecDeque;
use std::fs;
use std::mem::swap;
use std::time::Instant;

use itertools::Itertools;

type Int = u64;
type InputType = (Vec<Int>, Vec<(Vec<(Int, Int)>, Vec<(Int, Int)>)>);

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day05/input.txt").unwrap();

    let mut chunks = file.split("\n\n");

    let seeds = chunks
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<Int>().ok())
        .collect_vec();

    let mut maps = Vec::with_capacity(10);

    for chunk in chunks {
        let mut srcs = vec![];
        let mut dests = vec![];

        for line in chunk.lines().skip(1) {
            let (dest, src, n): (Int, Int, Int) = line
                .split_whitespace()
                .filter_map(|s| s.parse::<Int>().ok())
                .collect_tuple()
                .unwrap();

            let i = srcs.partition_point(|x: &(Int, Int)| x.0 < src);

            srcs.insert(i, (src, src + n));
            dests.insert(i, (dest, dest + n));
        }

        maps.push((srcs, dests))
    }

    (seeds, maps)
}

fn part1((mut seeds, maps): InputType) -> Int {
    for (srcs, dests) in maps {
        for seed in seeds.iter_mut() {
            let mut i = srcs.partition_point(|x| x.0 <= *seed);

            if i == 0 {
                continue;
            }

            i -= 1;
            let src = srcs[i];

            if *seed < src.1 {
                *seed -= src.0;
                *seed += dests[i].0;
                continue;
            }
        }
    }

    *seeds.iter().min().unwrap()
}

//                            | src.0 ----------------------------- src.1 |
//                            | dest.0 --------------------------- dest.1 |
// 1 | seed_from - seed_to |
// 2 | seed_from ---------- seed_to |
// 3 | seed_from ------------------------------------------------------------------------- seed_to |
// 4                                    | seed_from ----- seed_to |
// 5                                    | seed_from -------------------------------------- seed_to |
// 6                                                                         | seed_from - seed_to |

fn part2((seeds, maps): InputType) -> Int {
    let mut seeds: VecDeque<_> = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect();

    let mut queue = VecDeque::<(Int, Int)>::new();

    for (srcs, dests) in maps {
        while let Some((seed_from, seed_to)) = seeds.pop_front() {
            let mut i = srcs.partition_point(|x| x.0 <= seed_from);

            if i == 0 {
                let src = &srcs[i];

                // Case 1
                if seed_to < src.0 {
                    queue.push_back((seed_from, seed_to));
                    continue;
                }

                // Case 2 & 3
                queue.push_back((seed_from, src.0));
                seeds.push_back((src.0, seed_to));
                continue;
            }

            i -= 1;
            let src = &srcs[i];
            let dest = &dests[i];

            let seed_dest_start = dest.0 + (seed_from - src.0);

            // Case 4
            if seed_to < src.1 {
                queue.push_back((seed_dest_start, seed_dest_start + (seed_to - seed_from)));
                continue;
            }

            // Case 5
            if seed_from < src.1 {
                queue.push_back((seed_dest_start, dest.1));
                seeds.push_back((src.1, seed_to));
                continue;
            }

            // Case 6
            queue.push_back((seed_from, seed_to));
        }

        swap(&mut seeds, &mut queue);
    }

    seeds.iter().fold(Int::MAX, |acc, (curr, _)| acc.min(*curr))
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

    println!("--- Day 05 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 240320250);
    assert_eq!(part2, 28580589);
}
