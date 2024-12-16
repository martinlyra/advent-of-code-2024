use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
    usize,
};

use itertools::Itertools;

struct Path {
    visited: Vec<(usize, usize)>,
    last_direction: (i32, i32),
    turns: usize,
}

impl Path {
    fn score(&self) -> usize {
        self.visited.len() - 1 + self.turns * 1000
    }
}

fn main() {
    let test_1 = read_input("./input/day_16.test.1.txt");
    let test_2 = read_input("./input/day_16.test.2.txt");
    let input = read_input("./input/day_16.txt");

    let paths_t1 = find_paths(&test_1);
    let paths_t2 = find_paths(&test_2);
    let paths_in = find_paths(&input);

    println!("First part test 1 answer: {} == 7036", part_1(&paths_t1));
    println!("First part test 2 answer: {} == 11048", part_1(&paths_t2));
    println!("First part answer: {}", part_1(&paths_in));

    println!("Second part test 1 answer: {} == 45", part_2(&paths_t1));
    println!("Second part test 2 answer: {} == 64", part_2(&paths_t2));
    println!("Second part answer: {}", part_2(&paths_in));
}

fn part_1(input: &Vec<Path>) -> usize {
    input
        .iter()
        .map(|path| path.score())
        .sorted()
        .next()
        .unwrap()
}

fn part_2(input: &Vec<Path>) -> usize {
    let best_score = part_1(input);
    input
        .iter()
        .filter(|path| path.score() == best_score)
        .fold(HashSet::new(), |mut a, f| {
            a.extend(f.visited.clone());
            a
        })
        .len()
}

fn find_position_of(input: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, *c)))
        .flatten()
        .filter_map(|(x, y, c)| if c == target { Some((x, y)) } else { None })
        .next()
}

fn find_paths(input: &Vec<Vec<char>>) -> Vec<Path> {
    let start = find_position_of(input, 'S').unwrap();
    let end = find_position_of(input, 'E').unwrap();
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut paths: Vec<Path> = Vec::new();
    let mut best_visited: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
    let mut queue = VecDeque::from(vec![Path {
        visited: vec![start],
        last_direction: (1, 0),
        turns: 0,
    }]);

    while let Some(unfinished) = queue.pop_front() {
        let &last_position = unfinished.visited.last().unwrap();

        for (x, y, d) in directions.iter().map(|&(dx, dy)| {
            (
                (last_position.0 as i32 + dx) as usize,
                (last_position.1 as i32 + dy) as usize,
                (dx, dy),
            )
        }) {
            if unfinished.visited.contains(&(x, y)) {
                continue;
            }

            let turned = if unfinished.last_direction == d { 0 } else { 1 };

            match input[y][x] {
                '.' => {
                    let new = Path {
                        visited: unfinished
                            .visited
                            .iter()
                            .chain([&(x, y)])
                            .cloned()
                            .collect(),
                        last_direction: d,
                        turns: unfinished.turns + turned,
                    };

                    let k = (last_position, (x, y));
                    if new.score() <= *best_visited.entry(k).or_insert(usize::MAX) {
                        best_visited.insert(k, new.score());
                        queue.push_back(new);
                    }
                }
                'E' => {
                    assert_eq!((x, y), end);
                    paths.push(Path {
                        visited: unfinished
                            .visited
                            .iter()
                            .chain([&(x, y)])
                            .cloned()
                            .collect(),
                        last_direction: d,
                        turns: unfinished.turns + turned,
                    });
                }
                _ => continue,
            }
        }
    }

    paths
}

fn read_input(file_path: &str) -> Vec<Vec<char>> {
    read_to_string(file_path)
        .unwrap()
        .trim()
        .split("\n")
        .map(|row| row.chars().collect())
        .collect()
}
