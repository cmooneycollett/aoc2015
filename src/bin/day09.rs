use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use itertools::Itertools;

const PROBLEM_NAME: &str = "All in a Single Night";
const PROBLEM_INPUT_FILE: &str = "./input/day09.txt";
const PROBLEM_DAY: u64 = 9;

/// Processes the AOC 2015 Day 09 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 09 input file in the format required by the solver functions.
/// Returned value is hashmap mapping edge "from" nodes to edge "to" nodes and the associated edge
/// weight.
fn process_input_file(filename: &str) -> HashMap<String, HashMap<String, u64>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut edges: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let regex_line = Regex::new(r"^([[:alpha:]]+) to ([[:alpha:]]+) = (\d+)$").unwrap();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(Some(caps)) = regex_line.captures(line) {
            let node1 = &caps[1];
            let node2 = &caps[2];
            let weight = caps[3].parse::<u64>().unwrap();
            // Forward edge (node1 to node2)
            add_edges_between_nodes(&mut edges, node1, node2, weight);
            // Backward edge (node2 to node1)
            add_edges_between_nodes(&mut edges, node2, node1, weight);
        } else {
            panic!("Day 9 - bad format input file // {}", line);
        }
    }
    edges
}

/// Adds a new edge going from node1 to node2 with the given weight.
fn add_edges_between_nodes(
    edges: &mut HashMap<String, HashMap<String, u64>>,
    node1: &str,
    node2: &str,
    weight: u64,
) {
    if let Entry::Vacant(e) = edges.entry(node1.to_string()) {
        e.insert(HashMap::from([(node2.to_string(), weight)]));
    } else {
        edges
            .get_mut(&node1.to_string())
            .unwrap()
            .insert(node2.to_string(), weight);
    }
}

/// Solves AOC 2015 Day 09 Part 1 // Determines the minimum distance required to visit all nodes
/// in the graph.
fn solve_part1(edges: &HashMap<String, HashMap<String, u64>>) -> u64 {
    if let (Some(min), _) = find_min_max_distances_to_visit_all_nodes(edges) {
        return min;
    }
    panic!("Did not find the minimum distance path!");
}

/// Solves AOC 2015 Day 09 Part 2 // Determines the maximum distance required to visit all nodes in
/// the graph.
fn solve_part2(edges: &HashMap<String, HashMap<String, u64>>) -> u64 {
    if let (_, Some(max_dist)) = find_min_max_distances_to_visit_all_nodes(edges) {
        return max_dist;
    }
    panic!("Did not find the maximum distance path!");
}

/// Finds the minimum and maximum distances needed to visit all nodes in the graph. Returned value
/// is a tuple containing the minimum and maximum distances found (in that order).
fn find_min_max_distances_to_visit_all_nodes(
    edges: &HashMap<String, HashMap<String, u64>>,
) -> (Option<u64>, Option<u64>) {
    let mut min_dist: Option<u64> = None;
    let mut max_dist: Option<u64> = None;
    // Try each of the possible orders to visit all nodes
    for nodes in edges.keys().permutations(edges.len()) {
        // Calculate the total distance between the nodes in the order visited
        let mut dist = 0;
        for i in 1..nodes.len() {
            dist += edges.get(nodes[i - 1]).unwrap().get(nodes[i]).unwrap();
        }
        // Check if a new minimum or maximum distance has been found
        if min_dist.is_none() || dist < min_dist.unwrap() {
            min_dist = Some(dist);
        }
        if max_dist.is_none() || dist > max_dist.unwrap() {
            max_dist = Some(dist);
        }
    }
    (min_dist, max_dist)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 09 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day09_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(141, solution);
    }

    /// Tests the Day 09 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day09_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(736, solution);
    }
}
