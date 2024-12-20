mod day_16;

use std::collections::HashMap;

use cached::SizedCache;
use cached::proc_macro::cached;
use day_16::{find_paths, read_input, Path};
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let test = read_input("./input/day_20.test.txt");
    let input = read_input("./input/day_20.txt");

    let test_paths = find_paths(&test);
    let test_path = test_paths.iter().next().unwrap();

    let map = part_1(&test_path);
    println!("First part test answer:",);
    for key in map.keys().sorted() {
        println!("- {key}: {}", map[key]);
    }

    let input_paths = find_paths(&input);
    let input_path = input_paths.iter().next().unwrap();
    let map = part_1(&input_path);
    println!(
        "First part answer: {}",
        map.iter()
            .filter_map(|(k, v)| {
                if *k >= 100 {
                    return Some(v);
                }
                None
            })
            .sum::<i32>()
    )
}

#[cached(
    ty = "SizedCache<String, Option<usize>>",
    create = "{ SizedCache::with_size(256) }",
    convert = r#"{ format!("{:p}{}{}", path, position.0, position.1) }"#,
)]
fn find_index_of(path: &Vec<(usize, usize)>, position: (usize, usize)) -> Option<usize> {
    path.iter().position(|&p| p == position)
}

fn find_shortcuts(path: &Path) -> Vec<(usize, usize)> {
    path.visited
        .par_iter()
        .enumerate()
        .map(|(i, &p)| {
            let tail = path.visited[i..].to_vec();
            let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

            directions
                .iter()
                .filter_map(|(dx, dy)| {
                    let (x, y) = (
                        ((p.0 as i32) + (2 * dx)) as usize,
                        ((p.1 as i32) + (2 * dy)) as usize,
                    );

                    tail.iter().position(|&p| p == (x, y)).map(|j| (i, i + j))
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn part_1(path: &Path) -> HashMap<usize, i32> {
    find_shortcuts(path)
        .iter()
        .map(|(i, j)| j - i - 2)
        .filter(|&x| x > 0)
        .fold(HashMap::new(), |mut map, f| {
            map.entry(f).and_modify(|c| *c += 1).or_insert(1);
            map
        })
}
