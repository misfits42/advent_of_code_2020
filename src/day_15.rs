use std::collections::HashMap;

#[aoc_generator(day15)]
fn generate_input(input: &str) -> Vec<u64> {
    return input.trim().split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
}

#[aoc(day15, part1)]
fn solve_part_1(starting_numbers: &Vec<u64>) -> u64 {
    return play_memory_game(2020, starting_numbers);
}

#[aoc(day15, part2)]
fn solve_part_2(starting_numbers: &Vec<u64>) -> u64 {
    return play_memory_game(30000000, starting_numbers);
}

/// Plays the Elves' memory game for the specified number of turns using the given starting numbers.
/// 
/// Rules as described in AOC 2020 Day 15.
fn play_memory_game(num_turns: usize, starting_numbers: &Vec<u64>) -> u64 {
    // Record the numbers spoken so far and on what turns they have been spoken
    let mut nums_spoken: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut last_num_spoken = 0;
    // Conduct the turns
    for current_turn in 0..num_turns {
        // Determine the number to be spoken on current turn
        let number_spoken = {
            if current_turn < starting_numbers.len() {
                starting_numbers[current_turn]
            } else {
                let last_number_spoken_turns = nums_spoken.get(&last_num_spoken).unwrap();
                if last_number_spoken_turns.len() == 1 {
                    // Last number spoken was spoken for the first time
                    0
                } else {
                    // Last number spoken has been spoken multiple times
                    let end_i = last_number_spoken_turns.len() - 1;
                    last_number_spoken_turns[end_i] - last_number_spoken_turns[end_i - 1]
                }
            }
        };
        // Record the number as having been spoken
        if !nums_spoken.contains_key(&number_spoken) {
            nums_spoken.insert(number_spoken, vec![current_turn as u64]);
        } else {
            nums_spoken.get_mut(&number_spoken).unwrap().push(current_turn as u64);
        }
        // End of current turn
        last_num_spoken = number_spoken;
    }
    return last_num_spoken;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d15_p1_proper() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/day15.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(371, result);
    }

    #[test]
    fn test_d15_p2_proper() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/day15.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(352, result);
    }

    #[test]
    fn test_d15_p1_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_001.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(436, result);
    }

    #[test]
    fn test_d15_p1_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_002.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(1, result);
    }

    #[test]
    fn test_d15_p1_003() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_003.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(10, result);
    }

    #[test]
    fn test_d15_p1_004() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_004.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(27, result);
    }

    #[test]
    fn test_d15_p1_005() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_005.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(78, result);
    }

    #[test]
    fn test_d15_p1_006() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_006.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(438, result);
    }

    #[test]
    fn test_d15_p1_007() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_007.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(1836, result);
    }

    #[test]
    fn test_d15_p2_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_001.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(175594, result);
    }

    #[test]
    fn test_d15_p2_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_002.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(2578, result);
    }

    #[test]
    fn test_d15_p2_003() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_003.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(3544142, result);
    }

    #[test]
    fn test_d15_p2_004() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_004.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(261214, result);
    }

    #[test]
    fn test_d15_p2_005() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_005.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(6895259, result);
    }

    #[test]
    fn test_d15_p2_006() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_006.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(18, result);
    }

    #[test]
    fn test_d15_p2_007() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day15_test_007.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(362, result);
    }
}
