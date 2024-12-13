use itertools::Itertools;
use regex::Regex;
use rayon::prelude::*;
use std::{collections::HashSet, fs::read_to_string};

pub enum WalkResult<T> {
    Stop(T),
    Turn(T),
}

pub enum PatrolResult<T> {
    Ok(T),
    Loop,
}
impl<T> PatrolResult<T> {
    fn unwrap(self) -> T {
        match self {
            PatrolResult::Ok(value) => value,
            PatrolResult::Loop => panic!("Unhandled loop detected!"),
        }
    }
}

type Point = (i32, i32);
type Direction = (i32, i32);
type VisitedPoint = (Point, Direction);

fn main() {
    let test_map = read_map("./input/day_6.test.txt");
    println!("First part test answer: {} == 41", part_1(&test_map));

    let map = read_map("./input/day_6.txt");
    println!("First part answer: {}", part_1(&map));

    println!("Second part test answer: {} == 6", part_2(&test_map));
    println!("Second part answer: {}", part_2(&map));
}

fn get_map(map: &Vec<String>, position: (i32, i32)) -> Option<char> {
    let (x, y) = position;
    match map.get(y as usize) {
        Some(row) => match row.as_bytes().get(x as usize) {
            Some(block) => Some(*block as char),
            None => None,
        },
        None => None,
    }
}

fn part_1(map: &Vec<String>) -> u32 {
    let start = starting_position(map);
    patrol(map, start).unwrap().len() as u32
}

fn part_2(map: &Vec<String>) -> u32 {
    let start = starting_position(map);
    let visited: HashSet<Point> = HashSet::from_iter(
        patrol(map, start)
            .unwrap()
            .iter()
            .filter(|&x| *x != start)
            .copied(),
    );

    return visited
        .par_iter()
        .map(|(x, y)| {
            let mut copy: Vec<String> = map.iter().cloned().collect();
            copy[*y as usize].replace_range((*x as usize)..((*x + 1) as usize), "#");
            copy
        })
        .filter(|map| match patrol(map, start) {
            PatrolResult::Ok(_) => false,
            PatrolResult::Loop => true,
        })
        .count() as u32;
}

fn patrol(map: &Vec<String>, start: (i32, i32)) -> PatrolResult<HashSet<Point>> {
    let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();

    let mut current = start;
    let mut direction = *directions.next().unwrap();
    let mut all_visited: HashSet<VisitedPoint> = HashSet::new();
    loop {
        match walk(map, current, direction) {
            WalkResult::Turn((visited, next)) => {
                if visited.par_iter().any(|vp| all_visited.contains(vp)) {
                    return PatrolResult::Loop;
                }

                all_visited.extend::<HashSet<VisitedPoint>>(HashSet::from_iter(visited));
                current = next;
                direction = *directions.next().unwrap();
            }
            WalkResult::Stop((visited, _)) => {
                all_visited.extend::<HashSet<VisitedPoint>>(HashSet::from_iter(visited));
                break;
            }
        }
    }
    return PatrolResult::Ok(all_visited.iter().map(|(p, _)| p).unique().copied().collect());
}

fn walk(
    map: &Vec<String>,
    start: (i32, i32),
    direction: (i32, i32),
) -> WalkResult<(Vec<VisitedPoint>, (i32, i32))> {
    let mut current = start;
    let mut visited: Vec<VisitedPoint> = Vec::new();

    while let Some(next) = get_map(map, (current.0 + direction.0, current.1 + direction.1)) {
        match next {
            '#' => return WalkResult::Turn((visited, current)),
            _ => {
                visited.push((current, direction));
                current = (current.0 + direction.0, current.1 + direction.1);
            }
        }
    }

    visited.push((current, direction));
    return WalkResult::Stop((visited, current));
}

fn starting_position(map: &Vec<String>) -> (i32, i32) {
    let pattern = Regex::new(r"\^").unwrap();
    map.iter()
        .enumerate()
        .filter_map(|(y, row)| match pattern.find(row) {
            Some(m) => return Some((m.start() as i32, y as i32)),
            None => return None,
        })
        .next()
        .unwrap()
}

fn read_map(file_path: &str) -> Vec<String> {
    let buffer = read_to_string(file_path).unwrap();
    return buffer.trim().split("\n").map(|x| x.to_string()).collect();
}
