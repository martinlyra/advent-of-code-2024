use std::{fs::read_to_string, time::Instant};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

type Number = i64;

fn main() {
    let test = read_input("./input/day_7.test.txt");
    let input = read_input("./input/day_7.txt");

    println!("First part test value: {} == 3749", part_1(&test));
    println!("Second part test value: {} == 11387", part_2(&test));
    
    let before = Instant::now();
    println!("First part value: {}", part_1(&input));
    println!("Second part value: {}", part_2(&input));
    println!("Work time: {:.2?}", before.elapsed());
}

fn part_1(input: &Vec<(Number, Vec<Number>)>) -> Number {
    evaluate(input, &[inv_add, inv_mul])
}

fn part_2(input: &Vec<(Number, Vec<Number>)>) -> Number {
    evaluate(input, &[inv_add, inv_mul, inv_concat])
}

fn evaluate(
    input: &Vec<(Number, Vec<Number>)>,
    operators: &[fn(Number, Number) -> Option<Number>],
) -> Number {
    input
        .par_iter()
        .filter_map(|(g, ns)| {
            let numbers = ns.iter().rev().cloned().collect_vec();
            return match is_traceable(*g, &numbers, operators) {
                true => Some(*g),
                false => None,
            };
        })
        .sum()
}

fn digit_count(number: Number) -> u32 {
    f64::log10(number as f64) as u32 + 1
}

fn unconcat(a: Number, b: Number) -> (Number, Number) {
    let (c, d) = (a - b, Number::pow(10, digit_count(b)));
    (c / d, c % d)
}

fn inv_mul(a: Number, b: Number) -> Option<Number> {
    let (q, r) = (a / b, a % b);
    return if r == 0 { Some(q) } else { None };
}

fn inv_add(a: Number, b: Number) -> Option<Number> {
    let d = a - b;
    return if d < 0 { None } else { Some(d) };
}

fn inv_concat(a: Number, b: Number) -> Option<Number> {
    let (cq, cr) = unconcat(a, b);
    return if cr == 0 { Some(cq) } else { None };
}

fn is_traceable(
    number: Number,
    numbers: &[Number],
    operators: &[fn(Number, Number) -> Option<Number>],
) -> bool {
    let (head, tail) = (numbers[0], &numbers[1..]);
    if tail.len() < 1 {
        return head == number;
    }
    operators
        .iter()
        .filter_map(|function| function(number, head))
        .any(|new_number| is_traceable(new_number, tail, operators))
}

fn read_input(file_path: &str) -> Vec<(Number, Vec<Number>)> {
    let input_pattern = Regex::new(r"(\d+): ((?:\d+\s?)+)").unwrap();
    read_to_string(file_path)
        .unwrap()
        .trim()
        .split("\n")
        .par_bridge()
        .map(|s| {
            let (_, [first, second]) = input_pattern.captures(s).unwrap().extract();
            (
                first.parse::<Number>().unwrap(),
                second
                    .split(" ")
                    .map(|p| p.trim().parse::<Number>().unwrap())
                    .collect(),
            )
        })
        .collect()
}
