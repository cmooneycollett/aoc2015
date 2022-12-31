use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Not Quite Lisp";
const PROBLEM_INPUT_FILE: &str = "./input/day01.txt";
const PROBLEM_DAY: u64 = 1;

/// Processes the AOC 2015 Day 1 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 1 input file in the format required by the solver functions.
/// Returned value is vector of +1 (open parenthesis: '(') or -1 (closed parenthesis: ')') values
/// determined by the chars given in the input file.
fn process_input_file(filename: &str) -> Vec<i64> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().chars().map(|c| if c == '(' {1} else {-1}).collect::<Vec<i64>>()
}

/// Solves AOC 2015 Day 1 Part 1 // Determines the resulting floor by navigating up and down floors
/// accourding to the input characters (starting on floor 0).
fn solve_part1(input: &[i64]) -> i64 {
    input
        .iter()
        .sum::<i64>()
}

/// Solves AOC 2015 Day 1 Part 2 // Determines the index of the first movement that results in the
/// basement floor being reached.
fn solve_part2(input: &[i64]) -> usize {
    let mut floor = 0;
    for (i, value) in input.iter().enumerate() {
        floor += value;
        if floor == -1 {
            return i + 1;
        }
    }
    panic!("Day 1 Part 2: did not reach the basement floor (-1)!");
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 1 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day01_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(232, solution);
    }

    /// Tests the Day 1 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day01_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1783, solution);
    }
}
