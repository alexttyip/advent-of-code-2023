use itertools::Itertools;
use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

type Int = u16;
type InputType = Vec<Stone>;

type Float = f64;

type Stone = [Axis; 3];

#[derive(Debug, Copy, Clone)]
struct Axis {
    pos: Float,
    delta: Float,
}

fn m(axis1: &Axis, axis2: &Axis) -> Float {
    axis2.delta / axis1.delta
}

fn c(axis1: &Axis, axis2: &Axis) -> Float {
    let m = m(axis1, axis2);

    axis2.pos - (m * axis1.pos)
}

// fn is_point_in_future_trajectory(stone: Stone, (xx, yy): &(Float, Float)) -> bool {
//     let dxx = xx - stone.x;
//     let dyy = yy - stone.y;
//
//     dxx.signum() == stone.dx.signum() && dyy.signum() == self.dy.signum()
// }

fn intersect(a: &Stone, b: &Stone, n_axis: usize) -> Option<Vec<Float>> {
    let mut intersection = vec![None; n_axis];

    for i in 0..n_axis {
        for j in (i + 1)..n_axis {
            let ai = a[i];
            let aj = a[j];
            let bi = b[i];
            let bj = b[j];

            let ma = m(&ai, &aj);
            let mb = m(&bi, &bj);
            let ca = c(&ai, &aj);
            let cb = c(&bi, &bj);

            let intersection_i = (ca - cb) / (mb - ma);
            let intersection_j = (mb * ca - ma * cb) / (mb - ma);

            if !RANGE.contains(&intersection_i) || !RANGE.contains(&intersection_j) {
                return None;
            }

            if (intersection_i - ai.pos).signum() == ai.delta.signum()
                && (intersection_j - aj.pos).signum() == aj.delta.signum()
                && (intersection_i - bi.pos).signum() == bi.delta.signum()
                && (intersection_j - bj.pos).signum() == bj.delta.signum()
            {
                match intersection[i] {
                    Some(ii) if ii != intersection_i => {
                        return None;
                    }
                    _ => {
                        intersection[i] = Some(intersection_i);
                    }
                }

                match intersection[j] {
                    Some(ij) if ij != intersection_j => {
                        return None;
                    }
                    _ => {
                        intersection[j] = Some(intersection_j);
                    }
                }
            }
        }
    }

    let intersection = intersection.iter().flatten().copied().collect_vec();

    if intersection.is_empty() {
        None
    } else {
        Some(intersection)
    }
}

// #[derive(Debug, Copy, Clone)]
// struct Stone {
//     x: Float,
//     y: Float,
//     z: Float,
//     dx: Float,
//     dy: Float,
//     dz: Float,
// }
//
// impl Stone {
//     fn m(&self) -> Float {
//         self.dy / self.dx
//     }
//
//     fn c(&self) -> Float {
//         self.y - (self.m() * self.x)
//     }
//
//     fn intersection(&self, other: &Stone) -> (Float, Float) {
//         let m1 = self.m();
//         let c1 = self.c();
//         let m2 = other.m();
//         let c2 = other.c();
//
//         let x = (c1 - c2) / (m2 - m1);
//         let y = (m2 * c1 - m1 * c2) / (m2 - m1);
//
//         (x, y)
//     }
//
//     fn is_point_in_future_trajectory(&self, (xx, yy): &(Float, Float)) -> bool {
//         let dxx = xx - self.x;
//         let dyy = yy - self.y;
//
//         dxx.signum() == self.dx.signum() && dyy.signum() == self.dy.signum()
//     }
// }
//
fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day24/input.txt").unwrap();

    let mut stones = Vec::new();

    for line in file.lines() {
        let mut nums = line.split(&[',', ' ', '@'][..]).filter(|s| !s.is_empty());

        let x = nums.next().and_then(|s| s.parse::<Float>().ok()).unwrap();
        let y = nums.next().and_then(|s| s.parse::<Float>().ok()).unwrap();
        let z = nums.next().and_then(|s| s.parse::<Float>().ok()).unwrap();
        let dx = nums.next().and_then(|s| s.parse::<Float>().ok()).unwrap();
        let dy = nums.next().and_then(|s| s.parse::<Float>().ok()).unwrap();
        let dz = nums.next().and_then(|s| s.parse::<Float>().ok()).unwrap();

        stones.push([
            Axis { pos: x, delta: dx },
            Axis { pos: y, delta: dy },
            Axis { pos: z, delta: dz },
        ])
    }

    stones
}

const RANGE: RangeInclusive<Float> = 200000000000000.0..=400000000000000.0;

fn part1(input: InputType) -> Int {
    let mut count = 0;

    for pair in input.iter().combinations(2) {
        let [stone1, stone2] = pair[..] else {
            panic!();
        };

        if let Some(intersection) = intersect(stone1, stone2, 2) {
            // dbg!(intersection);
            count += 1;
        }

        // let intersection = stone1.intersection(stone2);
        //
        // if !RANGE.contains(&intersection.0) || !RANGE.contains(&intersection.1) {
        //     continue;
        // }
        //
        // if stone1.is_point_in_future_trajectory(&intersection)
        //     && stone2.is_point_in_future_trajectory(&intersection)
        // {
        //     dbg!(intersection);
        //     println!();
        //     count += 1;
        // }
    }

    count
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

    println!("--- Day 24 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    // assert_eq!(part1, 0);
    // assert_eq!(part2, 0);
}
