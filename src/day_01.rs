#[aoc_generator(day1)]
fn generate_input(input: &str) -> Vec<u64> {
    let mut output: Vec<u64> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value = line.parse::<u64>().unwrap();
        output.push(value);
    }
    return output;
}

#[aoc(day1, part1)]
fn solve_part_1(values: &Vec<u64>) -> u64 {
    for i in 0..values.len() {
        // Check values ahead of first index, to prevent duplication of work
        for j in (i+1)..values.len() {
            if values[i] + values[j] == 2020 {
                return values[i] * values[j];
            }
        }
    }
    panic!("Day 1 Part 1 - should not get here!");
}

#[aoc(day1, part2)]
fn solve_part_2(values: &Vec<u64>) -> u64 {
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            for k in (j + 1)..values.len() {
                if values[i] + values[j] + values[k] == 2020 {
                    return values[i] * values[j] * values[k];
                }
            }
        }
    }
    panic!("Day 1 Part 2 - should not get here!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d01_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day1.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(996075, result);
    }

    #[test]
    fn test_d01_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day1.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(51810360, result);
    }
}
