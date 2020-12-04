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
    unimplemented!();
}
