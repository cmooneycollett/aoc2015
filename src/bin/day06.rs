use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Probably a Fire Hazard";
const PROBLEM_INPUT_FILE: &str = "./input/day06.txt";
const PROBLEM_DAY: u64 = 6;

lazy_static! {
    static ref REGEX_LINE: Regex = Regex::new(r"^(.*) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
}

/// Represents the limits (inclusive) of the light field affected by a particular instruction.
struct LightField {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl LightField {
    pub fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

/// Represents the different instructions that apply to the light grid.
enum Instruction {
    TurnOn { field: LightField },
    TurnOff { field: LightField },
    Toggle { field: LightField },
}

/// Processes the AOC 2015 Day 06 input file and solves both parts of the problem. Solutions are
/// printed to stdout.
pub fn main() {
    let start = Instant::now();
    // Input processing
    let input = process_input_file(PROBLEM_INPUT_FILE);
    let input_parser_timestamp = Instant::now();
    let input_parser_duration = input_parser_timestamp.duration_since(start);
    // Solve part 1
    let p1_solution = solve_part1(&input);
    let p1_timestamp = Instant::now();
    let p1_duration = p1_timestamp.duration_since(input_parser_timestamp);
    // Solve part 2
    let p2_solution = solve_part2(&input);
    let p2_timestamp = Instant::now();
    let p2_duration = p2_timestamp.duration_since(p1_timestamp);
    // Print results
    println!("==================================================");
    println!("AOC 2015 Day {} - \"{}\"", PROBLEM_DAY, PROBLEM_NAME);
    println!("[+] Part 1: {}", p1_solution);
    println!("[+] Part 2: {}", p2_solution);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {:.2?}", input_parser_duration);
    println!("[+] Part 1: {:.2?}", p1_duration);
    println!("[+] Part 2: {:.2?}", p2_duration);
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2015 Day 06 input file in the format required by the solver functions.
/// Returned value is vector of instructions specified by the lines of the input file.
fn process_input_file(filename: &str) -> Vec<Instruction> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut instructions: Vec<Instruction> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(Some(caps)) = REGEX_LINE.captures(line) {
            process_regex_captures(caps, &mut instructions);
        } else {
            panic!("Day06 - bad format input line: {}", line);
        }
    }
    instructions
}

/// Processes the regex captures and adds the corresponding Instruction to the output record.
fn process_regex_captures(caps: fancy_regex::Captures, instructions: &mut Vec<Instruction>) {
    let min_x = caps[2].parse::<usize>().unwrap();
    let min_y = caps[3].parse::<usize>().unwrap();
    let max_x = caps[4].parse::<usize>().unwrap();
    let max_y = caps[5].parse::<usize>().unwrap();
    let field = LightField::new(min_x, max_x, min_y, max_y);
    let instruct = match &caps[1] {
        "turn on" => Instruction::TurnOn { field },
        "turn off" => Instruction::TurnOff { field },
        "toggle" => Instruction::Toggle { field },
        _ => panic!("Day06 - bad instruction: {}", &caps[1]),
    };
    instructions.push(instruct);
}

/// Solves AOC 2015 Day 06 Part 1 // Determines how many lights are left on in the 1000x1000 light
/// grid after all instructions have been processed (with all lights starting as off).
fn solve_part1(instructions: &[Instruction]) -> usize {
    let mut lightgrid: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    for instruct in instructions {
        match instruct {
            Instruction::TurnOn { field } => {
                for y in field.min_y..=field.max_y {
                    for x in field.min_x..=field.max_x {
                        lightgrid[y][x] = true;
                    }
                }
            }
            Instruction::TurnOff { field } => {
                for y in field.min_y..=field.max_y {
                    for x in field.min_x..=field.max_x {
                        lightgrid[y][x] = false;
                    }
                }
            }
            Instruction::Toggle { field } => {
                for y in field.min_y..=field.max_y {
                    for x in field.min_x..=field.max_x {
                        lightgrid[y][x] = !lightgrid[y][x];
                    }
                }
            }
        }
    }
    lightgrid
        .iter()
        .map(|line| line.iter().filter(|elem| **elem).count())
        .sum()
}

/// Solves AOC 2015 Day 06 Part 2 // ###
fn solve_part2(_instructions: &[Instruction]) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 06 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day06_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(377891, solution);
    }

    /// Tests the Day 06 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day06_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(14110788, solution);
    }
}
