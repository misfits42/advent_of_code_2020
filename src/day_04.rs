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
    CountryID
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
            _ => return None
        }
    }
}

#[aoc_generator(day4)]
fn generate_input(input: &str) -> Vec<HashMap<PassportField, String>> {
    let mut passports: Vec<HashMap<PassportField, String>> = vec![];
    let mut lines = input.lines();
    let field_regex = Regex::new(r"(.*):(.*)").unwrap();
    loop {
        let mut end_of_file = false;
        // Inner loop
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
                let captures = field_regex.captures(pair).unwrap();
                let field = PassportField::from_string(&captures[1]).unwrap();
                let value = captures[2].to_string();
                passport.insert(field, value);
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
    let height_cm_regex = Regex::new(r"(\d+)cm").unwrap();
    let height_in_regex = Regex::new(r"(\d+)in").unwrap();
    let hair_colour_regex = Regex::new(r"#([0-9a-f]{6})").unwrap();
    let eye_colour_regex = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let passport_id_regex = Regex::new(r"([0-9]{9})").unwrap();
    for passport in passports {
        let mut valid = true;
        for field in PassportField::into_enum_iter() {
            // Country ID field is optional, so do not check
            if field == PassportField::CountryID {
                continue;
            }
            // Check if required field if present
            if !passport.contains_key(&field) {
                valid = false;
                break;
            }
            // Now, check the more strict rules for validity
            match field {
                PassportField::BirthYear => {
                    // Get string and check length
                    let val = passport.get(&field).unwrap();
                    if !check_year_string_range(&val, 1920, 2002) {
                        valid = false;
                        break;
                    }
                },
                PassportField::IssueYear => {
                    // Get string and check length
                    let val = passport.get(&field).unwrap();
                    if !check_year_string_range(&val, 2010, 2020) {
                        valid = false;
                        break;
                    }
                },
                PassportField::ExpirationYear => {
                    // Get string and check length
                    let val = passport.get(&field).unwrap();
                    if !check_year_string_range(&val, 2020, 2030) {
                        valid = false;
                        break;
                    }
                },
                PassportField::Height => {
                    let val = passport.get(&field).unwrap();
                    if val.ends_with("cm") {
                        let caps = height_cm_regex.captures(val).unwrap();
                        // Valid line should only have one value
                        if caps.len() == 2 {
                            let val = caps[1].parse::<u64>().unwrap();
                            if val < 150 || val > 193 {
                                valid = false;
                                break;
                            }
                        } else {
                            valid = false;
                            break;
                        }
                    } else if val.ends_with("in") {
                        let caps = height_in_regex.captures(val).unwrap();
                        // Valid line should only have one value
                        if caps.len() == 2 {
                            let val = caps[1].parse::<u64>().unwrap();
                            if val < 59 || val > 76 {
                                valid = false;
                                break;
                            }
                        } else {
                            valid = false;
                            break;
                        }
                    } else {
                        valid = false;
                        break;
                    }
                },
                PassportField::HairColour => {
                    let val = passport.get(&field).unwrap();
                    if !hair_colour_regex.is_match(val) || val.len() != 7 {
                        valid = false;
                        break;
                    }
                },
                PassportField::EyeColour => {
                    let val = passport.get(&field).unwrap();
                    if !eye_colour_regex.is_match(val) {
                        valid = false;
                        break;
                    }
                },
                PassportField::PassportID => {
                    let val = passport.get(&field).unwrap();
                    if !passport_id_regex.is_match(val) || val.len() != 9 {
                        valid = false;
                        break;
                    }
                },
                PassportField::CountryID => ()
            }
        }
        if valid {
            valid_count += 1;
        }
    }
    return valid_count;
}

/// Checks if the given input string is a valid year value between (inclusive) the given lower and
/// upper bounds.
fn check_year_string_range(input: &String, lower: u64, upper: u64) -> bool {
    if input.len() != 4 {
        return false;
    }
    // Convert string to int and check if it falls outside of valid range
    let val = input.parse::<u64>();
    if val.is_err() {
        return false;
    }
    let val = val.unwrap();
    if val < lower || val > upper {
        return false;
    }
    return true;
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
