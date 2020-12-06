use std::collections::HashSet;

#[aoc_generator(day6)]
fn generate_input(input: &str) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = vec![];
    let mut lines = input.lines();
    loop {
        let mut end_of_input = false;
        let mut group: Vec<String> = vec![];
        loop {
            // Check if end of input reached (no more lines to read)
            let line = lines.next();
            if line.is_none() {
                end_of_input = true;
                break;
            }
            // Check if reached end of current group, but not EOF yet
            let line = line.unwrap().trim();
            if line.is_empty() {
                break;
            }
            group.push(line.to_string());
        }
        groups.push(group);
        if end_of_input {
            return groups;
        }
    }
}

#[aoc(day6, part1)]
fn solve_part_1(groups: &Vec<Vec<String>>) -> usize {
    let mut yes_total = 0;
    for group in groups {
        let mut group_seen_yes: HashSet<char> = HashSet::new();
        for entry in group {
            for c in entry.chars() {
                group_seen_yes.insert(c);
            }
        }
        yes_total += group_seen_yes.len();
    }
    return yes_total;
}

#[aoc(day6, part2)]
fn solve_part_2(groups: &Vec<Vec<String>>) -> usize {
    let mut common_yes_total = 0;
    for group in groups {
        let mut group_common_yes: HashSet<char> = HashSet::new();
        for i in 0..group.len() {
            // Determine the unique characters in the current group entry (form)
            let entry_uniq_chars = group[i].chars().collect::<HashSet<char>>();
            if i == 0 { // Initialise yes record on first group entry
                group_common_yes = entry_uniq_chars.clone();
            } else {
                // Check which yes questions seen common so far are not present in current entry
                let mut c_to_remove: Vec<char> = vec![];
                for c in group_common_yes.iter() {
                    if !entry_uniq_chars.contains(&c) {
                        c_to_remove.push(*c);
                    }
                }
                // Remove elements from group common that were in current entry but not seen yet
                for c in c_to_remove {
                    group_common_yes.remove(&c);
                }
            }
        }
        common_yes_total += group_common_yes.len();
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
