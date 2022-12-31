use std::fs;
use std::str;
use std::time::Instant;

use md5::{Md5, Digest};

const PROBLEM_NAME: &str = "The Ideal Stocking Stuffer";
const PROBLEM_INPUT_FILE: &str = "./input/day04.txt";
const PROBLEM_DAY: u64 = 4;

/// Processes the AOC 2015 Day 04 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 04 input file in the format required by the solver functions.
/// Returned value is String given in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    String::from(raw_input.trim())
}

/// Solves AOC 2015 Day 04 Part 1 // Determines the lowest number that results in an MD5 hash
/// starting with five zeroes when post-fixed to the secret key.
fn solve_part1(secret_key: &str) -> u64 {
    let mut serial: u64 = 1;
    loop {
        let hex_result = calculate_md5_hex_result(secret_key, serial);
        if hex_result.starts_with("00000") {
            return serial;
        }
        serial += 1;
    }
}

/// Solves AOC 2015 Day 04 Part 2 // ###
fn solve_part2(_secret_key: &str) -> u64 {
    0
}

/// Calculates the hexadecimal string representation of the MD5 hash from the concatenation of the
/// secret key and serial number.
fn calculate_md5_hex_result(secret_key: &str, serial: u64) -> String {
    let input = format!("{}{}", secret_key, serial);
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let mut hex_output = String::new();
    for b in result {
        hex_output.push_str(&format!("{:02x}", b));
    }
    hex_output
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 04 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day04_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(254575, solution);
    }

    /// Tests the Day 04 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day04_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1038736, solution);
    }
}
