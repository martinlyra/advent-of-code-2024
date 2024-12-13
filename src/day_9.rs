use std::{fs::read_to_string, num::IntErrorKind};

use itertools::Itertools;

fn main() {
    let test = read_disk_map("2333133121414131402");
    println!(
        "First part test answer: {} == 1928",
        consume_disk_map_1(&test)
    );
    println!(
        "Second part test answer: {} == 2858",
        consume_disk_map_2(&test)
    );

    let input_string = String::from(read_to_string("./input/day_9.txt").unwrap().trim());
    let input = read_disk_map(&input_string);
    println!("First part answer: {}", consume_disk_map_1(&input));
    println!("Second part answer: {}", consume_disk_map_2(&input));
}

fn consume_disk_map_1(input: &Vec<Option<u64>>) -> u64 {
    let mut a: usize = 0;
    let mut b: usize = input.len() - 1;
    let mut buffer = input.clone();
    while a != b {
        match buffer[a] {
            Some(_) => a += 1,
            None => {
                match buffer[b] {
                    Some(_) => {
                        buffer.swap(a, b);
                    }
                    None => {}
                }
                b -= 1;
            }
        }
    }
    buffer
        .iter()
        .filter_map(|i| *i)
        .enumerate()
        .map(|(i, j)| i as u64 * j)
        .sum()
}

fn block_size(input: &Vec<Option<u64>>, start: usize, step: i32, element: Option<u64>) -> usize {
    let mut i: usize = start;
    while input[i] == element {
        i = (i as i32 + step) as usize;
    }
    return (i.abs_diff(start)) as usize;
}

fn find_free_space(input: &Vec<Option<u64>>, size: usize, stop: usize) -> Option<usize> {
    let mut i = 0;
    while i < stop {
        match input[i] {
            None => {
                let possible = block_size(input, i, 1, None);
                if possible >= size {
                    return Some(i);
                } 
                i += possible;
            }
            Some(_) => {i += 1}
        }
    }
    return None
}

fn consume_disk_map_2(input: &Vec<Option<u64>>) -> u64 {
    let mut i: usize = input.len() - 1;
    let mut k: usize = 0;
    let mut buffer = input.clone();
    while i > k {
        match buffer[i] {
            None => { i -= 1; },
            Some(a) => {
                let size_a = block_size(&buffer, i, -1, Some(a));
                match find_free_space(&buffer, size_a, i) {
                    Some(j) => {
                        for k in 0..size_a {
                            buffer.swap(j + k, i - k);
                        }
                        k = find_free_space(input, 0, i).unwrap();
                    }
                    None => {
                    }
                }
                i -= size_a;
            }
        }
    }
    buffer
        .iter()
        .enumerate()
        .filter_map(|(i, j)| match j {
            Some(k) => Some(i as u64 * *k),
            None => None,
        } )
        .sum()
}

fn read_disk_map(map_string: &str) -> Vec<Option<u64>> {
    map_string
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let (q, r) = (i / 2, i % 2);
            let count = c.to_digit(10).unwrap() as usize;
            if r == 0 {
                (Some(q as u64), count)
            } else {
                (None, count)
            }
        })
        .map(|(id, count)| vec![id; count])
        .flatten()
        .collect()
}

fn debug_map(input: &Vec<Option<u64>>) {
    println!("{}", input.iter().map(|a| match a {
        None => String::from("."),
        Some(i) => i.to_string()
    }).join(""));
}
