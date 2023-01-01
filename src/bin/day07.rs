use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Some Assembly Required";
const PROBLEM_INPUT_FILE: &str = "./input/day07.txt";
const PROBLEM_DAY: u64 = 7;

lazy_static! {
    static ref REGEX_VALUE: Regex = Regex::new(r"^([a-z]+|\d+) -> ([a-z]+)$").unwrap();
    static ref REGEX_AND: Regex =
        Regex::new(r"^([a-z]+|\d+) AND ([a-z]+|\d+) -> ([a-z]+)$").unwrap();
    static ref REGEX_LSHIFT: Regex =
        Regex::new(r"^([a-z]+|\d+) LSHIFT ([a-z]+|\d+) -> ([a-z]+)$").unwrap();
    static ref REGEX_RSHIFT: Regex =
        Regex::new(r"^([a-z]+|\d+) RSHIFT ([a-z]+|\d+) -> ([a-z]+)$").unwrap();
    static ref REGEX_NOT: Regex = Regex::new(r"^NOT ([a-z]+|\d+) -> ([a-z]+)$").unwrap();
    static ref REGEX_OR: Regex =
        Regex::new(r"^([a-z]+|\d+) OR ([a-z]+|\d+) -> ([a-z]+|\d+)$").unwrap();
}

/// Represents the different operations observed in the problem.
#[derive(Clone, PartialEq, Eq)]
enum Operation {
    Value { left: String },
    And { left: String, right: String },
    LShift { left: String, right: String },
    RShift { left: String, right: String },
    Not { left: String },
    Or { left: String, right: String },
}

/// Processes the AOC 2015 Day 07 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 07 input file in the format required by the solver functions.
/// Returned value is ###.
fn process_input_file(filename: &str) -> HashMap<String, Operation> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut wires: HashMap<String, Operation> = HashMap::new();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Extract the wire and operation from the current line
        if let Ok(Some(caps)) = REGEX_VALUE.captures(line) {
            let left = caps[1].to_string();
            let wire = caps[2].to_string();
            wires.insert(wire, Operation::Value { left });
        } else if let Ok(Some(caps)) = REGEX_AND.captures(line) {
            let left = caps[1].to_string();
            let right = caps[2].to_string();
            let wire = caps[3].to_string();
            wires.insert(wire, Operation::And { left, right });
        } else if let Ok(Some(caps)) = REGEX_LSHIFT.captures(line) {
            let left = caps[1].to_string();
            let right = caps[2].to_string();
            let wire = caps[3].to_string();
            wires.insert(wire, Operation::LShift { left, right });
        } else if let Ok(Some(caps)) = REGEX_RSHIFT.captures(line) {
            let left = caps[1].to_string();
            let right = caps[2].to_string();
            let wire = caps[3].to_string();
            wires.insert(wire, Operation::RShift { left, right });
        } else if let Ok(Some(caps)) = REGEX_NOT.captures(line) {
            let left = caps[1].to_string();
            let wire = caps[2].to_string();
            wires.insert(wire, Operation::Not { left });
        } else if let Ok(Some(caps)) = REGEX_OR.captures(line) {
            let left = caps[1].to_string();
            let right = caps[2].to_string();
            let wire = caps[3].to_string();
            wires.insert(wire, Operation::Or { left, right });
        } else {
            panic!("Day 7: bad format input line // {}", line);
        }
    }
    wires
}

/// Solves AOC 2015 Day 07 Part 1 // Determines the value that is provided to wire "a".
fn solve_part1(wires: &HashMap<String, Operation>) -> u16 {
    determine_target_wire_value(&String::from("a"), wires)
}

/// Solves AOC 2015 Day 07 Part 2 // Determines the value that is provided to wire "a" after
/// mapping the initial value of wire "a" to wire "b" and recalculating the wire "a" value.
fn solve_part2(wires: &HashMap<String, Operation>) -> u16 {
    // Calculate initial value of wire "a"
    let wire_a_value = determine_target_wire_value(&String::from("a"), wires);
    // Update the value provided to wire "b"
    let mut new_wires = wires.clone();
    new_wires.insert(String::from("b"), Operation::Value { left: wire_a_value.to_string() });
    // Recalculate value of wire "a"
    determine_target_wire_value(&String::from("a"), &new_wires)
}

/// Determines the value provided to the target wire.
fn determine_target_wire_value(
    target_wire: &String,
    wires: &HashMap<String, Operation>,
) -> u16 {
    let mut wire_values: HashMap<String, u16> = HashMap::new();
    determine_target_wire_value_recursive(target_wire, wires, &mut wire_values)
}

/// Recursive support function used to determine the value provided to the target wire.
fn determine_target_wire_value_recursive(
    target_wire: &String,
    wires: &HashMap<String, Operation>,
    wire_values: &mut HashMap<String, u16>,
) -> u16 {
    // Check if the wire value has already been found
    if let Entry::Occupied(e) = wire_values.entry(target_wire.to_string()) {
        return *e.get();
    }
    // Calculate the value fed to the target wire
    let wire_value = {
        match wires.get(target_wire).unwrap() {
            Operation::Value { left } => get_term_value(left, wires, wire_values),
            Operation::And { left, right } => {
                let left = get_term_value(left, wires, wire_values);
                let right = get_term_value(right, wires, wire_values);
                left & right
            }
            Operation::LShift { left, right } => {
                let left = get_term_value(left, wires, wire_values);
                let right = get_term_value(right, wires, wire_values);
                left << right
            }
            Operation::RShift { left, right } => {
                let left = get_term_value(left, wires, wire_values);
                let right = get_term_value(right, wires, wire_values);
                left >> right
            }
            Operation::Not { left } => {
                let left = get_term_value(left, wires, wire_values);
                !left
            }
            Operation::Or { left, right } => {
                let left = get_term_value(left, wires, wire_values);
                let right = get_term_value(right, wires, wire_values);
                left | right
            }
        }
    };
    // Records the value fed to the target wire
    wire_values.insert(target_wire.to_string(), wire_value);
    wire_value
}

/// Gets the value of the given term, if it is a specific value or the name of a wire.
fn get_term_value(
    term: &String,
    wires: &HashMap<String, Operation>,
    wire_values: &mut HashMap<String, u16>,
) -> u16 {
    if let Ok(value) = term.parse::<u16>() {
        value
    } else {
        determine_target_wire_value_recursive(term, wires, wire_values)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 07 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day07_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(956, solution);
    }

    /// Tests the Day 07 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day07_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(40149, solution);
    }
}
