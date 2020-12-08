use std::collections::HashMap;

use regex::Regex;

/// Represents the different operations that can be executed by the handheld console specified in
/// AoC 2020 Day 8.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum HandheldConsoleOp {
    Acc {arg: isize},
    Jmp {arg: isize},
    Nop {arg: isize}
}

impl HandheldConsoleOp {
    /// Toggles Jmp instructions to Nop, and vice versa. Argument values and Acc instructions are
    /// unchanged.
    pub fn toggle(&self) -> Self {
        match self {
            HandheldConsoleOp::Acc{arg} => return HandheldConsoleOp::Acc{arg: *arg},
            HandheldConsoleOp::Jmp{arg} => return HandheldConsoleOp::Nop{arg: *arg},
            HandheldConsoleOp::Nop{arg} => return HandheldConsoleOp::Jmp{arg: *arg},
        }
    }

    /// Parses the provided input for instructions, assuming that each instruction is on its own
    /// line.
    pub fn parse_raw_code(input: &str) -> Option<Vec<HandheldConsoleOp>> {
        let mut instructions: Vec<HandheldConsoleOp> = vec![];
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
                instructions.push(HandheldConsoleOp::Acc{arg: arg});
            } else if jmp_regex.is_match(line) {
                let captures = jmp_regex.captures(line).unwrap();
                let arg = captures[1].parse::<isize>().unwrap();
                instructions.push(HandheldConsoleOp::Jmp{arg: arg});
            } else if nop_regex.is_match(line) {
                let captures = nop_regex.captures(line).unwrap();
                let arg = captures[1].parse::<isize>().unwrap();
                instructions.push(HandheldConsoleOp::Nop{arg: arg});
            } else {
                return None;
            }
        }
        return Some(instructions);
    }
}

/// Represents the handheld console specified in AoC 2020 Day 8.
pub struct HandheldConsole {
    instructions: Vec<HandheldConsoleOp>,
    toggle_i: usize,
    accumulator: isize,
    halted: bool,
    exec_count: u64,
    prog_counter: isize,
    history: HashMap<usize, Vec<u64>>
}

impl HandheldConsole {
    pub fn new(instructions: &Vec<HandheldConsoleOp>) -> Self {
        Self {
            instructions: instructions.clone(),
            toggle_i: 0,
            accumulator: 0,
            halted: false,
            exec_count: 0,
            prog_counter: 0,
            history: HashMap::new()
        }
    }

    /// Toggles back the last instruction that was toggled, resets fields to enable next attempt
    /// at executing the loaded boot code and toggles the next Jmp or Nop instruction.
    pub fn toggle_next_jmp_or_nop(&mut self) {
        // Toggle back the last instruction toggled
        if self.toggle_i > 0 {
            self.instructions[self.toggle_i - 1] = self.instructions[self.toggle_i - 1].toggle();
        }
        // Reset HandheldConsole fields to enable reattempt at executing the loaded boot code
        self.accumulator = 0;
        self.halted = false;
        self.exec_count = 0;
        self.prog_counter = 0;
        self.history = HashMap::new();
        // Find the next operation to be toggled and conduct the toggle
        for i in self.toggle_i..self.instructions.len() {
            match self.instructions[i] {
                HandheldConsoleOp::Acc{arg: _} => (),
                _ => {
                    self.instructions[i] = self.instructions[i].toggle();
                    self.toggle_i = i + 1;
                    return;
                },
            }
        }
        // No more instructions left to toggle
        self.toggle_i = self.instructions.len();
    }

    /// Checks if the HandheldConsole has been halted.
    pub fn is_halted(&self) -> bool {
        return self.halted;
    }

    /// Checks if the HandheldConsole has any more Jmp or Nop instructions that have not yet been
    /// toggled.
    pub fn check_toggles_exhausted(&self) -> bool {
        return self.toggle_i >= self.instructions.len();
    }

    /// Gets the value of the HandheldConsole accumulator.
    pub fn get_accumulator(&self) -> isize {
        return self.accumulator;
    }

    /// Executes the next instruction, halting if the instruction has already been executed (if
    /// option specified).
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
            HandheldConsoleOp::Acc{arg} => {
                self.accumulator += arg;
                self.prog_counter += 1;
            },
            HandheldConsoleOp::Jmp{arg} => {
                self.prog_counter += arg;
            },
            HandheldConsoleOp::Nop{arg: _} => {
                self.prog_counter += 1;
            }
        }
    }

    /// Executes instructions specified number of times, halting the HandheldConsole if an
    /// instruction would be executed twice if required.
    pub fn execute(&mut self, steps: usize, pause_on_history: bool) {
        let mut step = 0;
        loop {
            // Check if HandheldConsole execution
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
