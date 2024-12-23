use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    hash::{DefaultHasher, Hash, Hasher},
};

use rayon::prelude::*;

fn main() {
    let test: Vec<u64> = vec![1, 10, 100, 2024];
    let input = read_input("./input/day_22.txt");

    assert_eq!(15887950, find_secret(123, 1));
    assert_eq!(16495136, find_secret(123, 2));
    assert_eq!(527345, find_secret(123, 3));
    assert_eq!(704524, find_secret(123, 4));
    assert_eq!(1553684, find_secret(123, 5));

    println!("First part test answer: {} == 37327623", part_1(&test));
    println!("First part answer: {}", part_1(&input));

    println!("Second part test answer: {} == 24", part_2(&test));
    println!("Second part answer: {}", part_2(&input));
}

fn part_1(input: &Vec<u64>) -> u64 {
    input.par_iter().map(|x| find_secret(*x, 2000)).sum()
}

fn find_secret(x: u64, depth: usize) -> u64 {
    (0..depth).fold(x, |previous, _| next_secret_number(previous))
}

fn part_2(input: &Vec<u64>) -> u16 {
    *input
        .par_iter()
        .map(|x| record_price_changes(*x, 4, 2000))
        .collect::<Vec<_>>()
        .into_iter()
        .fold(HashMap::new(), |mut sum, seqs| {
            for key in seqs.keys() {
                let value = seqs[key];
                sum.entry(key.clone())
                    .and_modify(|v| *v += value)
                    .or_insert(value);
            }
            sum
        })
        .values()
        .max()
        .unwrap()
}

fn record_price_changes(
    initial_secret: u64,
    sequence_length: usize,
    depth: usize,
) -> HashMap<u64, u16> {
    let mut last_secret = initial_secret;
    let mut first_sequences: HashMap<u64, u16> = HashMap::new();
    let mut buffer: VecDeque<i8> = VecDeque::new();

    for _ in 0..depth {
        let new_secret = next_secret_number(last_secret);

        let last_price = (last_secret % 10) as i8;
        let new_price = (new_secret % 10) as i8;
        let diff = new_price - last_price;
        buffer.push_back(diff);
        last_secret = new_secret;

        if buffer.len() < sequence_length {
            continue;
        }
        if buffer.len() > sequence_length {
            buffer.pop_front();
        }

        let mut state = DefaultHasher::new();
        buffer.hash(&mut state);
        let key = state.finish();
        first_sequences.entry(key).or_insert(new_price as u16);
    }
    first_sequences
}

fn next_secret_number(secret: u64) -> u64 {
    let a = step(secret, secret * 64);
    let b = step(a, (a as f64 / 32f64).floor() as u64);
    step(b, b * 2048)
}

fn step(a: u64, b: u64) -> u64 {
    (a ^ b) % 16777216
}

fn read_input(file_path: &str) -> Vec<u64> {
    read_to_string(file_path)
        .unwrap()
        .trim()
        .split("\n")
        .map(|p| p.parse::<u64>().unwrap())
        .collect()
}
