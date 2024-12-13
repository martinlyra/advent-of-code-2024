use std::{collections::HashMap, time::Instant};

use cached::proc_macro::cached;

type Number = u64;

fn main() {
    let test = read_input("125 17");
    let input = read_input("28591 78 0 3159881 4254 524155 598 1");

    println!("First part test answer: {} == 55312", evaluate(&test, 25));
    let before = Instant::now();
    println!("First part answer: {}", evaluate(&input, 25));
    println!("Work time: {:.2?}", before.elapsed());

    let before = Instant::now();
    println!("Second part answer: {}", evaluate(&input, 75));
    println!("Work time: {:.2?}", before.elapsed());
}

fn evaluate(input: &Vec<Number>, iterations: usize) -> usize {
    (0..iterations)
        .into_iter()
        .fold(
            input
                .iter()
                .copied()
                .fold(HashMap::new(), |mut map, value| {
                    map.entry(value).and_modify(|c| *c += 1).or_insert(1);
                    map
                }),
            |stones, _| {
                stones
                    .iter()
                    .fold(HashMap::new(), |mut map, (number, count)| {
                        for i in blink(*number) {
                            map.entry(i).and_modify(|c| *c += count).or_insert(*count);
                        }
                        map
                    })
            },
        )
        .values()
        .sum()
}

fn blink(x: Number) -> Vec<Number> {
    match x {
        0 => vec![1],
        _ => {
            let n = digit_count(x);
            if n % 2 == 0 {
                let (a, b) = split_number(x);
                vec![a, b]
            } else {
                vec![x * 2024]
            }
        }
    }
}

fn split_number(x: Number) -> (Number, Number) {
    let n = Number::pow(10, (digit_count(x) / 2) as u32);
    (x / n, x % n)
}

fn digit_count(number: Number) -> usize {
    f64::log10(number as f64) as usize + 1
}

fn read_input(input: &str) -> Vec<Number> {
    input
        .trim()
        .split(" ")
        .map(|s| s.parse::<Number>().unwrap())
        .collect()
}
