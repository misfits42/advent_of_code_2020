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
    let messages = segments[1].lines().map(|x| x.trim().to_string()).collect::<Vec<String>>();
    return (rules_raw, messages);
}

#[aoc(day19, part1)]
fn solve_part_1(input: &(HashMap<u64, String>, Vec<String>)) -> u64 {
    let rules_raw = input.0.clone();
    let messages = input.1.clone();
    // Generate regex for rule 0
    let rule_0_regex_str = generate_regex_str_from_rules(0, &rules_raw).unwrap();
    let regex = Regex::new(&rule_0_regex_str).unwrap();
    let mut valid_count = 0;
    for message in messages {
        if regex.is_match(&message) {
            valid_count += 1;
        }
    }
    return valid_count;
}

fn generate_regex_str_from_rules(rule_num: u64, rules_raw: &HashMap<u64, String>) -> Option<String> {
    // Check if the current rule number exists
    if !rules_raw.contains_key(&rule_num) {
        return None;
    }
    // Get the rule text
    let rule_text = rules_raw.get(&rule_num).unwrap().clone();
    let mut regex_str = rule_text.clone();
    // Check if there are any sub-rules that need to be substituted
    if rule_text.starts_with("\"") {
        // println!("reached a base case ---- rule_num: {}, rule_text: {}", rule_num, rule_text);
        return Some(rule_text.replace("\"", ""));
    }
    let rule_num_regex = Regex::new(r"(\d+)*").unwrap();
    //let captures = rule_num_regex.captures_iter()(&rule_text).unwrap();
    // println!("[?] rule_num: {} ---- rule_text: {}", rule_num, rule_text);
    for cap in rule_num_regex.captures_iter(&rule_text) {
        // println!("[-] cap: {:?}", cap);
        if cap.len() != 2 || cap.get(1).is_none() {
            continue;
        }
        // println!("pre new rule num");
        let new_rule_num = cap[1].parse::<u64>().unwrap();
        // println!("new rule num: {}", new_rule_num);
        let sub_regex = generate_regex_str_from_rules(new_rule_num, rules_raw).unwrap();
        let replace_regex = Regex::new(&format!(r"(\s+({})\s+)|(^({})\s+)|(\s+({})$)", &cap[1],&cap[1],&cap[1])).unwrap();
        regex_str = {
            // println!(">>>>>>>>>> cap[1] = {}", &cap[1]);
            // println!("{}", regex_str);
            if sub_regex.len() == 1 {
                // regex_str.replace(&cap[1], &format!("{}", sub_regex))
                replace_regex.replace_all(&regex_str, |_caps: &Captures| {
                    format!(" {} ", sub_regex)
                }).to_string()
            } else {
                replace_regex.replace_all(&regex_str, |_caps: &Captures| {
                    format!(" ({}) ", sub_regex)
                }).to_string()
            }
        };
    }
    regex_str = regex_str.replace(" ", "");
    return Some(regex_str);
}