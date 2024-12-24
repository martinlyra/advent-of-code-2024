use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use itertools::Itertools;
use regex::Regex;

#[derive(PartialEq, Eq, Hash)]
enum Operation {
    Xor,
    Or,
    And,
}

#[derive(PartialEq, Eq, Hash)]
struct LogicGate {
    input_a: String,
    input_b: String,
    output: String,
    operation: Operation,
}

impl LogicGate {
    fn is_input_node(&self) -> bool {
        self.input_a.starts_with(&['x', 'y']) && self.input_b.starts_with(&['x', 'y'])
    }

    fn is_terminal_node(&self) -> bool {
        self.output.starts_with('z')
    }

    fn is_next_of(&self, other: &LogicGate) -> bool {
        self.input_a == other.output || self.input_b == other.output
    }

    fn is_bit(&self, bit: u32) -> bool {
        let as_string = bit.to_string();
        self.input_a.ends_with(&as_string) && self.input_b.ends_with(&as_string)
    }
}

fn main() {
    let test_1 = read_input("./input/day_24.test.1.txt");
    let test_2 = read_input("./input/day_24.test.2.txt");
    let input = read_input("./input/day_24.txt");

    println!(
        "First part test 1 answer: {} == 4",
        part_1(&test_1.0, &test_1.1)
    );
    println!(
        "First part test 2 answer: {} == 2024",
        part_1(&test_2.0, &test_2.1)
    );
    println!("First part answer: {}", part_1(&input.0, &input.1));
    println!("Second part answer: {}", part_2(&input.1));
}

fn evaluate_circuit(
    initial_values: &HashMap<String, bool>,
    gates: &Vec<LogicGate>,
) -> Option<HashMap<String, bool>> {
    // Uses deferred to wait for other calculations, instead of
    // re-queueing the gate over and over.
    // This way we avoid infinite loops due to poorly connected
    // circuit graph. This was useful for part 2 until I had to
    // rethink what I trying to do with my brute-force.
    let mut memory = initial_values.clone();
    let mut queue = gates.into_iter().collect::<VecDeque<_>>();
    let mut deferred: HashMap<&String, HashSet<&LogicGate>> =
        gates.iter().fold(HashMap::new(), |mut map, value| {
            map.insert(&value.output, HashSet::new());
            map
        });

    while let Some(g) = queue.pop_front() {
        let value_a = memory.get(&g.input_a);
        let value_b = memory.get(&g.input_b);
        if value_a.is_none() {
            deferred.entry(&g.input_a).and_modify(|v| {
                v.insert(g);
            });
            continue;
        }
        if value_b.is_none() {
            deferred.entry(&g.input_b).and_modify(|v| {
                v.insert(g);
            });
            continue;
        }

        let val_a = value_a.unwrap();
        let val_b = value_b.unwrap();
        let output_value = match g.operation {
            Operation::And => val_a & val_b,
            Operation::Or => val_a | val_b,
            Operation::Xor => val_a ^ val_b,
        };

        memory.insert(g.output.clone(), output_value);

        // Awake deferred calculations
        let waiting_on = deferred.get_mut(&g.output).unwrap();
        if !waiting_on.is_empty() {
            queue.extend(waiting_on.iter());
            waiting_on.clear();
        }
    }

    // If there are any still any deferred calculations, then the
    // circuit graph is bad.
    if deferred.iter().any(|(_, waiting)| !waiting.is_empty()) {
        return None;
    }

    Some(
        memory
            .into_iter()
            .filter(|(k, _)| k.starts_with('z'))
            .collect(),
    )
}

fn part_1(initial_values: &HashMap<String, bool>, instructions: &Vec<LogicGate>) -> u64 {
    memory_to_number(
        &evaluate_circuit(initial_values, instructions).unwrap(),
        'z',
    )
}

fn find_bad_gates(gates: &Vec<LogicGate>) -> Vec<&String> {
    // All credit to my saviours: @ropewalker and @wilkotom at Github
    // https://github.com/ropewalker/advent_of_code_2024/blob/master/src/day24.rs
    // https://github.com/wilkotom/AdventOfCode/blob/main/rust/2024/day24/src/main.rs
    let mut to_replace: Vec<&String> = vec![];

    for g in gates {
        if g.is_terminal_node() && g.operation != Operation::Xor && g.output != "z45" {
            to_replace.push(&g.output);
        }
        if !(g.is_terminal_node() || g.is_input_node()) && g.operation == Operation::Xor {
            to_replace.push(&g.output);
        }
        if g.operation == Operation::Xor
            && g.is_input_node()
            && g.output != "z00"
            && !gates
                .iter()
                .any(|next| next.operation == Operation::Xor && next.is_next_of(g))
        {
            to_replace.push(&g.output);
        }
        if g.operation == Operation::And
            && !g.is_bit(0)
            && !gates
                .iter()
                .any(|next| next.operation == Operation::Or && next.is_next_of(g))
        {
            to_replace.push(&g.output);
        }
    }

    to_replace.into_iter().unique().sorted().collect()
}

fn part_2(gates: &Vec<LogicGate>) -> String {
    find_bad_gates(gates).into_iter().join(",")
}

fn memory_to_binary_string(memory: &HashMap<String, bool>, prefix: char) -> String {
    memory
        .into_iter()
        .filter(|(key, _)| key.starts_with(prefix))
        .sorted()
        .rev()
        .fold(Vec::new(), |mut vec, (_, value)| {
            vec.push((*value as u8).to_string());
            vec
        })
        .join("")
}

fn memory_to_number(memory: &HashMap<String, bool>, prefix: char) -> u64 {
    u64::from_str_radix(&memory_to_binary_string(memory, prefix), 2).unwrap()
}

fn read_input(file_path: &str) -> (HashMap<String, bool>, Vec<LogicGate>) {
    let break_pattern = Regex::new(r"(\r?\n){2,}").unwrap();
    let string = read_to_string(file_path).unwrap().trim().to_owned();
    let parts: Vec<&str> = break_pattern.split(&string).collect();

    let variable_pattern = Regex::new(r"([\w\d]+): (\d)").unwrap();
    let gate_pattern = Regex::new(r"([\w\d]+) (\w+) ([\w\d]+) -> ([\w\d]+)").unwrap();

    (
        variable_pattern
            .captures_iter(parts[0])
            .fold(HashMap::new(), |mut map, entry| {
                let (_, [id, value]) = entry.extract();
                let id_owned = id.to_string();
                map.insert(id_owned, value.parse::<u8>().unwrap() != 0);
                map
            }),
        gate_pattern
            .captures_iter(parts[1])
            .fold(Vec::new(), |mut vec, entry| {
                let (_, [id_a, operation, id_b, id_out]) = entry.extract();
                vec.push(LogicGate {
                    input_a: id_a.to_owned(),
                    input_b: id_b.to_owned(),
                    output: id_out.to_owned(),
                    operation: match operation {
                        "XOR" => Operation::Xor,
                        "AND" => Operation::And,
                        "OR" => Operation::Or,
                        e => panic!("Unknown operation {e}"),
                    },
                });
                vec
            }),
    )
}
