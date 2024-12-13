mod utilities;
use utilities::read_lines;

fn main() {
    let mut num_as: Vec<i32> = Vec::new();
    let mut num_bs: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./input/day_1.txt") {
        for line in lines.flatten() {
            if let Some((str_a, str_b)) = line.split_once("   ") {
                num_as.push(str_a.parse::<i32>().unwrap());
                num_bs.push(str_b.parse::<i32>().unwrap());
            }
        }
    }
    println!("List 1 length: {}", num_as.len());
    println!("List 2 length: {}", num_bs.len());
    
    num_as.sort();
    num_bs.sort();
    let mut distances: Vec<i32> = Vec::new();
    for i in 0..num_as.len() {
        distances.push(i32::abs(num_as[i] - num_bs[i]));
    }
    println!("Part A answer: {}", distances.iter().sum::<i32>());

    let mut scores: Vec<i32> = Vec::new();
    for i in 0..num_as.len() {
        let a = num_as[i];
        let c = num_bs.iter().filter(|&b| *b == a).count() as i32;
        scores.push(a * c);
    }
    println!("Part B answer: {}", scores.iter().sum::<i32>());
}
