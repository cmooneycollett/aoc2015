use std::fs;
use std::time::Instant;

use aoc2015::utils::bespoke::Present;

const PROBLEM_NAME: &str = "I Was Told There Would Be No Math";
const PROBLEM_INPUT_FILE: &str = "./input/day02.txt";
const PROBLEM_DAY: u64 = 2;

/// Processes the AOC 2015 Day 02 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 02 input file into the format required by the solver functions.
/// Returned value is vector of Present structs specified by the input file lines.
fn process_input_file(filename: &str) -> Vec<Present> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut presents: Vec<Present> = vec![];
    for line in raw_input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let dims = line
            .split('x')
            .map(|elem| elem.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        presents.push(Present::new(dims[0], dims[1], dims[2]));
    }
    presents
}

/// Solves AOC 2015 Day 02 Part 1 // Calculates the total amount of wrapping paper (in square feet)
/// needed to wrap all the presents.
fn solve_part1(presents: &[Present]) -> u64 {
    presents.iter().map(|pres| pres.paper_needed()).sum()
}

/// Solves AOC 2015 Day 02 Part 2 // Calculates the total amount of ribbon needed (in feet) to wrap
/// all the presents.
fn solve_part2(presents: &[Present]) -> u64 {
    presents.iter().map(|pres| pres.ribbon_needed()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 02 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day02_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1588178, solution);
    }

    /// Tests the Day 02 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day02_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(3783758, solution);
    }
}
