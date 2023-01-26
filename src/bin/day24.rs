use core::panic;
use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "It Hangs in the Balance";
const PROBLEM_INPUT_FILE: &str = "./input/day24.txt";
const PROBLEM_DAY: u64 = 24;

/// Processes the AOC 2015 Day 24 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 24 input file into the format required by the solver functions.
/// Returned value is vector of values given in the input file.
fn process_input_file(filename: &str) -> Vec<u128> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|value| value.parse::<u128>().unwrap())
        .collect::<Vec<u128>>()
}

/// Solves AOC 2015 Day 24 Part 1 // Finds the minimum quantum entanglement value for the first
/// compartment (the one with the fewest possible number of presents) where there is a total of
/// three compartments.
fn solve_part1(values: &[u128]) -> u128 {
    let target: u128 = values.iter().sum::<u128>() / 3;
    if let Some(qe) = find_compartment1_qe(values, target) {
        return qe;
    }
    panic!("Could not determine the compartment 1 QE value!");
}

/// Solves AOC 2015 Day 24 Part 2 // Finds the minimum quantum entanglement value for the first
/// compartment (the one with the fewest possible number of presents) where these is a total of four
/// compartments.
fn solve_part2(values: &[u128]) -> u128 {
    let target: u128 = values.iter().sum::<u128>() / 4;
    if let Some(qe) = find_compartment1_qe(values, target) {
        return qe;
    }
    panic!("Could not determine the compartment 1 QE value!");
}

/// Finds the quantum entanglement value for the first compartment (the one with the fewest possible
/// number of presents).
fn find_compartment1_qe(values: &[u128], target: u128) -> Option<u128> {
    let mut comp1_qe: Option<u128> = None;
    let mut min_len: Option<usize> = None;
    find_compartment1_qe_recursive(values, target, &[], 0, 0, &mut comp1_qe, &mut min_len);
    comp1_qe
}

/// Recursive helper function to find the quantum entanglement value for the first compartment (the
/// one with the fewest possible number of presents).
fn find_compartment1_qe_recursive(
    values: &[u128],
    target: u128,
    picked: &[u128],
    running_total: u128,
    index: usize,
    comp1_qe: &mut Option<u128>,
    min_len: &mut Option<usize>,
) {
    // Check if the compartment total equals the target
    if running_total == target {
        if min_len.is_none() || min_len.unwrap() > picked.len() {
            *min_len = Some(picked.len());
            *comp1_qe = Some(picked.iter().product());
        } else if min_len.unwrap() == picked.len() {
            let qe: u128 = picked.iter().product();
            if comp1_qe.is_none() || comp1_qe.unwrap() > qe {
                *comp1_qe = Some(qe);
            }
        }
        return;
    }
    // Check if the target or values space have been overshot
    if running_total > target || index >= values.len() {
        return;
    }
    // Select the value at the current index
    let mut new_picked = picked.to_vec();
    new_picked.push(values[index]);
    // Select
    find_compartment1_qe_recursive(
        values,
        target,
        &new_picked,
        running_total + values[index],
        index + 1,
        comp1_qe,
        min_len,
    );
    // No select
    find_compartment1_qe_recursive(
        values,
        target,
        picked,
        running_total,
        index + 1,
        comp1_qe,
        min_len,
    );
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 24 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day24_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(11846773891, solution);
    }

    /// Tests the Day 24 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day24_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(80393059, solution);
    }
}
