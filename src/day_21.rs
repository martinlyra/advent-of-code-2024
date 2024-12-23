use std::{collections::HashMap, fs::read_to_string};

use cached::proc_macro::cached;
use cached::SizedCache;
use itertools::Itertools;
use lazy_static::lazy_static;

type Keypad = HashMap<char, (i32, i32)>;

lazy_static! {
    static ref NUMPAD: Keypad = { HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        //
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        //
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        //
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]) };
    static ref DIRPAD: Keypad = { HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        //
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
        ]) };
}

fn main() {
    let test = read_input("./input/day_21.test.txt");
    let input = read_input("./input/day_21.txt");

    println!("First part test answer: {} == 126384", part_1(&test));
    println!("First part answer: {}", part_1(&input));

    println!("Second part answer: {}", part_2(&input));
}

fn part_1(codes: &Vec<Vec<char>>) -> usize {
    eval(codes, 2)
}

fn part_2(codes: &Vec<Vec<char>>) -> usize {
    eval(codes, 25)
}

fn eval(codes: &Vec<Vec<char>>, depth: usize) -> usize {
    let sequences = codes
        .iter()
        .map(|code| min_input_path(code, depth + 1, &NUMPAD))
        .zip(codes)
        .map(|(seq, code)| {
            //println!("{:?}", seq.iter().collect::<String>());
            (
                code.iter()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
                seq,
            )
        })
        .collect::<Vec<_>>();
    sequences.iter().map(|(num, len)| num * len).sum::<usize>()
}

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(256) }",
    convert = r#"{ format!("{:?}{}{:p}", sequence, depth, input) }"#
)]
fn min_input_path(sequence: &Vec<char>, depth: usize, input: &Keypad) -> usize {
    if depth > 0 {
        let mut previous = 'A';
        let mut pieces = Vec::new();
        for &c in sequence {
            let paths = input_paths(previous, c, &input);
            let best = paths
                .iter()
                .map(|path| {
                    min_input_path(
                        &path.into_iter().chain(vec!['A'].iter()).cloned().collect(),
                        depth - 1,
                        &DIRPAD,
                    )
                })
                .min()
                .unwrap();
            pieces.push(best);
            previous = c;
        }

        return pieces.into_iter().sum();
    }
    return sequence.len();
}

#[cached(
    ty = "SizedCache<String, Vec<Vec<char>>>",
    create = "{ SizedCache::with_size(256) }",
    convert = r#"{ format!("{}{}{:p}", from, to, input) }"#
)]
fn input_paths(from: char, to: char, input: &Keypad) -> Vec<Vec<char>> {
    if from == to {
        return vec![vec![]];
    }

    let a = input[&from];
    let b = input[&to];

    let (dx, dy) = (b.0 - a.0, b.1 - a.1);

    let moves_x = (0..dx.abs())
        .map(|_| match dx.signum() {
            1 => '>',
            -1 => '<',
            _ => panic!("Invalid dx: {dx} ({})", dx.abs()),
        })
        .collect::<Vec<_>>();
    let moves_y = (0..dy.abs())
        .map(|_| match dy.signum() {
            1 => 'v',
            -1 => '^',
            _ => panic!("Invalid dx: {dy} ({})", dy.abs()),
        })
        .collect::<Vec<_>>();

    return if dx == 0 {
        vec![moves_y]
    } else if dy == 0 {
        vec![moves_x]
    } else if !input.values().contains(&(a.0, b.1)) {
        vec![vec_concat(&moves_x, &moves_y)]
    } else if !input.values().contains(&(b.0, a.1)) {
        vec![vec_concat(&moves_y, &moves_x)]
    } else {
        vec![
            vec_concat(&moves_x, &moves_y),
            vec_concat(&moves_y, &moves_x),
        ]
    };
}

fn vec_concat<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
where
    T: Copy,
{
    a.into_iter()
        .chain(b.into_iter())
        .copied()
        .collect::<Vec<T>>()
}

fn read_input(file_path: &str) -> Vec<Vec<char>> {
    read_to_string(file_path)
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| s.chars().collect())
        .collect()
}
