use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Infinite Elves and Infinite Houses";
const PROBLEM_INPUT_FILE: &str = "./input/day20.txt";
const PROBLEM_DAY: u64 = 20;

/// Processes the AOC 2015 Day 20 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 20 input file into the format required by the solver functions.
/// Returned value is integer value given in the input file.
fn process_input_file(filename: &str) -> usize {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().parse::<usize>().unwrap()
}

/// Solves AOC 2015 Day 20 Part 1 // Determines the lowest number house to get at least as many
/// presents as the target value. (elves delivering 10 presents to each house that is a multiple
/// of the elf number).
fn solve_part1(target: &usize) -> usize {
    let target = *target;
    let mut houses: Vec<usize> = vec![0; target];
    for elf in 1..=target {
        for i in (elf..=target).step_by(elf) {
            houses[i - 1] += elf * 10;
            if i == elf && houses[i - 1] >= target {
                return i;
            }
        }
    }
    panic!("Should not get here!");
}

/// Solves AOC 2015 Day 20 Part 2 // Determines the lowest number house to get at least as many
/// presents as the target value, with each elf visited 50 houses (including their starting house)
/// and delivering 11 presents to each house.
fn solve_part2(target: &usize) -> usize {
    let target = *target;
    let mut houses: Vec<usize> = vec![0; target];
    for elf in 1..target {
        for i in (elf..=target).step_by(elf).take(50) {
            houses[i - 1] += elf * 11;
            if i == elf && houses[i - 1] >= target {
                return i;
            }
        }
    }
    panic!("Should not get here!");
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 20 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day20_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(831600, solution);
    }

    /// Tests the Day 20 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day20_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(884520, solution);
    }
}
