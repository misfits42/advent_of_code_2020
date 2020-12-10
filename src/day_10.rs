use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day10)]
fn generate_input(input: &str) -> Vec<u64> {
    let mut adapters = input.lines().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
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
    let mut adapters_set = adapters.iter().map(|x| *x).collect::<HashSet<u64>>();
    adapters_set.insert(0);
    let mut adapters_next: HashMap::<u64, Vec<u64>> = HashMap::new();
    for adapter in adapters_set.iter() {
        let mut conns: Vec<u64> = vec![];
        for diff in 1..=3 {
            let check_joltage = adapter + diff;
            if adapters_set.contains(&check_joltage) {
                conns.push(check_joltage);
            }
        }
        adapters_next.insert(*adapter, conns);
    }
    println!("{:?}", adapters_next);
    let result = find_adapter_arrangement(&adapters_set, &adapters_next, 0);
    return result;
}

fn find_adapter_arrangement(adapters: &HashSet<u64>, adapters_next: &HashMap<u64, Vec<u64>>, current_joltage: u64) -> u64 {
    // Check if there is an adapter that can be connected next
    let mut count = 0;
    let next_adapters = adapters_next.get(&current_joltage).unwrap();
    if next_adapters.is_empty() {
        count += 1;
    } else {
        for adapter in next_adapters {
            count += find_adapter_arrangement(adapters, adapters_next, *adapter);
        }
    }
    if count > 10000000 {
        println!("Count: {}", count);
    }
    return count;
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
