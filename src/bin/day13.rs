use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use itertools::Itertools;

const PROBLEM_NAME: &str = "Knights of the Dinner Table";
const PROBLEM_INPUT_FILE: &str = "./input/day13.txt";
const PROBLEM_DAY: u64 = 13;

const PROTAGONIST_NAME: &str = "Mr. Robot";

/// Processes the AOC 2015 Day 13 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 13 input file into the format required by the solver functions.
/// Returned value is hashmap mapping each person to other people and the associated change in
/// happiness level if they sit next to each other.
fn process_input_file(filename: &str) -> HashMap<String, HashMap<String, i64>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut edges: HashMap<String, HashMap<String, i64>> = HashMap::new();
    let regex_line = Regex::new(concat!(
        r#"^([[:alpha:]]+) would (gain|lose) (\d+) happiness unit[s]? by "#,
        r#"sitting next to ([[:alpha:]]+).$"#,
    ))
    .unwrap();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(Some(caps)) = regex_line.captures(line) {
            let name_from = &caps[1];
            let name_to = &caps[4];
            let points = match &caps[2] {
                "gain" => caps[3].parse::<i64>().unwrap(),
                "lose" => -caps[3].parse::<i64>().unwrap(),
                _ => panic!("Bad gain/lose specification! // {}", &caps[2]),
            };
            if let Entry::Vacant(e) = edges.entry(name_from.to_string()) {
                e.insert(HashMap::from([(name_to.to_string(), points)]));
            } else {
                edges
                    .get_mut(name_from)
                    .unwrap()
                    .insert(name_to.to_string(), points);
            }
        } else {
            panic!("Bad format input line! // {line}");
        }
    }
    edges
}

/// Solves AOC 2015 Day 13 Part 1 // Determines the total change in happiness for the optimal
/// seating arrangement of the actual guest list.
fn solve_part1(edges: &HashMap<String, HashMap<String, i64>>) -> i64 {
    find_max_happiness_delta(edges)
}

/// Solves AOC 2015 Day 13 Part 2 // Determines the total change in happiness for the optimal
/// seating arrangement after the protagonist is added to the guest list.
fn solve_part2(edges: &HashMap<String, HashMap<String, i64>>) -> i64 {
    let edges = insert_new_attendee(edges, PROTAGONIST_NAME);
    find_max_happiness_delta(&edges)
}

/// Returns the updated edges map after inserting the new attendee with given name.
fn insert_new_attendee(
    edges: &HashMap<String, HashMap<String, i64>>,
    new_name: &str,
) -> HashMap<String, HashMap<String, i64>> {
    // Put new person in existing edge records
    let mut edges = edges.clone();
    for value in edges.values_mut() {
        value.insert(new_name.to_string(), 0);
    }
    // Insert edge record from the new name
    let mut new_name_edges: HashMap<String, i64> = HashMap::new();
    for name in edges.keys() {
        new_name_edges.insert(name.to_string(), 0);
    }
    edges.insert(new_name.to_string(), new_name_edges);
    edges
}

/// Determines the maximum change in happiness possible for a seating arrangement of the people
/// named in the given graph edges.
fn find_max_happiness_delta(edges: &HashMap<String, HashMap<String, i64>>) -> i64 {
    let mut max_happiness_delta: Option<i64> = None;
    // Try each possible ordering of the names
    for order in edges.keys().permutations(edges.len()) {
        // Calculate the happiness delta from the ordering being checked
        let mut happiness_delta = 0;
        for (i, name_from) in order.iter().enumerate() {
            let name_to = order[(i + 1) % order.len()];
            happiness_delta += edges.get(*name_from).unwrap().get(name_to).unwrap();
            happiness_delta += edges.get(name_to).unwrap().get(*name_from).unwrap();
        }
        // Check if a new maximum happiness delta has been found
        if max_happiness_delta.is_none() || happiness_delta > max_happiness_delta.unwrap() {
            max_happiness_delta = Some(happiness_delta);
        }
    }
    // Return the maximum happiness delta found
    if let Some(value) = max_happiness_delta {
        value
    } else {
        panic!("Did not find the maximum happiness change!");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 13 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day13_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(664, solution);
    }

    /// Tests the Day 13 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day13_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(640, solution);
    }
}
