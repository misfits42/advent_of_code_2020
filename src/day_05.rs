#[aoc_generator(day5)]
fn generate_input(input: &str) -> Vec<u64> {
    let mut seat_specs: Vec<u64> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.len() != 10 {
            panic!("Day 5 - input line length incorrect!");
        }
        seat_specs.push(calculate_seat_id(&line.to_string()));
    }
    // Sort the calculated seat IDs
    seat_specs.sort();
    return seat_specs;
}

#[aoc(day5, part1)]
fn solve_part_1(seat_ids: &Vec<u64>) -> u64 {
    // Seat IDs are already sorted, so highest seat ID is last value
    return *seat_ids.last().unwrap();
}

#[aoc(day5, part2)]
fn solve_part_2(seat_ids: &Vec<u64>) -> u64 {
    // Seat IDs are already sorted by the generator function - look for gap in seat IDs
    for i in 1..seat_ids.len() {
        let previous_id = seat_ids[i - 1];
        let current_id = seat_ids[i];
        if current_id - previous_id > 1 {
            return current_id - 1;
        }
    }
    panic!("Day 5 Part 2 - should not get here!");
}

/// Calculates the seat ID from the seat specification - consisting of 10 letters, the first 7 being
/// either 'F' or 'B' and the last 3 being either 'L' or 'R'.
fn calculate_seat_id(seat_spec: &String) -> u64 {
    let chars = seat_spec.chars().collect::<Vec<char>>();
    // Start by finding row number
    let mut r_lower = 0;
    let mut r_upper = 127;
    for i in 0..7 {
        if chars[i] == 'F' {
            r_upper = r_lower + (r_upper - r_lower) / 2;
        } else if chars[i] == 'B' {
            r_lower = r_upper - (r_upper - r_lower) / 2;
        }
    }
    // Now look for column number
    let mut c_lower = 0;
    let mut c_upper = 7;
    for i in 7..10 {
        if chars[i] == 'L' {
            c_upper = c_lower + (c_upper - c_lower) / 2;
        } else if chars[i] == 'R' {
            c_lower = c_upper - (c_upper - c_lower) / 2;
        }
    }
    // Now we have found the row and column number
    return r_lower * 8 + c_lower;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d05_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day5.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(955, result);
    }

    #[test]
    fn test_d05_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day5.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(569, result);
    }
}
