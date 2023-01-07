use std::fs;
use std::time::Instant;

use fancy_regex::Regex;

use aoc2015::utils::bespoke::Reindeer;

const PROBLEM_NAME: &str = "Reindeer Olympics";
const PROBLEM_INPUT_FILE: &str = "./input/day14.txt";
const PROBLEM_DAY: u64 = 14;

const RACE_DURATION: u64 = 2503;

/// Processes the AOC 2015 Day 14 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 14 input file in the format required by the solver functions.
/// Returned value is vector of reindeers as specified in the input file.
fn process_input_file(filename: &str) -> Vec<Reindeer> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut reindeers: Vec<Reindeer> = vec![];
    let regex_line = Regex::new(
        r"^.* can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$",
    )
    .unwrap();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(Some(caps)) = regex_line.captures(line) {
            let speed = caps[1].parse::<u64>().unwrap();
            let duration_travel = caps[2].parse::<u64>().unwrap();
            let duration_rest = caps[3].parse::<u64>().unwrap();
            reindeers.push(Reindeer::new(speed, duration_travel, duration_rest));
        } else {
            panic!("Bad format input line! // {line}");
        }
    }
    reindeers
}

/// Solves AOC 2015 Day 14 Part 1 // Determines the furthest distance travelled by a reindeer
/// during the race.
fn solve_part1(reindeers: &[Reindeer]) -> u64 {
    reindeers.iter().map(|r| r.distance_travelled_in_period(RACE_DURATION)).max().unwrap()
}

/// Solves AOC 2015 Day 14 Part 2 // ###
fn solve_part2(_input: &[Reindeer]) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 14 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day14_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(2640, solution);
    }

    /// Tests the Day 14 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day14_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1102, solution);
    }
}
