use super::utils::machines::ConwayCube3D;
use super::utils::machines::ConwayCube4D;

#[aoc_generator(day17)]
fn generate_input(input: &str) -> String {
    return input.to_string();
}

#[aoc(day17, part1)]
fn solve_part_1(input: &String) -> u64 {
    let mut conway_cube_3d = ConwayCube3D::new(input);
    for _ in 0..6 {
        conway_cube_3d.conduct_step();
    }
    return conway_cube_3d.count_active_cubes();
}

#[aoc(day17, part2)]
fn solve_part_2(input: &String) -> u64 {
    let mut conway_cube_4d = ConwayCube4D::new(input);
    for _ in 0..6 {
        conway_cube_4d.conduct_step();
    }
    return conway_cube_4d.count_active_cubes();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d17_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day17.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(388, result);
    }

    #[test]
    fn test_d17_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day17.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(2280, result);
    }

    #[test]
    fn test_d17_p1_001() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/test/day17_test_001.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(112, result);
    }

    #[test]
    fn test_d17_p2_001() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/test/day17_test_001.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(848, result);
    }
}
