use std::collections::HashMap;
use std::fs;
use std::iter::repeat;
use std::time::Instant;

use itertools::Itertools;

type Int = u64;
type InputType = Vec<(String, Vec<usize>)>;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day12/input.txt").unwrap();

    let mut springs = Vec::new();

    for line in file.lines() {
        let (s, g) = line.split_once(' ').unwrap();

        springs.push((
            s.to_string(),
            g.split(',').filter_map(|n| n.parse().ok()).collect(),
        ));
    }

    springs
}

fn count_matches(
    springs: &str,
    groups: Vec<usize>,
    from: usize,
    memo: &mut HashMap<(String, Vec<usize>), Int>,
) -> Int {
    if from >= springs.len() {
        if groups.is_empty() {
            return 1;
        }

        return 0;
    }

    let Some(group_length) = groups.first() else {
        if springs[from..].contains('#') {
            return 0;
        }

        return 1;
    };

    if (from + group_length) > springs.len() {
        return 0;
    }

    let memo_key = (springs[from..].to_string(), groups.clone());

    if let Some(ans) = memo.get(&memo_key) {
        return *ans;
    }

    let mut ans = 0;

    // Check if any corresponding springs are '.'
    if !&springs[from..(from + group_length)].contains('.') {
        // Check if the spring immediately after is '#'
        if springs.chars().nth(from + group_length) != Some('#') {
            // This slice is possible, check the rest
            ans += count_matches(springs, groups[1..].to_vec(), from + group_length + 1, memo);
        }
    }

    if springs.chars().nth(from) != Some('#') {
        ans += count_matches(springs, groups, from + 1, memo);
    }

    memo.insert(memo_key, ans);

    ans
}

fn solve(input: InputType) -> (Int, Int) {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut memo = HashMap::new();

    for (springs, groups) in input {
        let new_springs = repeat(&springs).take(5).join("?");
        let new_groups = repeat(groups.clone()).take(5).flatten().collect_vec();

        p1 += count_matches(springs.as_str(), groups, 0, &mut memo);
        p2 += count_matches(new_springs.as_str(), new_groups, 0, &mut memo);
    }

    (p1, p2)
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let (part1, part2) = solve(input.clone());
    let solve_elapsed = now.elapsed();

    println!("--- Day 12 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Both part together took: {:.2?}", solve_elapsed);

    assert_eq!(part1, 6949);
    assert_eq!(part2, 51456609952403);
}
