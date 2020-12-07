use std::collections::HashMap;

use regex::Regex;

#[aoc_generator(day7)]
fn generate_input(input: &str) -> HashMap<String, HashMap<String, u64>> {
    let mut bag_rules: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let bag_no_contents_regex = Regex::new(r"^(.*) bags contain no other bags.$").unwrap();
    let bag_with_contents_regex = Regex::new(r"^(.*) bags contain (.*).$").unwrap();
    let inner_bag_regex = Regex::new(r"(\d+) (.*) bag[s]?").unwrap();
    for line in input.lines() {
        // Trim leading and trailing whitespace, then skip empty lines.
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut inner_bags: HashMap<String, u64> = HashMap::new();
        // Match rule for bag containing no other bags
        if bag_no_contents_regex.is_match(line) {
            let captures = bag_no_contents_regex.captures(line).unwrap();
            let bag_type = captures[1].to_string();
            bag_rules.insert(bag_type, inner_bags);
        // Match rule for bag containing other bags
        } else if bag_with_contents_regex.is_match(line) {
            let captures = bag_with_contents_regex.captures(line).unwrap();
            let bag_type = captures[1].to_string();
            // Extract type and quantity for inner bags
            for inner_bag_spec in captures[2].split(", ") {
                let inner_bag_captures = inner_bag_regex.captures(inner_bag_spec).unwrap();
                let quantity = inner_bag_captures[1].parse::<u64>().unwrap();
                let inner_bag_type = inner_bag_captures[2].to_string();
                inner_bags.insert(inner_bag_type, quantity);
            }
            bag_rules.insert(bag_type, inner_bags);
        } else {
            panic!("Day 7 - malformed input file!");
        }
    }
    return bag_rules;
}

#[aoc(day7, part1)]
fn solve_part_1(bag_rules: &HashMap<String, HashMap<String, u64>>) -> u64 {
    let mut count = 0;
    // For each specified bag type, check if it eventually contains a "shiny gold" bag
    for bag_type in bag_rules.keys() {
        if check_for_inner_bag(bag_rules, bag_type, "shiny gold") {
            count += 1;
        }
    }
    return count;
}

#[aoc(day7, part2)]
fn solve_part_2(bag_rules: &HashMap<String, HashMap<String, u64>>) -> u64 {
    return count_inner_bags(bag_rules, "shiny gold");
}

/// Checks recursively if the current bag contains the target bag.
fn check_for_inner_bag(bag_rules: &HashMap<String, HashMap<String, u64>>, current_bag: &str,
        target_bag: &str) -> bool
{
    if !bag_rules.contains_key(current_bag) {
        return false;
    }
    for (inner_bag, _quantity) in bag_rules.get(current_bag).unwrap() {
        // Check if the current inner bag is the target bag or contains the target bag
        if inner_bag == target_bag || check_for_inner_bag(bag_rules, inner_bag, target_bag) {
            return true;
        }
    }
    return false;
}

/// Counts the number of bags contained within the current bag.
fn count_inner_bags(bag_rules: &HashMap<String, HashMap<String, u64>>, current_bag: &str) -> u64 {
    let mut count = 0;
    for (inner_bag, quantity) in bag_rules.get(current_bag).unwrap() {
        // Add the number of inner bags and all bags contained within that number of inner bags
        count += quantity * (1 + count_inner_bags(bag_rules, inner_bag));
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d07_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day7.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(238, result);
    }

    #[test]
    fn test_d07_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day7.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(82930, result);
    }
}
