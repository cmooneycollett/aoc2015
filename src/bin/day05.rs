use std::fs;
use std::time::Instant;

use fancy_regex::Regex; // fancy_regex needed for backreferences (not implemented in regex crate)
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Doesn't He Have Intern-Elves For This?";
const PROBLEM_INPUT_FILE: &str = "./input/day05.txt";
const PROBLEM_DAY: u64 = 5;

lazy_static! {
    static ref REGEX_P1_1: Regex = Regex::new(r"^.*[aeiou].*[aeiou].*[aeiou].*$").unwrap();
    static ref REGEX_P1_2: Regex = Regex::new(r"^.*([a-z])\1.*$").unwrap();
    static ref REGEX_P1_3: Regex = Regex::new(r"^.*(ab|cd|pq|xy).*$").unwrap();
}

/// Processes the AOC 2015 Day 05 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 05 input file in the format required by the solver functions.
/// Returned value is vector of strings given as lines in the input file.
fn process_input_file(filename: &str) -> Vec<String> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>()
}

/// Solves AOC 2015 Day 05 Part 1 // Determines how many of the input strings meet the part1
/// niceness rules.
fn solve_part1(input_strings: &[String]) -> usize {
    input_strings
        .iter()
        .filter(|s| check_part1_niceness(s))
        .count()
}

/// Solves AOC 2015 Day 05 Part 2 // ###
fn solve_part2(_input: &[String]) -> usize {
    unimplemented!();
}

/// Checks if the candidate string meets the day05 part1 niceness rules.
fn check_part1_niceness(candidate: &str) -> bool {
    REGEX_P1_1.is_match(candidate).unwrap()
        && REGEX_P1_2.is_match(candidate).unwrap()
        && !REGEX_P1_3.is_match(candidate).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 05 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day05_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(255, solution);
    }

    /// Tests the Day 05 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day05_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(55, solution);
    }
}
