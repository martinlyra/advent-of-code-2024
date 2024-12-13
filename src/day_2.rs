mod utilities;
use utilities::read_lines;

fn main() {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("./input/day_2.txt") {
        for line in lines.flatten() {
            if line.trim().is_empty() {
                continue;
            }
            let numbers = line
                .split(" ")
                .map(|x| str::parse::<i32>(x).unwrap())
                .collect::<Vec<i32>>();
            reports.push(numbers);
        }
    }
    println!("List 1 length: {}", reports.len());

    let mut safe_reports_1: usize = 0;
    let mut unsafe_reports: Vec<&Vec<i32>> = Vec::new();
    for i in 0..reports.len() {
        let report = &reports[i];
        if is_safe(&report) {
            safe_reports_1 += 1;
        } else {
            unsafe_reports.push(report);
        }
    }

    let mut safe_reports_2: usize = 0;
    for i in 0..unsafe_reports.len() {
        let report = &reports[i];
        for j in 0..report.len() {
            let mut modified = report.clone();
            modified.remove(j);
            if is_safe(&modified) {
                safe_reports_2 += 1;
                break;
            }
        }
    }

    println!("First part answer: {}", safe_reports_1);
    println!("Second part answer: {}", safe_reports_1 + safe_reports_2);
}

fn is_safe(numbers: &Vec<i32>) -> bool 
{
    let count = numbers.len() - 1;
    let mut deltas: Vec<i32> = Vec::with_capacity(count);
    for j in 0..count {
        deltas.push(numbers[j + 1] - numbers[j]);
    }

    let problems = (0..deltas.len())
        .into_iter()
        .map(|i| {
            !(1 <= deltas[i].abs() && deltas[i].abs() <= 3 && deltas[i].signum() == deltas[0].signum())
        })
        .filter(|x| *x)
        .count();
    return problems == 0;
}
