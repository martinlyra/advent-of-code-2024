use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    time::Instant,
    usize,
};

type Position = (usize, usize);

struct GardenRegion {
    label: char,
    positions: Vec<Position>,
}

impl GardenRegion {
    fn area(&self) -> usize {
        self.positions.len()
    }

    fn perimeter(&self) -> usize {
        self.positions
            .iter()
            .map(|p| plus_neighbors(p))
            .flatten()
            .filter(|&(i, j)| !self.positions.contains(&(i as usize, j as usize)))
            .count()
    }

    fn corners(&self) -> usize {
        let bmap = self.to_padded_bool_map();
        (0..(bmap.len() - 1))
            .flat_map(|j| (0..(bmap[0].len() - 1)).map(move |i| (i, j)))
            .map(|(i, j)| {
                let (a, b) = (bmap[j][i] as u8, bmap[j][i + 1] as u8);
                let (c, d) = (bmap[j + 1][i] as u8, bmap[j + 1][i + 1] as u8);

                match a + b + c + d {
                    1 => Some(1),
                    3 => Some(1),
                    2 => {
                        if (a == d) || (b == c) {
                            Some(2)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
            .flatten()
            .sum()
    }

    fn to_padded_bool_map(&self) -> Vec<Vec<bool>> {
        let ((xmin, ymin), (xmax, ymax)) = self.get_boundaries();
        let mut map = vec![vec![false; (xmax - xmin) + 3]; (ymax - ymin) + 3];
        for &(x, y) in &self.positions {
            map[y - ymin + 1][x - xmin + 1] = true;
        }
        map
    }

    fn get_boundaries(&self) -> ((usize, usize), (usize, usize)) {
        let (mut xmin, mut ymin) = (usize::MAX, usize::MAX);
        let (mut xmax, mut ymax) = (usize::MIN, usize::MIN);
        for &(x, y) in &self.positions {
            if x < xmin {
                xmin = x;
            }
            if y < ymin {
                ymin = y;
            }
            if x > xmax {
                xmax = x;
            }
            if y > ymax {
                ymax = y;
            }
        }
        ((xmin, ymin), (xmax, ymax))
    }
}

fn main() {
    let before = Instant::now();
    let example_1 = read_input("./input/day_12.test.ex1.txt");
    let example_2 = read_input("./input/day_12.test.ex2.txt");
    let test = read_input("./input/day_12.test.txt");
    let input = read_input("./input/day_12.txt");
    println!("Load time, all: {:.2?}\n", before.elapsed());

    println!(
        "First part example #1 answer: {} == 140",
        part_1(&example_1)
    );
    println!(
        "First part example #2 answer: {} == 772",
        part_1(&example_2)
    );
    println!("First part test answer: {} == 1930", part_1(&test));
    let before = Instant::now();
    println!("First part answer: {}", part_1(&input));
    println!("Work time: {:.2?}\n", before.elapsed());

    println!(
        "Second part example #1 answer: {} == 80",
        part_2(&example_1)
    );
    println!("Second part test answer: {} == 1206", part_2(&test));
    let before = Instant::now();
    println!("Second part answer: {}", part_2(&input));
    println!("Work time: {:.2?}", before.elapsed());
}

fn part_1(gardens: &Vec<GardenRegion>) -> usize {
    gardens.iter().map(|g| g.area() * g.perimeter()).sum()
}

fn part_2(gardens: &Vec<GardenRegion>) -> usize {
    gardens.iter().map(|g| g.area() * g.corners()).sum()
}

fn plus_neighbors(position: &Position) -> Vec<(i32, i32)> {
    let (x, y) = (position.0 as i32, position.1 as i32);
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn neighbors(input: &Vec<Vec<char>>, position: Position) -> Vec<Position> {
    let (x, y) = position;
    let label = input[y][x];
    let mut result = Vec::new();
    for (i, j) in plus_neighbors(&position) {
        if let Some(row) = input.get(j as usize) {
            if let Some(other_label) = row.get(i as usize) {
                if *other_label == label {
                    result.push((i as usize, j as usize));
                }
            }
        }
    }
    result
}

fn read_input(file_path: &str) -> Vec<GardenRegion> {
    let plot: Vec<Vec<char>> = read_to_string(file_path)
        .unwrap()
        .trim()
        .split("\n")
        .map(|row| row.trim().chars().map(|c| c).collect())
        .collect();

    let mut areas: Vec<GardenRegion> = Vec::new();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut queue: VecDeque<Position> = VecDeque::new();
    for position in plot
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
        .flatten()
    {
        if visited.contains(&position) {
            continue;
        }

        queue.clear();
        queue.push_back(position);

        let mut new_area = GardenRegion {
            label: plot[position.1][position.0],
            positions: Vec::new(),
        };
        while let Some(next) = queue.pop_front() {
            if visited.contains(&next) {
                continue;
            }
            for neighbor in neighbors(&plot, next) {
                queue.push_back(neighbor);
            }
            new_area.positions.push(next);
            visited.insert(next);
        }
        areas.push(new_area);
    }

    areas
}
