use std::fs::read_to_string;

use cached::proc_macro::cached;
use cached::SizedCache;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let test = read_input("./input/day_19.test.txt");
    let input = read_input("./input/day_19.txt");

    println!("First part test answer: {} == 6", part_1(&test.1, &test.0));
    println!("First part answer: {}", part_1(&input.1, &input.0));

    println!("Second part test answer: {} == 19", part_2(&test.1, &test.0));
    println!("Second part answer: {}", part_2(&input.1, &input.0));
}

fn part_1(input: &Vec<String>, materials: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|&order| can_make(order, materials) > 0)
        .count()
}

fn part_2(input: &Vec<String>, materials: &Vec<String>) -> usize {
    input.iter().map(|order| can_make(order, materials)).sum()
}

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(256) }",
    convert = r#"{ format!("{}", order) }"#,
)]
fn can_make(order: &str, materials: &Vec<String>) -> usize {
    if order.len() < 1 {
        return 1;
    }
    materials.iter().filter_map(
        |pattern| {
            match order.starts_with(pattern) {
                false => None,
                true => Some(can_make(&order[pattern.len()..], materials))
            }
        }
    ).sum()
}

fn read_input(file_path: &str) -> (Vec<String>, Vec<String>) {
    let break_pattern = Regex::new(r"(\r?\n){2,}").unwrap();
    let string = read_to_string(file_path).unwrap();
    let parts: Vec<&str> = break_pattern.split(&string).collect();

    (
        parts[0]
            .trim()
            .split(", ")
            .map(|s| s.to_string())
            .sorted_by(|a, b| Ord::cmp(&b.len(), &a.len()))
            .collect(),
        parts[1]
            .trim()
            .split("\n")
            .map(|s| s.trim().to_string())
            .collect(),
    )
}
