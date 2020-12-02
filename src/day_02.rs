use regex::Regex;

#[aoc_generator(day2)]
fn generate_input(input: &str) -> Vec<(usize, usize, String, String)> {
    let mut output: Vec<(usize, usize, String, String)> = vec![];
    let password_regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let captures = password_regex.captures(line).unwrap();
        let lower = captures[1].parse::<usize>().unwrap();
        let upper = captures[2].parse::<usize>().unwrap();
        let c = captures[3].to_string();
        let password = captures[4].to_string();
        output.push((lower, upper, c, password));
    }
    return output;
}

#[aoc(day2, part1)]
fn solve_part_1(input: &Vec<(usize, usize, String, String)>) -> u64 {
    let mut valid_count = 0;
    for (lower, upper, c, password) in input {
        // Count number of times check character occurs in password
        let count = &password.matches(c).count();
        // Increase valid count if count within specified range (inclusive lower and upper)
        if count >= lower && count <= upper {
            valid_count += 1;
        }
    }
    return valid_count;
}

#[aoc(day2, part2)]
fn solve_part_2(input: &Vec<(usize, usize, String, String)>) -> u64 {
    let mut valid_count = 0;
    for (lower, upper, c, password) in input {
        // Adjust lower and upper indices to accomodate for off-by-one
        let lower = lower - 1;
        let upper = upper - 1;
        let mut count = 0;
        // Check if validation character present at lower and upper indices
        if lower <= password.len() - 1 && &password[lower..lower+1] == c {
            count += 1;
        }
        if upper <= password.len() - 1 && &password[upper..upper+1] == c {
            count += 1;
        }
        // Increment valid count if check character is at exactly one of the specified indices
        if count == 1 {
            valid_count += 1;
        }
    }
    return valid_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d02_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day2.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(454, result);
    }

    #[test]
    fn test_d02_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day2.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(649, result);
    }
}
