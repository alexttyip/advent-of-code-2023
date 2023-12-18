use std::fs;
use std::time::Instant;

use itertools::Itertools;
use num::Complex;

type Int = i64;
type InputType = Vec<(char, Int, String)>;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day18/input.txt").unwrap();

    let mut plan = vec![];

    for line in file.lines() {
        let parts = line.split_whitespace().collect_vec();

        let dir = parts[0].chars().next().unwrap();
        let magnitude = parts[1].parse::<Int>().unwrap();
        let hex = parts[2].trim_matches(&['(', '#', ')'][..]).to_string();

        plan.push((dir, magnitude, hex));
    }

    plan
}

fn get_delta_for_direction(dir: char) -> Complex<Int> {
    match dir {
        'R' | '0' => Complex::new(1, 0),
        'D' | '1' => Complex::new(0, -1),
        'L' | '2' => Complex::new(-1, 0),
        'U' | '3' => Complex::new(0, 1),
        _ => panic!(),
    }
}

fn solve(input: InputType) -> (Int, Int) {
    let mut curr_p1 = Complex::new(0, 0);
    let mut vertices_p1: Vec<Complex<Int>> = Vec::from([curr_p1]);
    let mut perimeter_p1 = 0;

    let mut curr_p2 = Complex::new(0, 0);
    let mut vertices_p2: Vec<Complex<Int>> = Vec::from([curr_p2]);
    let mut perimeter_p2 = 0;

    for (dir, magnitude, hex) in input {
        curr_p1 += get_delta_for_direction(dir).scale(magnitude);
        vertices_p1.push(curr_p1);
        perimeter_p1 += magnitude;

        let dir = hex.chars().last().unwrap();
        let magnitude = Int::from_str_radix(&hex[..5], 16).unwrap();

        curr_p2 += get_delta_for_direction(dir).scale(magnitude);
        vertices_p2.push(curr_p2);
        perimeter_p2 += magnitude;
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for i in 0..vertices_p1.len() - 1 {
        p1 += (vertices_p1[i].re * vertices_p1[i + 1].im)
            - (vertices_p1[i].im * vertices_p1[i + 1].re);
        p2 += (vertices_p2[i].re * vertices_p2[i + 1].im)
            - (vertices_p2[i].im * vertices_p2[i + 1].re);
    }

    (
        p1.abs() / 2 + 1 + perimeter_p1 / 2,
        p2.abs() / 2 + 1 + perimeter_p2 / 2,
    )
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let (part1, part2) = solve(input.clone());
    let solve_elapsed = now.elapsed();

    println!("--- Day 18 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Both parts combined took: {:.2?}", solve_elapsed);

    assert_eq!(part1, 28911);
    assert_eq!(part2, 77366737561114);
}
