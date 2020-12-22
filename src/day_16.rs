use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum InputParseState {
    FieldRules,
    OwnTicket,
    NearbyTickets,
}

struct TicketRule {
    ranges: Vec<RangeInclusive<u64>>,
}

impl TicketRule {
    pub fn new() -> Self {
        TicketRule { ranges: vec![] }
    }

    pub fn add_new_range_inclusive(&mut self, lower: u64, upper: u64) {
        self.ranges.push(RangeInclusive::new(lower, upper));
    }

    pub fn check_value_validity(&self, value: u64) -> bool {
        for range in self.ranges.iter() {
            if range.contains(&value) {
                return true;
            }
        }
        return false;
    }
}

/// Stores the details captured in the train ticket dossier gathered in AOC 2020 Day 16.
struct TrainTicketDossier {
    field_rules: HashMap<String, TicketRule>,
    own_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

#[aoc_generator(day16)]
fn generate_input(input: &str) -> TrainTicketDossier {
    let field_rule_regex = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut input_parse_state = InputParseState::FieldRules;
    // Set up variables to store data for train ticket dossier
    let mut field_rules: HashMap<String, TicketRule> = HashMap::new();
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
                    let mut ticket_rule = TicketRule::new();
                    ticket_rule.add_new_range_inclusive(range_1_lower, range_1_upper);
                    ticket_rule.add_new_range_inclusive(range_2_lower, range_2_upper);
                    field_rules.insert(field_name, ticket_rule);
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

/// Checks the validity of the given value against the given record of ticket rules.
fn check_value_validity(value: u64, field_rules: &HashMap<String, TicketRule>) -> bool {
    for (_field_name, rule) in field_rules {
        if rule.check_value_validity(value) {
            return true;
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

#[aoc(day16, part2)]
fn solve_part_2(train_ticket_dossier: &TrainTicketDossier) -> u64 {
    // Determine what nearby tickets are valid
    let mut valid_nearby_tickets: Vec<Vec<u64>> = vec![];
    for ticket in train_ticket_dossier.nearby_tickets.iter() {
        let mut valid = true;
        for value in ticket {
            if !check_value_validity(*value, &train_ticket_dossier.field_rules) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_nearby_tickets.push(ticket.clone());
        }
    }
    // Now determine possible indices for each field
    let mut unknown_indices: HashMap<String, HashSet<usize>> = HashMap::new();
    for field_name in train_ticket_dossier.field_rules.keys() {
        unknown_indices.insert(field_name.clone(), HashSet::new());
    }
    for (field_name, rule) in train_ticket_dossier.field_rules.iter() {
        for i in 0..train_ticket_dossier.own_ticket.len() {
            let mut all_match = true;
            for ticket in valid_nearby_tickets.iter() {
                let value = ticket[i];
                if !rule.check_value_validity(value) {
                    all_match = false;
                    break;
                }
            }
            if all_match {
                unknown_indices.get_mut(field_name).unwrap().insert(i);
            }
        }
    }
    // Now determine the index applicable to each field
    let mut known_indices: HashMap<String, usize> = HashMap::new();
    while !unknown_indices.is_empty() {
        // Find field name and index for field with one and only one associated index
        let (field_name, index) = unknown_indices
            .iter()
            .filter(|(_, indices)| indices.len() == 1)
            .map(|(f, v)| (f.clone(), *v.iter().next().unwrap()))
            .next()
            .unwrap();
        known_indices.insert(field_name.clone(), index);
        unknown_indices.remove(&field_name);
        // Remove the index from the record
        for (_field_name, indices) in unknown_indices.iter_mut() {
            indices.remove(&index);
        }
    }
    // Now determine product of values in own ticket belonging to field starting with "departure"
    let product = known_indices
        .iter()
        .filter(|(f, _i)| f.starts_with("departure"))
        .map(|(_f, i)| train_ticket_dossier.own_ticket[*i])
        .product::<u64>();
    return product;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d16_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day16.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(19060, result);
    }

    #[test]
    fn test_d16_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day16.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(953713095011, result);
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
