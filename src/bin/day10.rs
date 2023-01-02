use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Elves Look, Elves Say";
const PROBLEM_INPUT_FILE: &str = "./input/day10.txt";
const PROBLEM_DAY: u64 = 10;

const PART1_ITERATIONS: u64 = 40;
const PART2_ITERATIONS: u64 = 50;

/// Processes the AOC 2015 Day 10 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 10 input file in the format required by the solver functions.
/// Returned value is vector of chars given in the input file.
fn process_input_file(filename: &str) -> Vec<char> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().chars().collect::<Vec<char>>()
}

/// Solves AOC 2015 Day 10 Part 1 // Determines the length of the character sequence after applying
/// 40 iterations of the "look-and-say" transformation.
fn solve_part1(seq: &[char]) -> usize {
    apply_lookandsay(seq, PART1_ITERATIONS)
}

/// Solves AOC 2015 Day 10 Part 2 // Determines the length of the character sequence after applying
/// 50 iterations of the "look-and-say" transformation.
fn solve_part2(seq: &[char]) -> usize {
    apply_lookandsay(seq, PART2_ITERATIONS)
}

/// Determines the length of the character sequence resulting from applying N iterations of the
/// look-and-say transformation.
fn apply_lookandsay(seq: &[char], n: u64) -> usize {
    let mut seq_transform = seq.to_vec();
    for _ in 0..n {
        let mut seq_new: Vec<char> = vec![];
        let mut i_left: usize = 0;
        let mut i_right: usize = 0;
        while i_left < seq_transform.len() {
            // Find end of current run of numbers
            while i_right < seq_transform.len() {
                if seq_transform[i_right] == seq_transform[i_left] {
                    i_right += 1;
                } else {
                    break;
                }
            }
            // Update the new sequence based on the current run of characters
            seq_new.extend((i_right - i_left).to_string().chars());
            seq_new.push(seq_transform[i_left]);
            // Move to next run of characters
            i_left = i_right;
        }
        // Update the character sequence
        seq_transform = seq_new;
    }
    seq_transform.len()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 10 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day10_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(329356, solution);
    }

    /// Tests the Day 10 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day10_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(4666278, solution);
    }
}
