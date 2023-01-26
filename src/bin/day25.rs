use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Let It Snow";
const PROBLEM_INPUT_FILE: &str = "./input/day25.txt";
const PROBLEM_DAY: u64 = 25;

/// Processes the AOC 2015 Day 25 input file and solves both parts of the problem. Solutions are
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
    // Print results
    println!("==================================================");
    println!("AOC 2015 Day {} - \"{}\"", PROBLEM_DAY, PROBLEM_NAME);
    println!("[+] Part 1: {}", p1_solution);
    println!("[+] Part 2: [CHRISTMAS IS SAVED!]");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {:.2?}", input_parser_duration);
    println!("[+] Part 1: {:.2?}", p1_duration);
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2015 Day 25 input file into the format required by the solver functions.
/// Returned value is ###.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let _raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    unimplemented!();
}

/// Solves AOC 2015 Day 25 Part 1 // ###
fn solve_part1(_input: &String) -> u64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 25 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day25_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(19980801, solution);
    }
}
