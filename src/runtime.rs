//use std::any::type_name;
use std::io::{Read, Write, stdin, stdout};

use crate::errs::RuntimeError;
pub struct Runtime {
    tape: [u8; 30000],
    pc: usize,
    dp: usize,
    bracket_map: Vec<usize>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            tape: [0u8; 30000],
            pc: 0,
            dp: 0,
            bracket_map: Vec::new(),
        }
    }
    fn reset(&mut self) {
        self.tape.fill(0);
        self.pc = 0;
        self.dp = 0;
        self.bracket_map.clear();
    }
    fn create_bracket_map(&mut self, code: &[u8]) -> Result<(), RuntimeError> {
        self.bracket_map = vec![0; code.len()];
        let mut stack = Vec::<usize>::new();
        for i in 0..code.len() {
            match code[i] {
                b'[' => stack.push(i),
                b']' => {
                    let open = stack.pop().ok_or(RuntimeError::UnmatchedClosingBracket)?;
                    self.bracket_map[i] = open;
                    self.bracket_map[open] = i;
                }
                _ => {}
            }
        }

        if !stack.is_empty() {
            return Err(RuntimeError::UnmatchedOpenBracket);
        }
        Ok(())
    }
    fn increment_dp(&mut self) {
        self.dp = (self.dp + 1) % self.tape.len();
    }
    fn decrement_dp(&mut self) {
        self.dp = if self.dp == 0 {
            self.tape.len() - 1
        } else {
            self.dp - 1
        };
    }
    fn increment_data(&mut self) {
        self.tape[self.dp] = self.tape[self.dp].wrapping_add(1);
    }
    fn decrement_data(&mut self) {
        self.tape[self.dp] = self.tape[self.dp].wrapping_sub(1);
    }

    pub fn run<T: AsRef<[u8]>>(&mut self, code: T) -> Result<(), RuntimeError> {
        let code_bytes = code.as_ref();

        self.create_bracket_map(code_bytes)?;

        while self.pc < code_bytes.len() {
            // dbg!(self.pc);
            match code_bytes[self.pc] {
                b'>' => self.increment_dp(),
                b'<' => self.decrement_dp(),
                b'.' => {
                    print!("{}", self.tape[self.dp] as char);
                    stdout().flush()?;
                }
                b',' => stdin().read_exact(&mut self.tape[self.dp..=self.dp])?,
                b'[' => {
                    if self.tape[self.dp] == 0 {
                        self.pc = self.bracket_map[self.pc]
                    }
                }
                b']' => {
                    if self.tape[self.dp] != 0 {
                        self.pc = self.bracket_map[self.pc];
                    }
                }
                b'+' => self.increment_data(),
                b'-' => self.decrement_data(),
                _ => {}
            }
            self.pc += 1;
        }
        self.reset();
        Ok(())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
