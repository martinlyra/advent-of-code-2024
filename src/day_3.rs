mod utilities;
use utilities::read_lines;

use regex::Regex;

fn main() {
    let buffer: String;
    match read_lines("./input/day_3.txt") {
        Ok(lines) => {
            buffer = lines.flatten().reduce(|acc, e| acc + &e).unwrap();
        }
        Err(e) => {
            panic!("Unable to read file! {e}");
        }
    }
    let mut products: Vec<i32> = Vec::new();
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (_, [term_a, term_b]) in mul_pattern.captures_iter(&buffer).map(|c| c.extract()) {
        products.push(term_a.parse::<i32>().unwrap() * term_b.parse::<i32>().unwrap());
    }
    let part_1_sum: i32 = products.iter().sum();
    println!("First part answer: {part_1_sum}");

    let program_pattern = Regex::new(&format!(
        "({}|{}|{})",
        r"do\(\)",
        r"don't\(\)",
        mul_pattern.as_str()
    ))
    .unwrap();
    products.clear();
    let mut mul_enabled = true;
    for token in program_pattern.find_iter(&buffer) {
        let instruction = token.as_str();
        println!("{}", instruction);
        match instruction {
            "do()" => {
                mul_enabled = true;
            }
            "don't()" => {
                mul_enabled = false;
            }
            _ => {
                if mul_enabled {
                    match mul_pattern.captures(instruction).map(|c| c.extract()) {
                        Some((_, [term_a, term_b])) => {
                            products.push(
                                term_a.parse::<i32>().unwrap() * term_b.parse::<i32>().unwrap(),
                            );
                        }
                        None => {}
                    }
                }
            }
        }
    }
    let part_2_sum: i32 = products.iter().sum();
    println!("First part answer: {part_2_sum}");
}
