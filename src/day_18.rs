use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

use grid::Grid;

struct Path {
    path: Vec<(usize, usize)>,
}

fn main() {
    let test = read_input("./input/day_18.test.txt");
    let input = read_input("./input/day_18.txt");

    println!(
        "First part test answer: {} == 22",
        part_1((7, 7), 12, &test)
    );
    println!("First part answer: {}", part_1((71, 71), 1024, &input),);

    println!(
        "Second part test answer: {:?} == (6, 1)",
        part_2((7, 7), 12, &test),
    );
    println!("Second part answer: {:?}", part_2((71, 71), 1024, &input),);
}

fn part_1(boundaries: (usize, usize), size: usize, input: &Vec<(usize, usize)>) -> usize {
    get_path(boundaries, size, input)
        .iter()
        .map(|p| p.path.len() - 2)
        .next()
        .unwrap()
}

fn part_2(boundaries: (usize, usize), size: usize, input: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut new_size = size + 1;
    while let Some(valid_path) = get_path(boundaries, new_size, input) {
        while !valid_path.path.contains(&input[new_size - 1]) {
            new_size += 1;
        }
    }
    input[new_size - 1]
}

fn get_path(boundaries: (usize, usize), size: usize, input: &Vec<(usize, usize)>) -> Option<Path> {
    let mut map: Grid<char> = Grid::init(boundaries.0, boundaries.1, '.');
    for &(x, y) in input[..size].iter() {
        map[(y, x)] = '#';
    }
    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let end = (boundaries.0 - 1, boundaries.1 - 1);

    let mut queue = VecDeque::from(vec![Path { path: vec![(0, 0)] }]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while let Some(next) = queue.pop_front() {
        let &(x, y) = next.path.iter().last().unwrap();

        if (x, y) == end {
            return Some(Path {
                path: next.path.clone().into_iter().chain([end]).collect(),
            });
        }
        for (dx, dy) in directions {
            let (i, j): (usize, usize) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

            match map.get(j, i) {
                Some(c) => match c {
                    '#' => {}
                    _ => {
                        if !visited.contains(&(i, j)) {
                            visited.insert((i, j));
                            queue.push_back(Path {
                                path: next.path.clone().into_iter().chain([(i, j)]).collect(),
                            });
                        }
                    }
                },
                None => {}
            }
        }
    }
    None
}

fn read_input(file_path: &str) -> Vec<(usize, usize)> {
    read_to_string(file_path)
        .unwrap()
        .split("\n")
        .filter_map(|f| {
            if f.len() < 1 {
                return None;
            }
            let mut s = f.trim().split(",").map(|s| s.parse::<usize>().unwrap());
            Some((s.next().unwrap(), s.next().unwrap()))
        })
        .collect()
}
