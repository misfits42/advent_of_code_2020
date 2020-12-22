use std::collections::HashMap;

use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum InputParseState {
    FieldRules,
    OwnTicket,
    NearbyTickets,
}

/// Stores the details captured in the train ticket dossier gathered in AOC 2020 Day 16.
struct TrainTicketDossier {
    field_rules: HashMap<String, Vec<(u64, u64)>>,
    own_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

#[aoc_generator(day16)]
fn generate_input(input: &str) -> TrainTicketDossier {
    let field_rule_regex = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut input_parse_state = InputParseState::FieldRules;
    // Set up variables to store data for train ticket dossier
    let mut field_rules: HashMap<String, Vec<(u64, u64)>> = HashMap::new();
    let mut own_ticket: Vec<u64> = vec![];
    let mut nearby_tickets: Vec<Vec<u64>> = vec![];
    // Process each line
    for line in input.lines() {
        // Check for empty lines - transitioning input parsing state as required
        let line = line.trim();
        if line.is_empty() {
            if input_parse_state == InputParseState::FieldRules {
                input_parse_state = InputParseState::OwnTicket;
            } else if input_parse_state == InputParseState::OwnTicket {
                input_parse_state = InputParseState::NearbyTickets;
            }
            continue;
        }
        // process current line depending on current input parsing state
        match input_parse_state {
            InputParseState::FieldRules => {
                if field_rule_regex.is_match(line) {
                    let captures = field_rule_regex.captures(line).unwrap();
                    let field_name = captures[1].to_string();
                    let range_1_lower = captures[2].parse::<u64>().unwrap();
                    let range_1_upper = captures[3].parse::<u64>().unwrap();
                    let range_2_lower = captures[4].parse::<u64>().unwrap();
                    let range_2_upper = captures[5].parse::<u64>().unwrap();
                    let ranges: Vec<(u64, u64)> = vec![
                        (range_1_lower, range_1_upper),
                        (range_2_lower, range_2_upper),
                    ];
                    field_rules.insert(field_name, ranges);
                } else {
                    panic!("Day 16 - malformed input file. Failed during field rule parsing.");
                }
            }
            InputParseState::OwnTicket => {
                if line == "your ticket:" {
                    continue;
                }
                own_ticket = line
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
            }
            InputParseState::NearbyTickets => {
                if line == "nearby tickets:" {
                    continue;
                }
                let ticket = line
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                nearby_tickets.push(ticket);
            }
        }
    }
    return TrainTicketDossier {
        field_rules: field_rules,
        own_ticket: own_ticket,
        nearby_tickets: nearby_tickets,
    };
}

fn check_value_validity(value: u64, field_rules: &HashMap<String, Vec<(u64, u64)>>) -> bool {
    for (_field_name, rules) in field_rules {
        for rule in rules {
            // Check if current ticket value is invalid
            if value >= rule.0 && value <= rule.1 {
                return true;
            }
        }
    }
    return false;
}

#[aoc(day16, part1)]
fn solve_part_1(train_ticket_dossier: &TrainTicketDossier) -> u64 {
    let mut error_rate = 0;
    // Check each field of all nearby tickets for validity
    for ticket in train_ticket_dossier.nearby_tickets.iter() {
        for value in ticket {
            if !check_value_validity(*value, &train_ticket_dossier.field_rules) {
                error_rate += value;
            }
        }
    }
    return error_rate;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d16_p1_proper() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/day16.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(19060, result);
    }

    #[test]
    fn test_d16_p1_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day16_test_001.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(71, result);
    }
}
