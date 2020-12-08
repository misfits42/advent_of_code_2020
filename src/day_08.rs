use super::utils::machines::HandheldConsole;
use super::utils::machines::HandheldConsoleOp;

#[aoc_generator(day8)]
fn generate_input(input: &str) -> Vec<HandheldConsoleOp> {
    let instructions = HandheldConsoleOp::parse_raw_code(input);
    if instructions.is_none() {
        panic!("Day 8 - malformed input file!");
    }
    return instructions.unwrap();
}

#[aoc(day8, part1)]
fn solve_part_1(instructions: &Vec<HandheldConsoleOp>) -> isize {
    // Execute the instructions on HandheldConsole until one would be executed for second time
    let mut handheld_console = HandheldConsole::new(instructions);
    handheld_console.execute(0, true);
    return handheld_console.get_accumulator();
}

#[aoc(day8, part2)]
fn solve_part_2(instructions: &Vec<HandheldConsoleOp>) -> isize {
    // Try increasing number of steps before number found that results in halt.
    let mut steps = 0;
    loop {
        let mut handheld_console = HandheldConsole::new(instructions);
        loop {
            // Try to execute
            handheld_console.execute(steps, false);
            // Check if the handheld console halted
            if handheld_console.is_halted() {
                return handheld_console.get_accumulator();
            }
            // Check if toggles exhausted
            if handheld_console.check_toggles_exhausted() {
                break;
            }
            // Did not halt within set number of steps, to toggle next JMP or NOP
            handheld_console.toggle_next_jmp_or_nop();
        }
        steps += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d08_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day8.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(2058, result);
    }

    #[test]
    fn test_d08_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day8.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1000, result);
    }
}
