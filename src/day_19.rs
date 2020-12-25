use std::collections::HashMap;

use regex::Captures;
use regex::Regex;

#[aoc_generator(day19)]
fn generate_input(input: &str) -> (HashMap<u64, String>, Vec<String>) {
    let rule_regex = Regex::new(r"(\d+): (.*)").unwrap();
    // Split input into two segments
    let segments = input.trim().split("\n\n").collect::<Vec<&str>>();
    // Extract rules
    let rules_raw = segments[0]
        .lines()
        .map(|x| x.trim())
        .map(|x| rule_regex.captures(x).unwrap())
        .map(|x| (x[1].parse::<u64>().unwrap(), x[2].to_string()))
        .collect::<HashMap<u64, String>>();
    // Extract messages
    let messages = segments[1]
        .lines()
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();
    return (rules_raw, messages);
}

#[aoc(day19, part1)]
fn solve_part_1(input: &(HashMap<u64, String>, Vec<String>)) -> u64 {
    let rules_raw = input.0.clone();
    let messages = input.1.clone();
    // Generate regex for rule 0
    let rule_0_regex_str = format!(
        "^{}$",
        generate_regex_str_from_rules(0, &rules_raw).unwrap(),
    );
    let regex = Regex::new(&rule_0_regex_str).unwrap();
    let mut valid_count = 0;
    for message in messages {
        if regex.is_match(&message) {
            valid_count += 1;
        }
    }
    return valid_count;
}

#[aoc(day19, part2)]
fn solve_part_2(input: &(HashMap<u64, String>, Vec<String>)) -> u64 {
    let mut rules_raw = input.0.clone();
    let messages = input.1.clone();
    // Make replacements of rules 8 and 11 - amended to allow matching of looped rule
    rules_raw.insert(8, String::from("(42)+"));
    rules_raw.insert(
        11,
        String::from("(42 31)|(42 42 31 31)|(42 42 42 31 31 31)|(42 42 42 42 31 31 31 31)"),
    );
    // Generate regex for rule 0
    let rule_0_regex_str = format!(
        "^{}$",
        generate_regex_str_from_rules(0, &rules_raw).unwrap(),
    );
    let regex = Regex::new(&rule_0_regex_str).unwrap();
    let mut valid_count = 0;
    for message in messages {
        if regex.is_match(&message) {
            valid_count += 1;
        }
    }
    return valid_count;
}

fn generate_regex_str_from_rules(
    rule_num: u64,
    rules_raw: &HashMap<u64, String>,
) -> Option<String> {
    // Check if the current rule number exists
    if !rules_raw.contains_key(&rule_num) {
        return None;
    }
    // Get the rule text
    let rule_text = rules_raw.get(&rule_num).unwrap().clone();
    let mut regex_str = rule_text.clone();
    // Check if there are any sub-rules that need to be substituted
    if rule_text.starts_with("\"") {
        return Some(rule_text.replace("\"", ""));
    }
    let rule_num_regex = Regex::new(r"(\d+)*").unwrap();
    for cap in rule_num_regex.captures_iter(&rule_text) {
        if cap.len() != 2 || cap.get(1).is_none() {
            continue;
        }
        let replace_regex = Regex::new(&format!(
            r"(\s+({})\s+)|(^({})\s+)|(\s+({})$)|^({})$|\(({})\)|\(({})\s+|\s+({})\)",
            &cap[1], &cap[1], &cap[1], &cap[1], &cap[1], &cap[1], &cap[1]
        ))
        .unwrap();
        let new_rule_num = cap[1].parse::<u64>().unwrap();
        let sub_regex =
            generate_regex_str_from_rules(new_rule_num, rules_raw).unwrap();
        regex_str = {
            if sub_regex.len() == 1 {
                replace_regex
                    .replace_all(&regex_str, |_caps: &Captures| format!(" {} ", sub_regex))
                    .to_string()
            } else {
                replace_regex
                    .replace_all(&regex_str, |_caps: &Captures| format!(" ({}) ", sub_regex))
                    .to_string()
            }
        };
    }
    regex_str = regex_str.replace(" ", "");
    return Some(regex_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d19_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day19.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(184, result);
    }

    #[test]
    fn test_d19_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day19.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(389, result);
    }

    #[test]
    fn test_d19_p1_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day19_test_002.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(2, result);
    }

    #[test]
    fn test_d19_p2_003() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day19_test_003.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(12, result);
    }
}
