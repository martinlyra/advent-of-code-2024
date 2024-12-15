use std::{fmt::Debug, fs::read_to_string};

use nalgebra::*;
use regex::Regex;

struct ClawMachine {
    offsets_a: (u64, u64),
    offsets_b: (u64, u64),
    reward_location: (u64, u64),
}

impl ClawMachine {
    fn solve(&self) -> Option<(u64, u64)> {
        let a = Matrix2::new(
            self.offsets_a.0 as f64,
            self.offsets_b.0 as f64,
            self.offsets_a.1 as f64,
            self.offsets_b.1 as f64,
        );
        let b = Vector2::new(self.reward_location.0 as f64, self.reward_location.1 as f64);
        match a.lu().solve(&b) {
            Some(solution) => {
                let error = f64::max(
                    ((solution[0].round()) as f64 - solution[0]).abs(),
                    ((solution[1].round()) as f64 - solution[1]).abs(),
                );
                return if error < 0.001 {
                    Some((solution[0].round() as u64, solution[1].round() as u64))
                } else {
                    None
                };
            }
            None => None,
        }
    }
}

impl Debug for ClawMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClawMachine")
            .field("offsets_a", &self.offsets_a)
            .field("offsets_b", &self.offsets_b)
            .field("reward_location", &self.reward_location)
            .finish()
    }
}

fn main() {
    let test = read_input("./input/day_13.test.txt");
    let input = read_input("./input/day_13.txt");
    println!("First part test answer: {} == 480", part_1(&test));
    println!("First part answer: {}", part_1(&input));
    println!("Second part answer: {}", part_2(&input));
}

fn part_1(input: &Vec<ClawMachine>) -> u64 {
    input
        .iter()
        .filter_map(|f| f.solve())
        .map(|f| {
            let (a, b) = f;
            a * 3 + b
        })
        .sum()
}

fn part_2(input: &Vec<ClawMachine>) -> u64 {
    input
        .iter()
        .map(|f| ClawMachine {
            offsets_a: f.offsets_a,
            offsets_b: f.offsets_b,
            reward_location: (
                10000000000000u64 + f.reward_location.0,
                10000000000000u64 + f.reward_location.1,
            ),
        })
        .filter_map(|f| f.solve())
        .map(|f| {
            let (a, b) = f;
            a * 3 + b
        })
        .sum()
}

fn read_input(file_path: &str) -> Vec<ClawMachine> {
    let pattern = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    pattern
        .captures_iter(read_to_string(file_path).unwrap().trim())
        .map(|c| {
            let ds: Vec<u64> = c
                .extract::<6>()
                .1
                .iter()
                .map(|d| d.parse::<u64>().unwrap())
                .collect();
            ClawMachine {
                offsets_a: (ds[0], ds[1]),
                offsets_b: (ds[2], ds[3]),
                reward_location: (ds[4], ds[5]),
            }
        })
        .collect()
}