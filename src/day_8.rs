use itertools::Itertools;
use std::fs::read_to_string;

type RadioAntenna = (char, i32, i32);

fn main() {
    let test = read_input("./input/day_8.test.txt");
    let input = read_input("./input/day_8.txt");

    println!("First part test value: {} == 14", part_1(&test.1, test.0));
    println!("Second part test value: {} == 34", part_2(&test.1, test.0));
    println!("First part value: {}", part_1(&input.1, input.0));
    println!("Second part value: {}", part_2(&input.1, input.0));
}

fn eval<F: Fn(RadioAntenna, RadioAntenna) -> Option<Vec<(i32, i32)>>>(
    input: &Vec<RadioAntenna>,
    boundaries: (usize, usize),
    function: &F,
) -> i32 {
    let antinodes: Vec<(i32, i32)> = input
        .iter()
        .enumerate()
        .map(|(i, &point)| {
            input[i..]
                .iter()
                .filter_map(move |&other| {
                    if point.0 == other.0 {
                        function(point, other)
                    } else {
                        None
                    }
                })
                .flatten()
        })
        .flatten()
        .filter(|&(x, y)| 0 <= x && x < boundaries.0 as i32 && 0 <= y && y < boundaries.1 as i32)
        .unique()
        .collect();
    antinodes.len() as i32
}

fn part_1(input: &Vec<RadioAntenna>, boundaries: (usize, usize)) -> i32 {
    eval(input, boundaries, &|(_, x, y), (_, ox, oy)| {
        let (dx, dy) = (ox - x, oy - y);
        if (dx.abs() + dy.abs()) < 1 {
            return None;
        }
        let points = vec![(x - dx, y - dy), (ox + dx, oy + dy)];
        return Some(points);
    })
}

fn part_2(input: &Vec<RadioAntenna>, boundaries: (usize, usize)) -> i32 {
    eval(input, boundaries, &|(_, x, y), (_, ox, oy)| {
        let (dx, dy) = (ox - x, oy - y);
        if (dx.abs() + dy.abs()) < 1 {
            return None;
        }
        let antinodes = (0..boundaries.0 as i32)
            .into_iter()
            .map(|i| vec![(x - dx * i, y - dy * i), (ox + dx * i, oy + dy * i)])
            .flatten()
            .collect();
        Some(antinodes)
    })
}

fn read_input(file_path: &str) -> ((usize, usize), Vec<RadioAntenna>) {
    let loaded: Vec<String> = read_to_string(file_path)
        .unwrap()
        .split("\n")
        .map(|s| String::from(s.trim()))
        .collect();
    let (width, height) = (loaded[0].len(), loaded.len());
    (
        (width, height),
        loaded
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(x, char)| match char {
                        '.' => None,
                        _ => Some((char, x as i32, y as i32)),
                    })
            })
            .flatten()
            .collect(),
    )
}

fn debug_map(points: &Vec<(i32, i32)>, boundaries: (usize, usize)) {
    let mut map: Vec<String> = (0..boundaries.1)
        .into_iter()
        .map(|_| (0..boundaries.0).into_iter().map(|_| '.').join(""))
        .collect();
    println!("{:?}", map);
    for &(x, y) in points {
        let i: usize = x as usize;
        map[y as usize].replace_range(i..i + 1, "X");
    }
    println!("{}", map.join("\n"));
}
