use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = u16;
type InputType = (Map, Coord);
type Map = [[Tile; NC]; NR];
type Coord = (usize, usize);

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Tile {
    Vert,
    Horiz,
    NE, // L
    NW, // J
    SW, // 7
    SE, // F
    Ground,
    Start,
}

impl Tile {
    // direction: Top, Right, Down, Left

    // If self is to the `direction` of a square, is it connected?
    fn is_connected(&self, direction: &Direction) -> bool {
        match direction {
            Direction::Up => self == &Tile::Vert || self == &Tile::SW || self == &Tile::SE,
            Direction::Right => self == &Tile::Horiz || self == &Tile::NW || self == &Tile::SW,
            Direction::Down => self == &Tile::Vert || self == &Tile::NE || self == &Tile::NW,
            Direction::Left => self == &Tile::Horiz || self == &Tile::NE || self == &Tile::SE,
        }
    }

    // Do the pipe on the self tile connect to the neighbour, which is to the `direction`.
    fn is_connected_to_pipe(&self, neighbour: Tile, &direction: &Direction) -> bool {
        if !neighbour.is_connected(&direction) {
            return false;
        }

        match self {
            Tile::Vert => direction == Direction::Up || direction == Direction::Down,
            Tile::Horiz => direction == Direction::Right || direction == Direction::Left,
            Tile::NE => direction == Direction::Up || direction == Direction::Right,
            Tile::NW => direction == Direction::Up || direction == Direction::Left,
            Tile::SW => direction == Direction::Down || direction == Direction::Left,
            Tile::SE => direction == Direction::Down || direction == Direction::Right,
            _ => false,
        }
    }
}

const NR: usize = 140;
const NC: usize = 140;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day10/input.txt").unwrap();

    let mut map = [[Tile::Ground; NC]; NR];
    let mut start_coord = (0, 0);

    for (row, line) in file.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            map[row][col] = match c {
                '|' => Tile::Vert,
                '-' => Tile::Horiz,
                'L' => Tile::NE,
                'J' => Tile::NW,
                '7' => Tile::SW,
                'F' => Tile::SE,
                '.' => Tile::Ground,
                'S' => {
                    start_coord = (row, col);
                    Tile::Start
                }
                _ => panic!(),
            }
        }
    }

    (map, start_coord)
}

fn find_start_neighbours(map: &Map, (row, col): Coord) -> Vec<(Direction, Coord)> {
    let mut queue = Vec::<(Direction, Coord)>::new();

    for (direction, dr, dc) in [
        (Direction::Up, -1, 0),
        (Direction::Right, 0, 1),
        (Direction::Down, 1, 0),
        (Direction::Left, 0, -1),
    ]
    .iter()
    {
        let Ok(rr) = usize::try_from(row as isize + dr) else {
            continue;
        };
        let Ok(cc) = usize::try_from(col as isize + dc) else {
            continue;
        };
        if rr >= NR || cc >= NC {
            continue;
        }

        let neighbour = map[rr][cc];

        if neighbour.is_connected(direction) {
            queue.push((*direction, (rr, cc)));
        }
    }

    queue
}
fn part1((map, start_coord): InputType) -> Int {
    let mut distances = HashMap::<Coord, Int>::from([(start_coord, 0)]);
    let mut queue: VecDeque<(Coord, Int)> = find_start_neighbours(&map, start_coord)
        .iter()
        .map(|(_, (r, c))| ((*r, *c), 1))
        .collect();

    while let Some(((row, col), dist)) = queue.pop_front() {
        if distances.contains_key(&(row, col)) {
            continue;
        }

        distances.insert((row, col), dist);

        let curr_tile = map[row][col];

        for (direction, dr, dc) in [
            (Direction::Up, -1, 0),
            (Direction::Right, 0, 1),
            (Direction::Down, 1, 0),
            (Direction::Left, 0, -1),
        ]
        .iter()
        {
            let Ok(rr) = usize::try_from(row as isize + dr) else {
                continue;
            };
            let Ok(cc) = usize::try_from(col as isize + dc) else {
                continue;
            };
            if rr >= NR || cc >= NC {
                continue;
            }

            let neighbour = map[rr][cc];

            if curr_tile.is_connected_to_pipe(neighbour, direction) {
                queue.push_back(((rr, cc), dist + 1));
            }
        }
    }

    *distances.values().max().unwrap()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn determine_start_tile(d1: &Direction, d2: &Direction) -> Tile {
    match (d1, d2) {
        (Direction::Up, Direction::Right) => Tile::NE,
        (Direction::Up, Direction::Down) => Tile::Horiz,
        (Direction::Up, Direction::Left) => Tile::NW,
        (Direction::Right, Direction::Down) => Tile::SE,
        (Direction::Right, Direction::Left) => Tile::Horiz,
        (Direction::Down, Direction::Left) => Tile::SW,
        _ => panic!(),
    }
}

fn get_left_and_right(
    map: &Map,
    direction_into: Direction,
    row: usize,
    col: usize,
) -> (HashSet<Coord>, HashSet<Coord>) {
    let curr = map[row][col];

    let row = row as isize;
    let col = col as isize;

    let mut lefts = Vec::new();
    let mut rights = Vec::new();

    match (curr, direction_into) {
        (Tile::Vert, Direction::Up) => {
            lefts.push((row, col - 1));
            rights.push((row, col + 1));
        }

        (Tile::Vert, Direction::Down) => {
            lefts.push((row, col + 1));
            rights.push((row, col - 1));
        }

        (Tile::Horiz, Direction::Right) => {
            lefts.push((row - 1, col));
            rights.push((row + 1, col));
        }

        (Tile::Horiz, Direction::Left) => {
            lefts.push((row + 1, col));
            rights.push((row - 1, col));
        }

        (Tile::NE, Direction::Left) => {
            lefts.extend([(row + 1, col), (row + 1, col - 1), (row, col - 1)]);
            rights.push((row - 1, col + 1));
        }

        (Tile::NE, Direction::Down) => {
            rights.extend([(row + 1, col), (row + 1, col - 1), (row, col - 1)]);
            lefts.push((row - 1, col + 1));
        }

        (Tile::NW, Direction::Down) => {
            lefts.extend([(row, col + 1), (row + 1, col + 1), (row + 1, col)]);
            rights.push((row - 1, col - 1));
        }

        (Tile::NW, Direction::Right) => {
            rights.extend([(row, col + 1), (row + 1, col + 1), (row + 1, col)]);
            lefts.push((row - 1, col - 1));
        }

        (Tile::SW, Direction::Up) => {
            lefts.push((row + 1, col - 1));
            rights.extend([(row, col + 1), (row - 1, col + 1), (row - 1, col)]);
        }

        (Tile::SW, Direction::Right) => {
            rights.push((row + 1, col - 1));
            lefts.extend([(row, col + 1), (row - 1, col + 1), (row - 1, col)]);
        }

        (Tile::SE, Direction::Up) => {
            lefts.extend([(row, col - 1), (row - 1, col - 1), (row - 1, col)]);
            rights.push((row + 1, col + 1));
        }

        (Tile::SE, Direction::Left) => {
            rights.extend([(row, col - 1), (row - 1, col - 1), (row - 1, col)]);
            lefts.push((row + 1, col + 1));
        }

        _ => panic!(),
    }

    (
        lefts
            .iter()
            .filter_map(|&(r, c)| {
                let rr = usize::try_from(r).ok()?;
                let cc = usize::try_from(c).ok()?;

                if rr >= NR || cc >= NC {
                    None
                } else {
                    Some((rr, cc))
                }
            })
            .collect(),
        rights
            .iter()
            .filter_map(|&(r, c)| {
                let rr = usize::try_from(r).ok()?;
                let cc = usize::try_from(c).ok()?;

                if rr >= NR || cc >= NC {
                    None
                } else {
                    Some((rr, cc))
                }
            })
            .collect(),
    )
}

fn traverse_and_count_adjacent(
    map: &Map,
    mut direction: Direction,
    (mut row, mut col): Coord,
    pipes: &mut HashSet<Coord>,
    lefts: &mut HashSet<Coord>,
    rights: &mut HashSet<Coord>,
) {
    loop {
        pipes.insert((row, col));

        let curr_tile = map[row][col];

        let (left, right) = get_left_and_right(map, direction, row, col);

        lefts.extend(left);
        rights.extend(right);

        let mut next = None;

        for (dd, dr, dc) in [
            (Direction::Up, -1, 0),
            (Direction::Right, 0, 1),
            (Direction::Down, 1, 0),
            (Direction::Left, 0, -1),
        ]
        .iter()
        {
            let Ok(rr) = usize::try_from(row as isize + dr) else {
                continue;
            };
            let Ok(cc) = usize::try_from(col as isize + dc) else {
                continue;
            };
            if rr >= NR || cc >= NC {
                continue;
            }

            let neighbour = map[rr][cc];

            if curr_tile.is_connected_to_pipe(neighbour, dd) && !pipes.contains(&(rr, cc)) {
                next = Some((*dd, rr, cc));
                break;
            }
        }

        let Some((dd, rr, cc)) = next else {
            return;
        };

        direction = dd;
        row = rr;
        col = cc;
    }
}

fn part2((mut map, (row, col)): InputType) -> Int {
    let start_neighbours = find_start_neighbours(&map, (row, col));

    let (d1, d2): (_, _) = start_neighbours
        .iter()
        .map(|(d, _)| d)
        .sorted_unstable()
        .collect_tuple()
        .unwrap();

    map[row][col] = determine_start_tile(d1, d2);

    let (direction, coord) = start_neighbours.first().unwrap();
    let mut pipes = HashSet::from([(row, col)]);
    let (mut lefts, mut rights) = get_left_and_right(
        &map,
        get_entry_direction_with_exit_direction(map[row][col], direction),
        row,
        col,
    );

    traverse_and_count_adjacent(
        &map,
        *direction,
        *coord,
        &mut pipes,
        &mut lefts,
        &mut rights,
    );

    count_insides(pipes, lefts, rights)
}

fn get_entry_direction_with_exit_direction(tile: Tile, exit_direction: &Direction) -> Direction {
    match (tile, exit_direction) {
        (Tile::Vert, entry) => *entry,
        (Tile::Horiz, entry) => *entry,
        (Tile::NE, Direction::Right) => Direction::Down,
        (Tile::NE, Direction::Up) => Direction::Left,
        (Tile::NW, Direction::Left) => Direction::Down,
        (Tile::NW, Direction::Up) => Direction::Right,
        (Tile::SW, Direction::Down) => Direction::Right,
        (Tile::SW, Direction::Left) => Direction::Up,
        (Tile::SE, Direction::Down) => Direction::Left,
        (Tile::SE, Direction::Right) => Direction::Up,
        _ => panic!(),
    }
}

fn count_insides(pipes: HashSet<Coord>, lefts: HashSet<Coord>, rights: HashSet<Coord>) -> Int {
    let mut is_left = true;
    let mut r = 0;

    // Check if any of the lefts touches the edge
    while is_left && (0..NR).contains(&r) {
        for c in 0..NC {
            if (r == 0 || r == NR - 1 || c == 0 || c == NC - 1)
                && !pipes.contains(&(r, c))
                && lefts.contains(&(r, c))
            {
                is_left = false;
                break;
            }
        }

        r += 1;
    }

    let mut queue: VecDeque<_> = if is_left { lefts } else { rights }
        .iter()
        .copied()
        .filter(|&(r, c)| !pipes.contains(&(r, c)))
        .collect();

    let mut insides = HashSet::new();

    while let Some((r, c)) = queue.pop_front() {
        if insides.contains(&(r, c)) {
            continue;
        }

        insides.insert((r, c));

        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
            let Ok(rr) = usize::try_from(r as isize + dr) else {
                continue;
            };
            let Ok(cc) = usize::try_from(c as isize + dc) else {
                continue;
            };
            if rr >= NR || cc >= NC {
                continue;
            }
            if pipes.contains(&(rr, cc)) {
                continue;
            }
            if insides.contains(&(rr, cc)) {
                continue;
            }

            queue.push_back((rr, cc));
        }
    }

    insides.len() as Int
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let part1 = part1(input);
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 10 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 6754);
    assert_eq!(part2, 567);
}
