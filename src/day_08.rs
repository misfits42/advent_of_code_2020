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
    handheld_console.execute();
    return handheld_console.gen_accumulator();
}
