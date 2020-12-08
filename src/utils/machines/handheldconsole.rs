use std::collections::HashMap;

use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum HandheldConsoleInstruction {
    Acc {arg: isize},
    Jmp {arg: isize},
    Nop {arg: isize}
}

impl HandheldConsoleInstruction {
    pub fn toggle(&self) -> Self {
        match self {
            HandheldConsoleInstruction::Acc{arg} => return HandheldConsoleInstruction::Acc{arg: *arg},
            HandheldConsoleInstruction::Jmp{arg} => return HandheldConsoleInstruction::Nop{arg: *arg},
            HandheldConsoleInstruction::Nop{arg} => return HandheldConsoleInstruction::Jmp{arg: *arg},
        }
    }

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
    toggle_index: Option<usize>,
    accumulator: isize,
    halted: bool,
    exec_count: u64,
    prog_counter: isize,
    history: HashMap<usize, Vec<u64>>
}

impl HandheldConsole {
    pub fn new(instructions: &Vec<HandheldConsoleInstruction>) -> Self {
        Self {
            instructions: instructions.clone(),
            toggle_index: Some(0),
            accumulator: 0,
            halted: false,
            exec_count: 0,
            prog_counter: 0,
            history: HashMap::new()
        }
    }

    pub fn toggle_next_jmp_or_nop(&mut self) {
        // Toggle back the last instruction toggled
        if self.toggle_index.unwrap() > 0 {
            self.instructions[self.toggle_index.unwrap() - 1] = self.instructions[self.toggle_index.unwrap() - 1].toggle();
        }
        self.accumulator = 0;
        self.halted = false;
        self.exec_count = 0;
        self.prog_counter = 0;
        self.history = HashMap::new();
        for i in self.toggle_index.unwrap()..self.instructions.len() {
            match self.instructions[i] {
                HandheldConsoleInstruction::Acc{arg: _} => (),
                _ => {
                    self.instructions[i] = self.instructions[i].toggle();
                    self.toggle_index = Some(i + 1);
                    return;
                },
            }
        }
        self.toggle_index = None;
    }

    pub fn is_halted(&self) -> bool {
        return self.halted;
    }

    pub fn check_toggles_exhausted(&self) -> bool {
        return self.toggle_index.is_none();
    }

    pub fn get_accumulator(&self) -> isize {
        return self.accumulator;
    }

    fn execute_single_instruction(&mut self, pause_on_history: bool) {
        if self.halted == true {
            return;
        }
        // Check if program counter is within instruction space
        if self.prog_counter < 0 || self.prog_counter as usize >= self.instructions.len() {
            self.halted = true;
            return;
        }
        // Check if current instruction would be executed for a second time
        if pause_on_history && self.history.contains_key(&(self.prog_counter as usize)) {
            self.halted = true;
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

    /// Executes instructions until an instruction would be executed for a second time (but)
    pub fn execute(&mut self, steps: usize, pause_on_history: bool) {
        let mut step = 0;
        loop {
            if !pause_on_history && step >= steps {
                return;
            }
            if self.halted == true {
                return;
            }
            step += 1;
            self.execute_single_instruction(pause_on_history);
        }
    }
}
