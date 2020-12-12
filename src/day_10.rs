use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc_generator(day10)]
fn generate_input(input: &str) -> Vec<u64> {
    let mut adapters = input.lines().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    adapters.push(0);
    adapters.sort();
    return adapters;
}

#[aoc(day10, part1)]
fn solve_part_1(adapters: &Vec<u64>) -> u64 {
    let mut total_diff_1 = 0;
    let mut total_diff_3 = 0;
    for i in 0..adapters.len() + 1 {
        let prev = {
            if i == 0 {
                0
            } else {
                adapters[i - 1]
            }
        };
        let current = {
            if i < adapters.len() {
                adapters[i]
            } else {
                adapters[adapters.len() - 1] + 3
            }
        };
        let diff = current - prev;
        if diff == 1 {
            total_diff_1 += 1;
        } else if diff == 3 {
            total_diff_3 += 1;
        }
    }
    return total_diff_1 * total_diff_3;
}

#[aoc(day10, part2)]
fn solve_part_2(adapters: &Vec<u64>) -> u64 {
    // Determine sets of adapters with joltage ratings separated by only 1 joltage
    let mut adapter_groups: VecDeque<HashSet<u64>> = VecDeque::new();
    let mut in_group = false;
    for i in 1..adapters.len() {
        // Compare current to previous
        let diff = adapters[i] - adapters[i - 1];
        if diff == 1 {
            if !in_group {
                adapter_groups.push_back(HashSet::new());
                in_group = true;
            }
            adapter_groups.back_mut().unwrap().insert(adapters[i - 1]);
            adapter_groups.back_mut().unwrap().insert(adapters[i]);
        } else {
            in_group = false;
        }
    }
    // Determine number of possible paths in each group, multiplying counts to get overall answer
    let mut result: u64 = 1;
    let base: u64 = 2;
    for group in adapter_groups {
        // Calculate number of paths possible within group based on there length
        let count = {
            let len = group.len() as u32;
            if len == 5 {
                base.pow(len - 2) - 1
            } else {
                base.pow(len - 2)
            }
        };
        result *= count;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d10_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day10.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(2170, result);
    }

    #[test]
    fn test_d10_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day10.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(24803586664192, result);
    }

    #[test]
    fn test_d10_p1_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day10_test_001.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(35, result);
    }

    #[test]
    fn test_d10_p2_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day10_test_001.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(8, result);
    }

    #[test]
    fn test_d10_p1_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day10_test_002.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(220, result);
    }

    #[test]
    fn test_d10_p2_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day10_test_002.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(19208, result);
    }
}
