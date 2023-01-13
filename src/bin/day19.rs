use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Medicine for Rudolph";
const PROBLEM_INPUT_FILE: &str = "./input/day19.txt";
const PROBLEM_DAY: u64 = 19;

/// Type definition to simplify signature of input file parser and solver functions.
type ProblemInput = (HashMap<String, Vec<String>>, String);

/// Processes the AOC 2015 Day 19 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 19 input file into the format required by the solver functions.
/// Returned value is tuple containing: hashmap of input molecules mapped to possible replacement
/// molecures, and the target molecule.
fn process_input_file(filename: &str) -> ProblemInput {
    // Read contents of problem input file
    let _raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    unimplemented!();
}

/// Solves AOC 2015 Day 19 Part 1 // ###
fn solve_part1(_input: &ProblemInput) -> usize {
    unimplemented!();
}

/// Solves AOC 2015 Day 19 Part 2 // ###
fn solve_part2(_input: &ProblemInput) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 19 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day19_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(518, solution);
    }

    /// Tests the Day 19 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day19_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(200, solution);
    }
}
