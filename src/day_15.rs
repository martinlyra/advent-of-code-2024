use std::fs::read_to_string;

use itertools::Itertools;
use regex::Regex;

enum Instruction {
    Left,
    Right,
    Up,
    Down,
}

fn main() {
    let small_1 = read_input("./input/day_15.test.small_1.txt");
    let small_2 = read_input("./input/day_15.test.small_2.txt");
    let test = read_input("./input/day_15.test.txt");
    let input = read_input("./input/day_15.txt");

    println!(
        "First part small test answer: {} == 2028",
        part_1(&small_1.0, &small_1.1)
    );
    println!(
        "First part test answer: {} == 10092",
        part_1(&test.0, &test.1)
    );
    println!("First part answer: {}", part_1(&input.0, &input.1));

    println!(
        "Second part small test answer: {}",
        part_2(&small_2.0, &small_2.1)
    );
    println!(
        "Second part test answer: {} == 9021",
        part_2(&test.0, &test.1)
    );
    println!("Second part answer: {}", part_2(&input.0, &input.1));
}

fn part_1(map: &Vec<Vec<char>>, instuctions: &Vec<Instruction>) -> usize {
    run(map, instuctions)
        .iter()
        .enumerate()
        .map(|(j, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    'O' => Some((100 * j + i) as usize),
                    _ => None,
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_2(map: &Vec<Vec<char>>, instuctions: &Vec<Instruction>) -> usize {
    let modified: Vec<Vec<char>> = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match c {
                    '#' => "##",
                    '.' => "..",
                    'O' => "[]",
                    '@' => "@.",
                    _ => "",
                })
                .join("")
                .chars()
                .collect()
        })
        .collect();

    run(&modified, instuctions)
        .iter()
        .enumerate()
        .map(|(j, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    '[' => Some((100 * j + i) as usize),
                    _ => None,
                })
                .sum::<usize>()
        })
        .sum()
}

fn run(map: &Vec<Vec<char>>, instuctions: &Vec<Instruction>) -> Vec<Vec<char>> {
    let start = map
        .iter()
        .enumerate()
        .filter_map(|(j, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(i, c)| match *c {
                    '@' => Some((i, j)),
                    _ => None,
                })
                .next()
        })
        .next()
        .unwrap();
    let mut current_pos = start;
    let mut current_map = map.clone();
    for instuction in instuctions {
        let (x, y) = current_pos;
        let (dx, dy) = match instuction {
            Instruction::Left => (-1, 0),
            Instruction::Right => (1, 0),
            Instruction::Up => (0, -1),
            Instruction::Down => (0, 1),
        };

        if dy != 0 {
            if can_vertical_push(&current_map, current_pos, dy) {
                push_vertical(&mut current_map, current_pos, dy);
                current_pos = (x, (y as i32 + dy) as usize)
            }
        } else {
            if can_horizontial_push(&current_map, current_pos, dx) {
                push_horizontial(&mut current_map, current_pos, dx);
                current_pos = ((x as i32 + dx) as usize, y)
            }
        }
    }
    current_map.clone()
}

fn push_horizontial(map: &mut Vec<Vec<char>>, position: (usize, usize), direction: i32) {
    let new_pos = ((position.0 as i32 + direction) as usize, position.1);
    match map[new_pos.1][new_pos.0] {
        'O' => push_horizontial(map, new_pos, direction),
        '[' => push_horizontial(map, new_pos, direction),
        ']' => push_horizontial(map, new_pos, direction),
        _ => (),
    };
    map[new_pos.1][new_pos.0] = map[position.1][position.0];
    map[position.1][position.0] = '.';
}

fn push_vertical(map: &mut Vec<Vec<char>>, position: (usize, usize), direction: i32) {
    let new_pos = (position.0, (position.1 as i32 + direction) as usize);
    match map[new_pos.1][new_pos.0] {
        'O' => push_vertical(map, new_pos, direction),
        '[' => {
            push_vertical(map, new_pos, direction);
            push_vertical(map, (new_pos.0 + 1, new_pos.1), direction);
        }
        ']' => {
            push_vertical(map, new_pos, direction);
            push_vertical(map, (new_pos.0 - 1, new_pos.1), direction);
        }
        _ => (),
    }
    map[new_pos.1][new_pos.0] = map[position.1][position.0];
    map[position.1][position.0] = '.';
}

fn can_vertical_push(map: &Vec<Vec<char>>, position: (usize, usize), direction: i32) -> bool {
    let (x, y) = position;
    let ny = (y as i32 + direction) as usize;
    match map[ny][x] {
        '#' => false,
        '.' => true,
        'O' => can_vertical_push(map, (x, ny), direction),
        '[' => {
            can_vertical_push(map, (x, ny), direction)
                && can_vertical_push(map, (x + 1, ny), direction)
        }
        ']' => {
            can_vertical_push(map, (x - 1, ny), direction)
                && can_vertical_push(map, (x, ny), direction)
        }
        c => panic!("Unknown tile: {c} {}", c as i32),
    }
}

fn can_horizontial_push(map: &Vec<Vec<char>>, position: (usize, usize), direction: i32) -> bool {
    let (x, y) = position;
    let nx = (x as i32 + direction) as usize;
    match map[y][nx] {
        '#' => false,
        '.' => true,
        _ => can_horizontial_push(map, (nx, y), direction),
    }
}

fn read_input(file_path: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let break_pattern = Regex::new(r"(\r?\n){2,}").unwrap();
    let string = read_to_string(file_path).unwrap();
    let parts: Vec<&str> = break_pattern.split(&string).collect();

    (
        String::from(parts[0])
            .split("\n")
            .map(|s| s.chars().collect())
            .collect(),
        String::from(parts[1])
            .split("\n")
            .join("")
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Instruction::Up),
                'v' => Some(Instruction::Down),
                '<' => Some(Instruction::Left),
                '>' => Some(Instruction::Right),
                _ => None,
            })
            .collect(),
    )
}
