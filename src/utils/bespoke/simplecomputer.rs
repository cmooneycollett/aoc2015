use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_HLF: Regex = Regex::new(r"^hlf (a|b)$").unwrap();
    static ref REGEX_TPL: Regex = Regex::new(r"^tpl (a|b)$").unwrap();
    static ref REGEX_INC: Regex = Regex::new(r"^inc (a|b)$").unwrap();
    static ref REGEX_JMP: Regex = Regex::new(r"^jmp ([-+]?\d+)$").unwrap();
    static ref REGEX_JIE: Regex = Regex::new(r"^jie (a|b), ([-+]?\d+)$").unwrap();
    static ref REGEX_JIO: Regex = Regex::new(r"^jio (a|b), ([-+]?\d+)$").unwrap();
}

/// Represents the different instructions that can be executed by the simple computer.
#[derive(Clone, Copy)]
enum Instruction {
    Half { register: char },                      // hlf
    Triple { register: char },                    // tpl
    Increment { register: char },                 // inc
    Jump { offset: isize },                       // jmp
    JumpIfEven { register: char, offset: isize }, // jie
    JumpIfOne { register: char, offset: isize },  // jio
}

/// Represents the simple computer used in AOC 2015 Day 23 (https://adventofcode.com/2015/day/23).
#[derive(Clone)]
pub struct SimpleComputer {
    instructions: Vec<Instruction>,
    pc: isize, // program counter
    register_a: isize,
    register_b: isize,
}

impl SimpleComputer {
    pub fn new(raw_input: &str, register_a: isize, register_b: isize) -> Option<SimpleComputer> {
        let mut instructions: Vec<Instruction> = vec![];
        for line in raw_input.lines() {
            let line = line.trim();
            let instruction = {
                if let Ok(Some(caps)) = REGEX_HLF.captures(line) {
                    let register = caps[1].chars().next().unwrap();
                    Instruction::Half { register }
                } else if let Ok(Some(caps)) = REGEX_TPL.captures(line) {
                    let register = caps[1].chars().next().unwrap();
                    Instruction::Triple { register }
                } else if let Ok(Some(caps)) = REGEX_INC.captures(line) {
                    let register = caps[1].chars().next().unwrap();
                    Instruction::Increment { register }
                } else if let Ok(Some(caps)) = REGEX_JMP.captures(line) {
                    let offset = caps[1].parse::<isize>().unwrap();
                    Instruction::Jump { offset }
                } else if let Ok(Some(caps)) = REGEX_JIE.captures(line) {
                    let register = caps[1].chars().next().unwrap();
                    let offset = caps[2].parse::<isize>().unwrap();
                    Instruction::JumpIfEven { register, offset }
                } else if let Ok(Some(caps)) = REGEX_JIO.captures(line) {
                    let register = caps[1].chars().next().unwrap();
                    let offset = caps[2].parse::<isize>().unwrap();
                    Instruction::JumpIfOne { register, offset }
                } else {
                    // Invalid instruction, so the simple computer cannot be created
                    return None;
                }
            };
            instructions.push(instruction);
        }
        Some(SimpleComputer {
            instructions,
            pc: 0,
            register_a,
            register_b,
        })
    }

    /// Gets the value of the "register_a" field.
    pub fn register_a(&self) -> isize {
        self.register_a
    }

    /// Gets the value of the "register_b" field.
    pub fn register_b(&self) -> isize {
        self.register_b
    }

    /// Updates the value of the "register_a" field.
    pub fn set_register_a(&mut self, new_value: isize) {
        self.register_a = new_value;
    }

    /// Updates the value of the "register_b" field.
    pub fn set_register_b(&mut self, new_value: isize) {
        self.register_b = new_value;
    }

    /// Executes the instructions stored in the simple computer and halts when the program counter
    /// is outside of the instruction space.
    pub fn execute(&mut self) {
        loop {
            // Halt execution if outside of instruction space
            if self.pc < 0 || self.pc as usize >= self.instructions.len() {
                return;
            }
            // Execute the current instruction
            let pc = self.pc as usize;
            match self.instructions[pc] {
                Instruction::Half { register } => {
                    match register {
                        'a' => self.register_a /= 2,
                        'b' => self.register_b /= 2,
                        _ => (),
                    }
                    self.pc += 1;
                }
                Instruction::Triple { register } => {
                    match register {
                        'a' => self.register_a *= 3,
                        'b' => self.register_b *= 3,
                        _ => (),
                    }
                    self.pc += 1;
                }
                Instruction::Increment { register } => {
                    match register {
                        'a' => self.register_a += 1,
                        'b' => self.register_b += 1,
                        _ => (),
                    }
                    self.pc += 1;
                }
                Instruction::Jump { offset } => self.pc += offset,
                Instruction::JumpIfEven { register, offset } => {
                    let valid = match register {
                        'a' => self.register_a % 2 == 0,
                        'b' => self.register_b % 2 == 0,
                        _ => false,
                    };
                    if valid {
                        self.pc += offset;
                    } else {
                        self.pc += 1;
                    }
                }
                Instruction::JumpIfOne { register, offset } => {
                    let valid = match register {
                        'a' => self.register_a == 1,
                        'b' => self.register_b == 1,
                        _ => false,
                    };
                    if valid {
                        self.pc += offset;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }
}
