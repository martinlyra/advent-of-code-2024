mod day_16;

use std::collections::HashMap;

use day_16::{find_paths, read_input, Path};
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let test = read_input("./input/day_20.test.txt");
    let input = read_input("./input/day_20.txt");

    // Challenge unit-tests
    let test_paths = find_paths(&test);
    let test_path = test_paths.iter().next().unwrap();

    let map = part_1(&test_path, 0);
    println!("First part test answer:",);
    for key in map.keys().sorted() {
        println!("- {key}: {}", map[key]);
    }

    let map = part_2(&test_path, 50);
    println!("Second part test answer:",);
    for key in map.keys().sorted() {
        println!("- {key}: {}", map[key]);
    }

    // Challenge input
    let input_paths = find_paths(&input);
    let input_path = input_paths.iter().next().unwrap();
    let map = part_1(&input_path, 100);
    println!("First part answer: {}", map.values().sum::<i32>());
    let map = part_2(&input_path, 100);
    println!("Second part answer: {}", map.values().sum::<i32>());
}

fn manhattan(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn find_cheat_points(path: &Path, max_length: usize) -> Vec<(usize, usize, usize)> {
    (0..path.visited.len())
        .into_par_iter()
        .map(|i| {
            let p = path.visited[i];
            let tail = (i + 1)..path.visited.len();
            tail.into_iter()
                .filter_map(move |j| {
                    let o = path.visited[j];
                    let l = manhattan(&p, &o);
                    if l <= max_length {
                        Some((i, j, l))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn eval(path: &Path, max_length: usize, minimum: usize) -> HashMap<usize, i32> {
    find_cheat_points(path, max_length)
        .par_iter()
        .map(|(i, j, l)| j - i - l)
        .filter(|&x| x >= minimum)
        .collect::<Vec<_>>()
        .iter()
        .fold(HashMap::new(), |mut map, &f| {
            map.entry(f).and_modify(|c| *c += 1).or_insert(1);
            map
        })
}

fn part_1(path: &Path, minimum: usize) -> HashMap<usize, i32> {
    eval(path, 2, minimum)
}

fn part_2(path: &Path, minimum: usize) -> HashMap<usize, i32> {
    eval(path, 20, minimum)
}
