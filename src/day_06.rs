use std::collections::HashSet;

#[aoc_generator(day6)]
fn generate_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    let mut groups: Vec<Vec<HashSet<char>>> = vec![];
    let mut lines = input.lines();
    loop {
        let mut end_of_input = false;
        let mut group: Vec<HashSet<char>> = vec![];
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
            group.push(line.chars().collect::<HashSet<char>>());
        }
        groups.push(group);
        if end_of_input {
            return groups;
        }
    }
}

#[aoc(day6, part1)]
fn solve_part_1(groups: &Vec<Vec<HashSet<char>>>) -> usize {
    let mut total_count = 0;
    for group in groups {
        // Record the questions that at least one person in the group answered "yes" to
        let mut group_set: HashSet<char> = HashSet::new();
        for entry in group {
            group_set = group_set.union(&entry).map(|c| *c).collect::<HashSet<char>>();
        }
        total_count += group_set.len();
    }
    return total_count;
}

#[aoc(day6, part2)]
fn solve_part_2(groups: &Vec<Vec<HashSet<char>>>) -> usize {
    let mut total_count = 0; // Recount total of counts across all groups
    for group in groups {
        // Record the questions that everyone in the group answered "yes" to
        let mut overlap: HashSet<char> = HashSet::new();
        for i in 0..group.len() {
            if i == 0 { // Initialise yes record on first group entry
                overlap = overlap.union(&group[i]).map(|c| *c).collect::<HashSet<char>>();
            } else {
                overlap = overlap.intersection(&group[i]).map(|c| *c).collect::<HashSet<char>>();
            }
        }
        total_count += overlap.len();
    }
    return total_count;
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
