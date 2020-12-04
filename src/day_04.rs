use std::collections::HashMap;

use enum_iterator::IntoEnumIterator;
use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, IntoEnumIterator)]
enum PassportField {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColour,
    EyeColour,
    PassportID,
    CountryID,
}

impl PassportField {
    fn from_string(input: &str) -> Option<PassportField> {
        match input {
            "byr" => return Some(PassportField::BirthYear),
            "iyr" => return Some(PassportField::IssueYear),
            "eyr" => return Some(PassportField::ExpirationYear),
            "hgt" => return Some(PassportField::Height),
            "hcl" => return Some(PassportField::HairColour),
            "ecl" => return Some(PassportField::EyeColour),
            "pid" => return Some(PassportField::PassportID),
            "cid" => return Some(PassportField::CountryID),
            _ => return None,
        }
    }
}

#[aoc_generator(day4)]
fn generate_input(input: &str) -> Vec<HashMap<PassportField, String>> {
    let mut passports: Vec<HashMap<PassportField, String>> = vec![];
    let mut lines = input.lines();
    let field_regex = Regex::new(r"(byr|iyr|eyr|hgt|hcl|ecl|pid|cid):(.*)").unwrap();
    // Outer loop - process all blocks representing passports
    loop {
        let mut end_of_file = false;
        // Inner loop - process current passport block
        let mut passport: HashMap<PassportField, String> = HashMap::new();
        loop {
            // Check if end-of-file reached
            let line = lines.next();
            if line.is_none() {
                end_of_file = true;
                break;
            }
            // Check if blank line reached, indicating end of current passport
            let line = line.unwrap().trim();
            if line.is_empty() {
                break;
            }
            // Extract passport fields from current line
            let pairs = line.split(" ");
            for pair in pairs {
                if field_regex.is_match(pair) {
                    let captures = field_regex.captures(pair).unwrap();
                    let field = PassportField::from_string(&captures[1]).unwrap();
                    let value = captures[2].to_string();
                    passport.insert(field, value);
                } else {
                    panic!("Day 4 - malformed input file!");
                }
            }
        }
        passports.push(passport);
        if end_of_file == true {
            break;
        }
    }
    return passports;
}

#[aoc(day4, part1)]
fn solve_part_1(passports: &Vec<HashMap<PassportField, String>>) -> u64 {
    let mut valid_count = 0;
    for passport in passports {
        let mut valid = true;
        for field in PassportField::into_enum_iter() {
            // Country ID field is optional, so do not check
            if field == PassportField::CountryID {
                continue;
            }
            if !passport.contains_key(&field) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_count += 1;
        }
    }
    return valid_count;
}

#[aoc(day4, part2)]
fn solve_part_2(passports: &Vec<HashMap<PassportField, String>>) -> u64 {
    let mut valid_count = 0;
    // Create regexes to help with validity checking
    let byr_regex = Regex::new(r"^19[2-9][0-9]|200[0-2]$").unwrap();
    let iyr_regex = Regex::new(r"^201[0-9]|2020$").unwrap();
    let eyr_regex = Regex::new(r"^202[0-9]|2030$").unwrap();
    let hgt_cm_regex = Regex::new(r"^(1[5-8][0-9]|19[0-3])cm$").unwrap();
    let hgt_in_regex = Regex::new(r"^(59|6[0-9]|7[0-6])in$").unwrap();
    let hcl_regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
    let ecl_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_regex = Regex::new(r"^([0-9]{9})$").unwrap();
    for passport in passports {
        // Assume passport is valid, until proven otherwise
        let mut valid_passport = true;
        for field in PassportField::into_enum_iter() {
            // Country ID field is optional, so do not check
            if field == PassportField::CountryID {
                continue;
            }
            // Check if required field if present
            if !passport.contains_key(&field) {
                valid_passport = false;
                break;
            }
            // Now, check the more strict rules for validity
            let field_raw = passport.get(&field).unwrap();
            let valid_field = match field {
                PassportField::BirthYear => byr_regex.is_match(field_raw),
                PassportField::IssueYear => iyr_regex.is_match(field_raw),
                PassportField::ExpirationYear => eyr_regex.is_match(field_raw),
                PassportField::Height => {
                    hgt_cm_regex.is_match(field_raw) || hgt_in_regex.is_match(field_raw)
                }
                PassportField::HairColour => hcl_regex.is_match(field_raw),
                PassportField::EyeColour => ecl_regex.is_match(field_raw),
                PassportField::PassportID => pid_regex.is_match(field_raw),
                PassportField::CountryID => true, // CountryID field is optional, so ignore
            };
            // Check if current field is valid
            if !valid_field {
                valid_passport = false;
                break;
            }
        }
        // If current passport is valid, increase valid count
        if valid_passport {
            valid_count += 1;
        }
    }
    return valid_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d04_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day4.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(264, result);
    }

    #[test]
    fn test_d04_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day4.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(224, result);
    }
}
