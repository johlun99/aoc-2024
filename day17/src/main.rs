use core::panic;
use std::collections::HashMap;

struct Input {
    registers: HashMap<char, u32>,
    program: Vec<u32>,
}

fn main() {
    let input = include_str!("input.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> String {
    let mut p = 0;
    let mut registers = input.registers.clone();
    let program = input.program.clone();

    let mut output: Vec<u32> = Vec::new();

    while p < program.len() - 1 {
        let opcode = program[p];
        let operand = program[p + 1];

        if opcode == 0 {
            let combo = get_combo_operand(&operand, &registers);

            if let Some(val) = registers.get_mut(&'A') {
                *val /= 2_u32.pow(combo);
            } else {
                panic!("Invalid register A!")
            }
        } else if opcode == 1 {
            if let Some(val) = registers.get_mut(&'B') {
                *val ^= operand;
            } else {
                panic!("Invalid register B!");
            }
        } else if opcode == 2 {
            let combo = get_combo_operand(&operand, &registers);

            if let Some(val) = registers.get_mut(&'B') {
                *val = combo % 8;
            } else {
                panic!("Invalid register B!");
            }
        } else if opcode == 3 {
            if let Some(val) = registers.get(&'A') {
                if *val != 0 {
                    p = operand as usize;
                    continue;
                }
            } else {
                panic!("Invalid Register A!");
            }
        } else if opcode == 4 {
            let c_val = registers.get(&'C').cloned().expect("Invalid Register C!");
            if let Some(val) = registers.get_mut(&'B') {
                *val ^= c_val;
            } else {
                panic!("Invalid Register B!");
            }
        } else if opcode == 5 {
            let combo = get_combo_operand(&operand, &registers);
            output.push(combo % 8);
        } else if opcode == 6 {
            let combo = get_combo_operand(&operand, &registers);

            if let Some(val) = registers.get_mut(&'B') {
                *val /= 2_u32.pow(combo);
            } else {
                panic!("Invalid Register C!");
            }
        } else if opcode == 7 {
            let combo = get_combo_operand(&operand, &registers);

            if let Some(val) = registers.get_mut(&'C') {
                *val /= 2_u32.pow(combo);
            } else {
                panic!("Invalid Register C!");
            }
        } else {
            panic!("Invalid opcode!");
        }

        p += 2;
    }

    output.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",")
}

fn get_combo_operand(operand: &u32, registers: &HashMap<char, u32>) -> u32 {
    if *operand <= 3 {
        return *operand;
    }

    match operand {
        4 => *registers.get(&'A').expect("Valid operand"),
        5 => *registers.get(&'B').expect("Valid operand"),
        6 => *registers.get(&'C').expect("Valid operand"),
        _ => panic!("Invalid combo"),
    }
}

fn parse_input(input: &str) -> Input {
    let mut registers: HashMap<char, u32> = HashMap::new();
    let (registers_part, program_part) = input.split_once("\n\n").expect("valid input");

    for line in registers_part.lines() {
        if let Some((left, right)) = line.split_once(":") {
            if let Some(key) = left.trim().chars().last() {
                if let Ok(val) = right.trim().parse::<u32>() {
                    registers.insert(key, val);
                }
            }
        }
    }

    let program: Vec<u32> = program_part
        .trim()
        .split_once(" ")
        .expect("valid program")
        .1
        .split(',')
        .map(|n| n.parse::<u32>().expect("valid positive number"))
        .collect();

    Input { registers, program }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("sample-input.txt");
        let input = parse_input(input);

        assert_eq!(
            input.registers,
            HashMap::from([('A', 729), ('B', 0), ('C', 0)])
        );
        assert_eq!(input.program, vec![0, 1, 5, 4, 3, 0]);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt");
        let input = parse_input(input);

        assert_eq!(part1(&input), "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
