#[aoc_generator(day10)]
fn generate_input(input: &str) -> Vec<u64> {
    let mut adapters = input.lines().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    adapters.sort();
    return adapters;
}

#[aoc(day10, part1)]
fn solve_part_1(adapters: &Vec<u64>) -> u64 {
    let mut total_diff_1 = 0;
    let mut total_diff_3 = 0;
    for i in 0..adapters.len()+1 {
        let prev = {
            if i == 0 {
                0
            } else {
                adapters[i - 1]
            }
        };
        let current = {
            if i < adapters.len() {
                adapters[i]
            } else {
                adapters[adapters.len() - 1] + 3
            }
        };
        let diff = current - prev;
        if diff == 1 {
            total_diff_1 += 1;
        } else if diff == 3 {
            total_diff_3 += 1;
        }
    }
    return total_diff_1 * total_diff_3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d10_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day10.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(2170, result);
    }
}
