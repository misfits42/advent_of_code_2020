use std::collections::VecDeque;

#[aoc_generator(day9)]
fn generate_input(input: &str) -> Vec<u64> {
    return input.lines().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
}

#[aoc(day9, part1)]
fn solve_part_1(xmas_stream: &Vec<u64>) -> u64 {
    let result = find_invalid_number(xmas_stream, 25);
    if result.is_some() {
        return result.unwrap();
    }
    // Should have found invalid number already, so should not get to this point.
    panic!("Day 9 Part 1 - should not get here!");
}

#[aoc(day9, part2)]
fn solve_part_2(xmas_stream: &Vec<u64>) -> u64 {
    let invalid_num = find_invalid_number(xmas_stream, 25).unwrap();
    // Try ranges of varying lengths
    for length in 2..=xmas_stream.len() {
        let mut range: VecDeque<u64> = VecDeque::new();
        let mut range_sum = 0;
        for start_i in 0..xmas_stream.len() {
            // Check if we can check array of current length from current starting index
            if start_i + length >= xmas_stream.len() {
                break;
            }
            // Update the range sum and values included within the range
            if start_i == 0 {
                // Initialise range and count
                for i in 0..length {
                    range.push_back(xmas_stream[i]);
                    range_sum += xmas_stream[i];
                }
            } else {
                // Pop front from range and push new value to back
                let out = range.pop_front().unwrap();
                range_sum -= out;
                let new_in = xmas_stream[start_i + length - 1];
                range_sum += new_in;
                range.push_back(new_in);
            }
            // Check if range adds up to the invalid number
            if range_sum == invalid_num {
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                return min + max;
            }
        }
    }
    panic!("Day 9 Part 2 - should not get here!");
}

/// Finds the first invalid number in the XMAS stream.
fn find_invalid_number(xmas_stream: &Vec<u64>, preamble_size: usize) -> Option<u64> {
    for i in 25..xmas_stream.len() {
        if !check_for_previous_sum(xmas_stream, i, preamble_size) {
            return Some(xmas_stream[i]);
        }
    }
    return None;
}

/// Checks the XMAS data stream to see if any two values in the range of length equal to preamble
/// before value at index sum to the value at index.
fn check_for_previous_sum(xmas_stream: &Vec<u64>, index: usize, preamble_size: usize) -> bool {
    for i in (index - preamble_size)..index {
        for j in (i + 1)..index {
            // Two number in sum must be different
            if i == j || xmas_stream[i] == xmas_stream[j] {
                continue;
            }
            if xmas_stream[index] == xmas_stream[i] + xmas_stream[j] {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d09_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day9.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(31161678, result);
    }

    #[test]
    fn test_d09_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day9.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(5453868, result);
    }
}
