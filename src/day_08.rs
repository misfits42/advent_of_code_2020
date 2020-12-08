use super::utils::machines::HandheldConsole;
use super::utils::machines::HandheldConsoleInstruction;

#[aoc_generator(day8)]
fn generate_input(input: &str) -> Vec<HandheldConsoleInstruction> {
    let instructions = HandheldConsoleInstruction::parse_raw_code(input);
    if instructions.is_none() {
        panic!("Day 8 - malformed input file!");
    }
    return instructions.unwrap();
}

#[aoc(day8, part1)]
fn solve_part_1(instructions: &Vec<HandheldConsoleInstruction>) -> isize {
    let mut handheld_console = HandheldConsole::new(instructions);
    handheld_console.execute(0, true);
    return handheld_console.get_accumulator();
}

#[aoc(day8, part2)]
fn solve_part_2(instructions: &Vec<HandheldConsoleInstruction>) -> isize {
    let mut handheld_console = HandheldConsole::new(instructions);
    loop {
        // Try to execute
        handheld_console.execute(1000, false);
        // Check if the handheld console halted
        if handheld_console.is_halted() {
            return handheld_console.get_accumulator();
        }
        // Check if toggles exhausted
        if handheld_console.check_toggles_exhausted() {
            panic!("Day 8 Part 2 - toggles exhausted!");
        }
        // Did not halt within set number of steps, to toggle next JMP or NOP
        handheld_console.toggle_next_jmp_or_nop();
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
