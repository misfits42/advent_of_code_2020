use std::collections::HashSet;

#[aoc_generator(day6)]
fn generate_input(input: &str) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = vec![];
    let mut lines = input.lines();
    loop {
        let mut end_of_file = false;
        let mut group: Vec<String> = vec![];
        loop {
            let line = lines.next();
            if line.is_none() {
                end_of_file = true;
                break;
            }
            let line = line.unwrap().trim();
            // Check if reached end of current group, but not EOF yet
            if line.is_empty() {
                break;
            }
            group.push(line.to_string());
        }
        groups.push(group);
        if end_of_file {
            return groups;
        }
    }
}

#[aoc(day6, part1)]
fn solve_part_1(groups: &Vec<Vec<String>>) -> usize {
    let mut yes_total = 0;
    for group in groups {
        let mut unique_question: HashSet<char> = HashSet::new();
        for entry in group {
            for c in entry.chars() {
                unique_question.insert(c);
            }
        }
        yes_total += unique_question.len();
    }
    return yes_total;
}

#[aoc(day6, part2)]
fn solve_part_2(groups: &Vec<Vec<String>>) -> usize {
    let mut common_yes_total = 0;
    for group in groups {
        let mut record: HashSet<char> = HashSet::new();
        for i in 0..group.len() {
            let entry_chars = group[i].chars().collect::<HashSet<char>>();
            if i == 0 { // Initialise yes record on first first entry
                record = entry_chars.clone();
            } else {
                // Check which yes questions seen common so far are not present in current entry
                let mut c_to_remove: Vec<char> = vec![];
                for c in record.iter() {
                    if !entry_chars.contains(&c) {
                        c_to_remove.push(*c);
                    }
                }
                for c in c_to_remove {
                    record.remove(&c);
                }
            }
        }
        common_yes_total += record.len();
    }
    return common_yes_total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d06_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day6.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(6714, result);
    }

    #[test]
    fn test_d06_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day6.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(3435, result);
    }
}
