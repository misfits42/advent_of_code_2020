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
        let count = password.matches(c).count();
        if count >= *lower && count <= *upper {
            valid_count += 1;
        }
    }
    return valid_count;
}
