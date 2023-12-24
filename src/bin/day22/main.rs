use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = usize;
type Coord = (Int, Int, Int);
type InputType = Vec<(Coord, Coord)>;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day22/input.txt").unwrap();

    let mut coords: InputType = Vec::new();

    for line in file.lines() {
        let (l, r) = line.split_once('~').unwrap();

        let ll = l
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect_tuple()
            .unwrap();
        let rr = r
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect_tuple()
            .unwrap();

        coords.push((ll, rr));
    }

    coords.sort_unstable_by_key(|(_, r)| r.2);

    coords
}

const N: Int = 10;

// Height, block_idx
type Map = [[(Int, Int); N]; N];

fn get_parents_of_blocks(input: &InputType) -> BTreeMap<Int, BTreeSet<Int>> {
    let mut map: Map = [[(0, 0); N]; N];

    let mut parents_by_blocks = BTreeMap::new();

    for (i, (l, r)) in input.iter().enumerate() {
        let i = i + 1;
        let mut max_z = 0;
        let mut parents = BTreeSet::new();

        for ys in map.iter().take(r.0 + 1).skip(l.0) {
            for (z, block_idx) in ys.iter().take(r.1 + 1).skip(l.1) {
                match z.cmp(&max_z) {
                    Ordering::Greater => {
                        max_z = *z;
                        parents.clear();
                    }
                    Ordering::Less => continue,
                    _ => {}
                }

                parents.insert(*block_idx);
            }
        }

        max_z += r.2 - l.2 + 1;

        for ys in map.iter_mut().take(r.0 + 1).skip(l.0) {
            ys[l.1..=r.1].fill((max_z, i));
        }

        parents_by_blocks.insert(i, parents);
    }

    parents_by_blocks
}

fn part1(input: InputType) -> Int {
    let single_parents_count = get_parents_of_blocks(&input)
        .values()
        .filter_map(|parents| {
            if parents.len() == 1 {
                parents.first()
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>()
        .len();

    input.len() + 1 - single_parents_count
}

fn get_descendants_count(
    children_by_block: &BTreeMap<Int, BTreeSet<Int>>,
    parents_by_block: &BTreeMap<Int, BTreeSet<Int>>,
    block: &Int,
) -> Int {
    let Some(children) = children_by_block.get(block) else {
        return 0;
    };

    if children
        .iter()
        .all(|child| parents_by_block.get(child).unwrap().len() != 1)
    {
        return 0;
    }

    let mut all_descendants_and_self = BTreeSet::from([*block]);
    let mut queue = VecDeque::from_iter(children);
    let mut count = 0;

    while let Some(curr) = queue.pop_front() {
        if all_descendants_and_self.contains(curr) {
            continue;
        }

        if parents_by_block
            .get(curr)
            .unwrap()
            .is_subset(&all_descendants_and_self)
        {
            all_descendants_and_self.insert(*curr);

            if let Some(new_children) = children_by_block.get(curr) {
                queue.extend(new_children);
            }

            count = 0;
            continue;
        }

        queue.push_back(curr);
        count += 1;

        if count >= queue.len() {
            break;
        }
    }

    all_descendants_and_self.len() - 1
}

fn part2(input: InputType) -> Int {
    let parents_by_blocks = get_parents_of_blocks(&input);

    let mut children_by_block = BTreeMap::<Int, BTreeSet<Int>>::new();

    for (block, parents) in &parents_by_blocks {
        for parent in parents {
            children_by_block
                .entry(*parent)
                .and_modify(|set| {
                    set.insert(*block);
                })
                .or_insert(BTreeSet::from([*block]));
        }
    }

    (1..=input.len())
        .map(|i| get_descendants_count(&children_by_block, &parents_by_blocks, &i))
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

    println!("--- Day 22 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 480);
    assert_eq!(part2, 84021);
}
