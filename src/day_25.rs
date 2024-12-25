use std::fs::read_to_string;

use grid::Grid;
use regex::Regex;

fn main() {
    let test = read_input("./input/day_25.test.txt");
    let input = read_input("./input/day_25.txt");

    println!("Test answer: {:?}", fit_keys_and_locks(&test));
    println!("Answer: {:?}", fit_keys_and_locks(&input));
}

fn fit_keys_and_locks(input: &Vec<Grid<char>>) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for object in input {
        let is_a_lock = object.iter_row(0).all(|&c| c == '#');

        let as_height_map: Vec<usize> = object
            .iter_cols()
            .map(|v| v.filter(|&&c| c == '#').count() - 1)
            .collect();

        if is_a_lock {
            locks.push(as_height_map);
        } else {
            keys.push(as_height_map);
        }
    }

    let count_locks = locks.len();
    let count_keys = keys.len();
    (0..count_locks)
        .map(|i| (0..count_keys).map(move |j| (i, j)))
        .flatten()
        .filter(|(i, j)| (0..5).all(|k| locks[*i][k] + keys[*j][k] < 6))
        .count()
}

fn read_input(file_path: &str) -> Vec<Grid<char>> {
    let string = read_to_string(file_path).unwrap().trim().to_owned();
    let break_pattern = Regex::new(r"(\r?\n){2,}").unwrap();

    break_pattern
        .split(&string)
        .map(|pattern| {
            Grid::from_vec(
                Vec::from_iter(pattern.replace("\r", "").replace("\n", "").chars()),
                5,
            )
        })
        .collect()
}
