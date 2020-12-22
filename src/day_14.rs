use std::collections::HashMap;

use regex::Regex;

enum Operation {
    SetMask{mask_bits: Vec<(u64, u64)>},
    SetMemory{index: u64, value: u64}
}

#[aoc_generator(day14)]
fn generate_input(input: &str) -> Vec<Operation> {
    // Create regexes to match mask and memory operation lines
    let mask_regex = Regex::new(r"^mask = ((X|1|0){36})$").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let mut operations: Vec<Operation> = vec![];
    for line in input.lines() {
        // Trim leading and trailing whitespace, then ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Check if we have a set mask or set memory line
        if mask_regex.is_match(line) {
            let mut mask_bits: Vec<(u64, u64)> = vec![];
            let captures = mask_regex.captures(line).unwrap();
            let mask_chars = captures[1].chars().collect::<Vec<char>>();
            for i in 0..36 {
                let place = (35 - i) as u64;
                if mask_chars[i] != 'X' {
                    let bit = mask_chars[i].to_digit(10).unwrap() as u64;
                    mask_bits.push((place, bit))
                }
            }
            let operation = Operation::SetMask{mask_bits: mask_bits};
            operations.push(operation);
        } else if mem_regex.is_match(line) {
            let captures = mem_regex.captures(line).unwrap();
            let index = captures[1].parse::<u64>().unwrap();
            let value = captures[2].parse::<u64>().unwrap();
            let operation = Operation::SetMemory{index: index, value: value};
            operations.push(operation);
        } else {
            panic!("Day 14 - malformed input file!");
        }
    }
    return operations;
}

/// Applies the bitwise projection operation (proj) to the given input value.
/// 
/// Truth table (a proj b):
/// 
/// a | b | result
/// --|---|-------
/// 0 | 0 | 0
/// 0 | 1 | 0
/// 1 | 0 | 1
/// 1 | 1 | 1
fn apply_bitwise_projection(input: u64, index: u64, bit: u64) -> Option<u64> {
    if index > 36 {
        return None;
    }
    let bit = bit & 0x1;
    if bit == 0 { // clear bit
        let mask = 0xFFFFFFFFFFFFFFFF ^ (1 << index);
        return Some(input & mask);
    } else if bit == 1 { // set bit
        let mask = 0 ^ (1 << index);
        return Some(input | mask);
    } else {
        return None;
    }
}

#[aoc(day14, part1)]
fn solve_part_1(operations: &Vec<Operation>) -> u64 {
    let mut port_comp_memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask: &Vec<(u64, u64)> = &vec![];
    // Process all operations, being set mask or set memory operations
    for operation in operations.iter() {
        match operation {
            Operation::SetMask{mask_bits} => {
                current_mask = mask_bits;
            },
            Operation::SetMemory{index, value} => {
                // Apply bitwise projection to value as required before loading value into memory
                let mut new_value = *value;
                for (index, bit) in current_mask.iter() {
                    new_value = apply_bitwise_projection(new_value, *index, *bit).unwrap();
                }
                port_comp_memory.insert(*index, new_value);
            },
        }
    }
    // Add all values in memory that are NEQ 0
    let mut values = port_comp_memory.values().map(|x| *x).collect::<Vec<u64>>();
    values.retain(|x| *x != 0);
    let sum = values.iter().sum();
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d14_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day14.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(17481577045893, result);
    }
}
