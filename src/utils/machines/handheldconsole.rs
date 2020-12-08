use std::collections::HashMap;

use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum HandheldConsoleInstruction {
    Acc {arg: isize},
    Jmp {arg: isize},
    Nop {arg: isize}
}

impl HandheldConsoleInstruction {
    /// Parses the provided input for instructions, assuming that each instruction is on its own
    /// line.
    pub fn parse_raw_code(input: &str) -> Option<Vec<HandheldConsoleInstruction>> {
        let mut instructions: Vec<HandheldConsoleInstruction> = vec![];
        let acc_regex = Regex::new(r"acc ([-|+]\d+)").unwrap();
        let jmp_regex = Regex::new(r"jmp ([-|+]\d+)").unwrap();
        let nop_regex = Regex::new(r"nop ([-|+]\d+)").unwrap();
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if acc_regex.is_match(line) {
                let captures = acc_regex.captures(line).unwrap();
                let arg = captures[1].parse::<isize>().unwrap();
                instructions.push(HandheldConsoleInstruction::Acc{arg: arg});
            } else if jmp_regex.is_match(line) {
                let captures = jmp_regex.captures(line).unwrap();
                let arg = captures[1].parse::<isize>().unwrap();
                instructions.push(HandheldConsoleInstruction::Jmp{arg: arg});
            } else if nop_regex.is_match(line) {
                let captures = nop_regex.captures(line).unwrap();
                let arg = captures[1].parse::<isize>().unwrap();
                instructions.push(HandheldConsoleInstruction::Nop{arg: arg});
            } else {
                return None;
            }
        }
        return Some(instructions);
    }
}

pub struct HandheldConsole {
    instructions: Vec<HandheldConsoleInstruction>,
    accumulator: isize,
    exec_count: u64,
    prog_counter: isize,
    history: HashMap<usize, Vec<u64>>
}

impl HandheldConsole {
    pub fn new(instructions: &Vec<HandheldConsoleInstruction>) -> Self {
        Self {
            instructions: instructions.clone(),
            accumulator: 0,
            exec_count: 0,
            prog_counter: 0,
            history: HashMap::new()
        }
    }

    pub fn gen_accumulator(&self) -> isize {
        return self.accumulator;
    }

    /// Executes instructions until an instruction would be executed for a second time (but)
    pub fn execute(&mut self) {
        loop {
            // Check if program counter is within instruction space
            if self.prog_counter < 0 || self.prog_counter as usize >= self.instructions.len() {
                return;
            }
            // Check if current instruction would be executed for a second time
            if self.history.contains_key(&(self.prog_counter as usize)) {
                return;
            }
            // Add program counter to history
            self.history.insert(self.prog_counter as usize, vec![self.exec_count]);
            // Now execute the instruction
            match self.instructions[self.prog_counter as usize] {
                HandheldConsoleInstruction::Acc{arg} => {
                    self.accumulator += arg;
                    self.prog_counter += 1;
                },
                HandheldConsoleInstruction::Jmp{arg} => {
                    self.prog_counter += arg;
                },
                HandheldConsoleInstruction::Nop{arg: _} => {
                    self.prog_counter += 1;
                }
            }
        }
    }
}
