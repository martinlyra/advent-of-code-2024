mod utilities;
use std::char;

use utilities::read_lines;

fn main() {
    let test_map: Vec<String>;
    let pattern: String = String::from("XMAS");
    match read_lines("./input/day_4.test.txt") {
        Ok(lines) => {
            test_map = lines.flatten().collect();
        }
        Err(e) => {
            panic!("Unable to read file! {e}");
        }
    }

    let test_answer = find_all(&pattern, &test_map);
    assert_eq!(test_answer, 18);

    let map: Vec<String>;
    match read_lines("./input/day_4.txt") {
        Ok(lines) => {
            map = lines.flatten().collect();
        }
        Err(e) => {
            panic!("Unable to read file! {e}");
        }
    }

    let first_answer = find_all(&pattern, &map);
    println!("First part answer: {}", first_answer);

    let second_test_answer = find_all_crosses(&test_map);
    assert_eq!(second_test_answer, 9);

    let second_answer = find_all_crosses(&map);
    println!("Second part answer: {}", second_answer);
}

fn map_get(map: &Vec<String>, x: i32, y: i32) -> char {
    match map.get(y as usize) {
        Some(row) => match row.chars().nth(x as usize) {
            Some(c) => return c,
            None => return ' ',
        },
        None => return ' ',
    }
}

fn generate_path<P>(length: i32, func: P) -> Vec<(i32, i32)>
where
    P: FnMut(i32) -> (i32, i32),
{
    return (0..length).into_iter().map(func).collect();
}

fn search_walk(map: &Vec<String>, pattern: &String, positions: &Vec<(i32, i32)>) -> bool {
    return positions
        .iter()
        .enumerate()
        .all(|(i, (x, y))| map_get(map, *x, *y) == pattern.chars().nth(i).unwrap());
}

fn search(map: &Vec<String>, pattern: &String, start_x: i32, start_y: i32) -> i32 {
    let length = pattern.len() as i32;
    let paths = &[
        generate_path(length, |i| (start_x + i, start_y)),
        generate_path(length, |i| (start_x - i, start_y)),
        generate_path(length, |i| (start_x + i, start_y + i)),
        generate_path(length, |i| (start_x + i, start_y - i)),
        generate_path(length, |i| (start_x - i, start_y + i)),
        generate_path(length, |i| (start_x - i, start_y - i)),
        generate_path(length, |i| (start_x, start_y + i)),
        generate_path(length, |i| (start_x, start_y - i)),
    ];
    return paths
        .into_iter()
        .filter(|p| search_walk(map, pattern, *p))
        .count() as i32;
}

fn search_cross(map: &Vec<String>, pattern: &String, center_x: i32, center_y: i32) -> bool {
    let length: i32 = pattern.len() as i32;
    let offset: i32 = (length - 1) / 2;
    let paths_left = &[
        generate_path(length, |i| (center_x - offset + i, center_y + offset - i)), // from lower left, up
        generate_path(length, |i| (center_x + offset - i, center_y - offset + i)), // from upper right, down
    ];
    let paths_right = &[
        generate_path(length, |i| (center_x - offset + i, center_y - offset + i)), // from upper left, down
        generate_path(length, |i| (center_x + offset - i, center_y + offset - i)), // from lower right, up
    ];
    paths_left
        .iter()
        .filter(|p| search_walk(map, pattern, p))
        .any(|_| paths_right.iter().any(|p| search_walk(map, pattern, p)))
}

fn find_all_crosses(map: &Vec<String>) -> i32 {
    let mut count = 0;
    let pattern = String::from("MAS");
    for y in 1..(map.len() - 1) {
        let row = &map[y];
        for x in 1..(row.len() - 1) {
            if search_cross(map, &pattern, x as i32, y as i32) {
                count += 1;
            }
        }
    }
    return count;
}

fn find_all(pattern: &String, map: &Vec<String>) -> i32 {
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            (0..row.len())
                .into_iter()
                .map(|j| search(map, pattern, j as i32, i as i32))
                .sum::<i32>()
        })
        .sum()
}
