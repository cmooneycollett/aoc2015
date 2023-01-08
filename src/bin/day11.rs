use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Corporate Policy";
const PROBLEM_INPUT_FILE: &str = "./input/day11.txt";
const PROBLEM_DAY: u64 = 11;

lazy_static! {
    static ref REGEX_TRIPLE: Regex =
        Regex::new(r"(abc|bcd|cde|def|efg|fgh|pqr|qrs|rst|stu|tuv|uvw|vwx|wxy|xyz)").unwrap();
    static ref REGEX_PAIR: Regex = Regex::new(r"([a-z])\1.*([a-z])\2").unwrap();
    static ref REGEX_BAD: Regex = Regex::new(r"(i|l|o)").unwrap();
}

/// Password struct used to generate valid passwords based on the correctness rules.
pub struct PasswordGenerator {
    chars: Vec<char>,
}

impl PasswordGenerator {
    pub fn new(chars: &[char]) -> PasswordGenerator {
        PasswordGenerator {
            chars: chars.to_vec(),
        }
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chars.is_empty() {
            self.chars = vec!['a'];
        }
        loop {
            let mut i = self.chars.len() - 1;
            // Increment chars to find the next candidate password
            loop {
                // Increment password
                if self.chars[i] != 'z' {
                    self.chars[i] = char::from(self.chars[i] as u8 + 1);
                    break;
                } else {
                    self.chars[i] = 'a';
                    if i == 0 {
                        self.chars.insert(0, 'a');
                        break;
                    } else {
                        i -= 1;
                    }
                }
            }
            // Check if password is valid
            let candidate = self.chars.iter().collect::<String>();
            if let Ok(Some(caps)) = REGEX_PAIR.captures(&candidate) {
                if caps[1] == caps[2] {
                    continue;
                }
            } else {
                continue;
            }
            if REGEX_TRIPLE.is_match(&candidate).unwrap()
                && !REGEX_BAD.is_match(&candidate).unwrap()
            {
                return Some(candidate);
            }
        }
    }
}

/// Processes the AOC 2015 Day 11 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 11 input file into the format required by the solver functions.
/// Returned value is password seed string given in input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().to_string()
}

/// Solves AOC 2015 Day 11 Part 1 // Gets the next valid password from the current seed password.
fn solve_part1(seed: &str) -> String {
    let mut password_gen = PasswordGenerator::new(&seed.chars().collect::<Vec<char>>());
    password_gen.next().unwrap()
}

/// Solves AOC 2015 Day 11 Part 2 // Gets the second next valid password from the current seed
/// password.
fn solve_part2(seed: &str) -> String {
    let mut password_gen = PasswordGenerator::new(&seed.chars().collect::<Vec<char>>());
    password_gen.next();
    password_gen.next().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 11 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day11_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(String::from("hepxxyzz"), solution);
    }

    /// Tests the Day 11 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day11_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(String::from("heqaabcc"), solution);
    }
}
