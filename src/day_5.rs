use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

fn main() {
    let (test_rules, test_chains) = parse_input("./input/day_5.test.txt");

    println!(
        "First part test answer: {} == 143",
        part_1(&test_rules, &test_chains)
    );

    let (rules, chains) = parse_input("./input/day_5.txt");

    println!("First part answer: {}", part_1(&rules, &chains));

    println!(
        "Second part test answer: {} == 123",
        part_2(&test_rules, &test_chains)
    );

    println!("Second part answer: {}", part_2(&rules, &chains))
}

fn get_pos<T>(a: &Vec<T>, e: T) -> usize
where
    T: std::cmp::PartialEq + Copy,
{
    return a.iter().position(|&y| y == e).unwrap();
}

fn part_1(order_rules: &HashMap<i32, Vec<i32>>, chains: &Vec<Vec<i32>>) -> i32 {
    let valid_chains: Vec<&Vec<i32>> = chains
        .iter()
        .filter(|chain| {
            chain.iter().enumerate().all(|(index, number)| {
                chain
                    .iter()
                    .filter(|&x| match order_rules.get(number) {
                        Some(rules) => rules.contains(x),
                        None => false,
                    })
                    .all(|&x| index < get_pos(chain, x))
            })
        })
        .collect();

    return valid_chains.iter().map(|v| v[v.len() / 2]).sum();
}

fn part_2(order_rules: &HashMap<i32, Vec<i32>>, chains: &Vec<Vec<i32>>) -> i32 {
    let invalid_chains: Vec<&Vec<i32>> = chains
    .iter()
    .filter(|chain| {
        chain.iter().enumerate().any(|(index, number)| {
            chain
                .iter()
                .filter(|&x| match order_rules.get(number) {
                    Some(rules) => rules.contains(x),
                    None => false,
                })
                .any(|&x| !(index < get_pos(chain, x)))
        })
    })
    .collect();

    let corrected: Vec<Vec<&i32>> = invalid_chains.iter().map(|&chain| {
        let mut new_chain: Vec<&i32> = Vec::new();
        new_chain.push(&chain[0]);
        for l in &chain[1..] {
            let mut inserted = false;
            for i in (0..new_chain.len()).rev() {
                match order_rules.get(new_chain[i]) {
                    Some(rule) => if rule.contains(l) {
                        new_chain.insert(i + 1, l);
                        inserted = true;
                        break;
                    }
                    None => continue
                }
            }
            if !inserted {
                new_chain.insert(0, l)
            }
        }
        return new_chain;
    }).collect();

    return corrected.iter().map(|v| v[v.len() / 2]).sum();
}

fn parse_input(input_path: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let buffer = read_to_string(input_path).unwrap();

    let order_rule_pattern = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let chain_rule_pattern = Regex::new(r"(\d+,)+\d+").unwrap();

    let order_rules: Vec<(i32, i32)> = order_rule_pattern
        .find_iter(&buffer)
        .map(|m| {
            m.as_str()
                .split_once("|")
                .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                .unwrap()
        })
        .collect();
    let order_map: HashMap<i32, Vec<i32>> =
        HashMap::from_iter(order_rules.iter().map(|x| x.0).unique().map(|key| {
            (
                key,
                order_rules
                    .iter()
                    .filter(|(a, _)| *a == key)
                    .map(|(_, b)| *b)
                    .collect(),
            )
        }));

    let chains: Vec<Vec<i32>> = chain_rule_pattern
        .find_iter(&buffer)
        .map(|m| {
            m.as_str()
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    return (order_map, chains);
}
