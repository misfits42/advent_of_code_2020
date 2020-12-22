use std::collections::HashMap;

use regex::Regex;

enum Operation {
    SetMask{mask_bits: Vec<(u64, BitmaskBit)>},
    SetMemory{index: u64, value: u64}
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum BitmaskBit {
    Bit0,
    Bit1,
    BitX
}

impl BitmaskBit {
    pub fn from_char(input: char) -> Option<BitmaskBit> {
        match input {
            '0' => return Some(BitmaskBit::Bit0),
            '1' => return Some(BitmaskBit::Bit1),
            'X' => return Some(BitmaskBit::BitX),
            _ => return None,
        }
    }
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
            let mut mask_bits: Vec<(u64, BitmaskBit)> = vec![];
            let captures = mask_regex.captures(line).unwrap();
            let mask_chars = captures[1].chars().collect::<Vec<char>>();
            for i in 0..36 {
                let place = (35 - i) as u64;
                let bitmask_bit = BitmaskBit::from_char(mask_chars[i]).unwrap();
                mask_bits.push((place, bitmask_bit));
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

/// Applies the given bitmask to the provided value using the Version 1 rules specified in AOC 2020
/// Day 14 Part 1.
fn apply_bitmask_version_1(value: u64, bitmask: &Vec<(u64, BitmaskBit)>) -> u64 {
    let mut new_value = value;
    for (index, bitmask_bit) in bitmask.iter() {
        match bitmask_bit {
            BitmaskBit::Bit0 => { // Clear bit at current index
                let mask = 0xFFFFFFFFFFFFFFFF ^ (1 << index);
                new_value &= mask;
            },
            BitmaskBit::Bit1 => { // Set bit at current index
                let mask = 0 ^ (1 << index);
                new_value |= mask;
            },
            BitmaskBit::BitX => (),
        }
    }
    return new_value;
}

/// Applies the given bitmask to the provided memory address using the Version 2 rules to generate
/// an array of modified addresses.
fn apply_bitmask_version_2(address: u64, bitmask: &Vec<(u64, BitmaskBit)>) -> Vec<u64> {
    let mut new_address = address;
    // First set any bits that need to be set
    for (index, bitmask_bit) in bitmask.iter() {
        match bitmask_bit {
            BitmaskBit::Bit0 => (),
            BitmaskBit::Bit1 => {
                let mask = 0 ^ (1 << index);
                new_address |= mask;
            },
            BitmaskBit::BitX => (),
        }
    }
    // Now handle the floating bits
    let mut floating_bits = bitmask.to_vec();
    floating_bits.retain(|x| x.1 == BitmaskBit::BitX);
    // Generate bit sequences and add results to output
    let cap = (2 as u64).pow(floating_bits.len() as u32);
    let mut modified_addresses: Vec<u64> = vec![];
    for val in 0..cap {
        // Process each bit in the current bit sequence
        let mut modified_address = new_address;
        for i in (0..floating_bits.len()).rev() {
            let bit_value = (val & (1 << i)) >> i;
            if bit_value == 0 {
                let mask = u64::MAX ^ (1 << floating_bits[i].0);
                modified_address &= mask;
            } else { // bit_value == 1
                let mask = 0 ^ (1 << floating_bits[i].0);
                modified_address |= mask;
            }
        }
        modified_addresses.push(modified_address);
    }
    // Got the new address after applying the bitmask
    return modified_addresses;
}

#[aoc(day14, part1)]
fn solve_part_1(operations: &Vec<Operation>) -> u64 {
    let mut port_comp_memory: HashMap<u64, u64> = HashMap::new();
    let mut current_bitmask: &Vec<(u64, BitmaskBit)> = &vec![];
    // Process all operations, being set mask or set memory operations
    for operation in operations.iter() {
        match operation {
            Operation::SetMask{mask_bits} => {
                current_bitmask = mask_bits;
            },
            Operation::SetMemory{index, value} => {
                // Apply bitwise projection to value as required before loading value into memory
                let new_value = apply_bitmask_version_1(*value, current_bitmask);
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

#[aoc(day14, part2)]
fn solve_part_2(operations: &Vec<Operation>) -> u64 {
    let mut port_comp_memory: HashMap<u64, u64> = HashMap::new();
    let mut current_bitmask: &Vec<(u64, BitmaskBit)> = &vec![];
    // Process all operations, being set mask or set memory operations
    for operation in operations.iter() {
        match operation {
            Operation::SetMask{mask_bits} => {
                current_bitmask = mask_bits;
            },
            Operation::SetMemory{index, value} => {
                let modified_addresses = apply_bitmask_version_2(*index, current_bitmask);
                for addr in modified_addresses {
                    port_comp_memory.insert(addr, *value);
                }
            }
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

    #[test]
    fn test_d14_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day14.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(4160009892257, result);
    }

    #[test]
    fn test_d14_p1_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day14_test_001.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(165, result);
    }

    #[test]
    fn test_d13_p2_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day14_test_002.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(208, result);
    }
}
