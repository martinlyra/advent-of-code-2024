use std::{fs::read_to_string, time::Instant};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

struct Robot {
    start: (i32, i32),
    velocity: (i32, i32),
}

fn main() {
    let test_area = (11, 7);
    let real_area = (101, 103);
    let test = read_input("./input/day_14.test.txt");
    let input = read_input("./input/day_14.txt");

    println!(
        "First part test answer: {}",
        evaluate(&test, test_area, 100)
    );
    println!("First part answer: {}", evaluate(&input, real_area, 100));

    let before = Instant::now();
    println!("Second part answer: {}", find_image(&input, real_area));
    println!("Work time: {:.2?}", before.elapsed());
}

fn find_image(input: &Vec<Robot>, boundaries: (i32, i32)) -> usize {
    *(0..10000)
        .par_bridge()
        .filter(|&i| simulate(input, boundaries, i).iter().unique().count() == input.len())
        .collect::<Vec<_>>()
        .first()
        .unwrap()
}

fn simulate(input: &Vec<Robot>, boundaries: (i32, i32), ticks: usize) -> Vec<(i32, i32)> {
    let (width, height) = boundaries;
    input
        .iter()
        .map(|robot| {
            let t = (
                robot.start.0 + (robot.velocity.0 * ticks as i32) % width,
                robot.start.1 + (robot.velocity.1 * ticks as i32) % height,
            );
            ((t.0 + width) % width, (t.1 + height) % height)
        })
        .collect()
}

fn evaluate(input: &Vec<Robot>, boundaries: (i32, i32), ticks: usize) -> usize {
    let (width, height) = boundaries;
    let (mid_w, mid_h) = ((width - 1) / 2, (height - 1) / 2);
    let quadrants = simulate(input, boundaries, ticks).iter().fold(
        (0, 0, 0, 0),
        |(q1, q2, q3, q4), &(x, y)| {
            if x < mid_w {
                if y < mid_h {
                    return (q1 + 1, q2, q3, q4);
                } else if mid_h < y {
                    return (q1, q2, q3 + 1, q4);
                }
            } else if mid_w < x {
                if y < mid_h {
                    return (q1, q2 + 1, q3, q4);
                } else if mid_h < y {
                    return (q1, q2, q3, q4 + 1);
                }
            }
            (q1, q2, q3, q4)
        },
    );
    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn read_input(file_path: &str) -> Vec<Robot> {
    let pattern = Regex::new(r"p=(\d+),(\d+) v=([\d\-]+),([\d\-]+)").unwrap();
    read_to_string(file_path)
        .unwrap()
        .trim()
        .split("\n")
        .map(|f| {
            pattern
                .captures(f)
                .unwrap()
                .extract::<4>()
                .1
                .map(|f| f.parse::<i32>().unwrap())
        })
        .map(|c| Robot {
            start: (c[0], c[1]),
            velocity: (c[2], c[3]),
        })
        .collect()
}
