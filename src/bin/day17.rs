use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "No Such Thing as Too Much";
const PROBLEM_INPUT_FILE: &str = "./input/day17.txt";
const PROBLEM_DAY: u64 = 17;

const TARGET_TOTAL: u64 = 150; // litres

/// Processes the AOC 2015 Day 17 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 17 input file into the format required by the solver functions.
/// Returned value is vector of values given as lines in the problem input file.
fn process_input_file(filename: &str) -> Vec<u64> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

/// Solves AOC 2015 Day 17 Part 1 // Finds the total number of ways to reach the target total of 150
/// from the given values.
fn solve_part1(values: &[u64]) -> u64 {
    let (total, _) = find_subsets_adding_to_total(values);
    total
}

/// Solves AOC 2015 Day 17 Part 2 // ###
fn solve_part2(_values: &[u64]) -> u64 {
    0
}

/// Finds the total number of subsets of the given values that add up to the target total, and the
/// number of ways to reach the target total with the minimum number of terms.
fn find_subsets_adding_to_total(values: &[u64]) -> (u64, u64) {
    let mut container_counts: HashMap<u64, u64> = HashMap::new();
    find_subsets_adding_to_total_recursive(values, 0, 0, 0, &mut container_counts);
    let total: u64 = container_counts.values().sum();
    let min_terms: u64 = *container_counts.keys().min().unwrap();
    let min_terms_count: u64 = *container_counts.get(&min_terms).unwrap();
    (total, min_terms_count)
}

/// Recursive helper function used to find subsets of the given values vector that add up to the
/// target total.
fn find_subsets_adding_to_total_recursive(
    values: &[u64],
    i: usize,
    num_terms: u64,
    running_total: u64,
    container_counts: &mut HashMap<u64, u64>,
) {
    match running_total.cmp(&TARGET_TOTAL) {
        Ordering::Less => {
            if i >= values.len() {
                return;
            }
            // Include the current term
            find_subsets_adding_to_total_recursive(
                values,
                i + 1,
                num_terms + 1,
                running_total + values[i],
                container_counts,
            );
            // Exclude the current term
            find_subsets_adding_to_total_recursive(
                values,
                i + 1,
                num_terms,
                running_total,
                container_counts,
            );
        }
        Ordering::Equal => {
            if let Entry::Vacant(e) = container_counts.entry(num_terms) {
                e.insert(1);
            } else {
                *container_counts.get_mut(&num_terms).unwrap() += 1;
            }
        }
        Ordering::Greater => return,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 17 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day17_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1638, solution);
    }

    /// Tests the Day 17 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day17_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(17, solution);
    }
}
