use std::{collections::VecDeque, fs::read_to_string};

use itertools::Itertools;

type HikeMap = Vec<Vec<u32>>;

fn main() {
    let test = read_map("./input/day_10.test.txt");
    let input = read_map("./input/day_10.txt");

    println!("First part test value: {} == 36", part_1(&test));
    println!("Second part test value: {} == 81", part_2(&test));

    println!("First part answer: {}", part_1(&input));
    println!("Second part answer: {}", part_2(&input));
}

fn part_1(map: &HikeMap) -> usize {
    visit_trails(map)
        .iter()
        .map(|head| head.iter().unique().count())
        .sum()
}

fn part_2(map: &HikeMap) -> usize {
    visit_trails(map).iter().map(|head| head.len()).sum()
}

fn find_trailheads(map: &HikeMap) -> Vec<(usize, usize)> {
    let length = map.len();
    (0..length)
        .map(|y| (0..length).map(move |x| (y, x)))
        .flatten()
        .filter(|&(x, y)| map[y][x] == 0)
        .collect()
}

fn neighbors(map: &HikeMap, position: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = position;
    let (a, b): (i32, i32) = (x as i32, y as i32);
    [(a + 1, b), (a - 1, b), (a, b + 1), (a, b - 1)]
        .iter()
        .filter_map(|&(i, j)| match map.get(j as usize) {
            Some(row) => match row.get(i as usize) {
                Some(_) => Some((i as usize, j as usize)),
                None => None,
            },
            None => None,
        })
        .filter(|&(i, j)| u32::abs_diff(map[y][x], map[j][i]) == 1 && map[y][x] < map[j][i])
        .collect()
}

fn visit_trails(map: &HikeMap) -> Vec<Vec<(usize, usize)>> {
    find_trailheads(map)
        .iter()
        .map(|&start| {
            let mut queue = VecDeque::from([Vec::from([start])]);
            let mut visited: Vec<(usize, usize)> = Vec::new();
            while let Some(trail) = queue.pop_front() {
                let &(x, y) = trail.last().unwrap();

                if map[y][x] == 9 {
                    visited.push((x, y))
                } else {
                    queue.extend(
                        neighbors(map, (x, y))
                            .iter()
                            .map(|p| trail.iter().chain([p]).cloned().collect()),
                    );
                }
            }

            visited
        })
        .collect()
}

fn read_map(file_path: &str) -> HikeMap {
    read_to_string(file_path)
        .unwrap()
        .trim()
        .split("\n")
        .map(|row| row.chars().map(|c| c.to_digit(10)).flatten().collect())
        .collect()
}
