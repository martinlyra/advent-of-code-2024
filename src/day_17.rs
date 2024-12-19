use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

use itertools::Itertools;
use regex::Regex;

type RegisterType = u64;
type OperandType = u8;

fn main() {
    let test_1 = read_input("./input/day_17.test.1.txt");
    let test_2 = read_input("./input/day_17.test.2.txt");
    let input = read_input("./input/day_17.txt");

    println!(
        "1st part test: {:?}",
        run_program(&test_1.0, &test_1.1).iter().join(",")
    );
    println!(
        "1st part: {:?}",
        run_program(&input.0, &input.1).iter().join(",")
    );

    println!("2nd part test: {:?} == 117440", crack_program(&test_2.1));
    println!("2nd part: {:?}", crack_program(&input.1));
}

fn crack_program(program: &Vec<OperandType>) -> Option<RegisterType> {
    let mut queue: VecDeque<(usize, RegisterType)> = VecDeque::new();
    queue.push_back((program.len() - 1, 0));

    while let Some(next_job) = queue.pop_front() {
        let (offset, next_value) = next_job;
        for number in 0..8 {
            let test_value: RegisterType = (next_value << 3) + (number as RegisterType);

            let registers = HashMap::from_iter(vec![('A', test_value), ('B', 0), ('C', 0)]);

            let result = run_program(&registers, program);

            if result == *program {
                return Some(test_value);
            } else if result == program[offset..] {
                queue.push_back((offset - 1, test_value));
            }
        }
    }
    None
}

fn parse_operand(registers: &HashMap<char, RegisterType>, operand: OperandType) -> RegisterType {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[&'A'],
        5 => registers[&'B'],
        6 => registers[&'C'],
        e => panic!("Invalid operand: {e}"),
    }
}

fn run_program(
    registers: &HashMap<char, RegisterType>,
    program: &Vec<OperandType>,
) -> Vec<OperandType> {
    let mut registers = registers.clone();
    let mut program_pointer = 0;
    let mut output = Vec::new();
    while program_pointer < program.len() {
        let op_code = program[program_pointer];
        let operand = program[program_pointer + 1];
        let mut move_pointer = true;

        let value = parse_operand(&registers, operand);
        match op_code {
            0 => {
                // adv
                registers.insert('A', registers[&'A'] / RegisterType::pow(2, value as u32));
            }
            1 => {
                // bxl
                registers.insert('B', registers[&'B'] ^ value);
            }
            2 => {
                // bst
                registers.insert('B', value % 8);
            }
            3 => {
                // jnz
                if registers[&'A'] != 0 {
                    program_pointer = value as usize;
                    move_pointer = false;
                }
            }
            4 => {
                // bxc
                registers.insert('B', registers[&'B'] ^ registers[&'C']);
            }
            5 => {
                // out
                output.push((value % 8) as OperandType);
            }
            6 => {
                // bdv
                registers.insert('B', registers[&'A'] / RegisterType::pow(2, value as u32));
            }
            7 => {
                // cdv
                registers.insert('C', registers[&'A'] / RegisterType::pow(2, value as u32));
            }
            e => panic!("Unknown opcode: {e}"),
        }
        if move_pointer {
            program_pointer += 2;
        }
    }
    output
}

fn read_input(file_path: &str) -> (HashMap<char, RegisterType>, Vec<OperandType>) {
    let register_pattern = Regex::new(r"Register (\w): (\d+)").unwrap();
    let program_pattern = Regex::new(r"Program: ((\d+,?)+)").unwrap();

    let buffer = read_to_string(file_path).unwrap();
    (
        HashMap::from_iter(register_pattern.captures_iter(&buffer).map(|capture| {
            let (_, [id, value]) = capture.extract();
            (
                id.chars().next().unwrap(),
                value.parse::<RegisterType>().unwrap(),
            )
        })),
        program_pattern.captures(&buffer).unwrap().extract::<2>().1[0]
            .split(",")
            .map(|s| s.parse::<OperandType>().unwrap())
            .collect(),
    )
}
