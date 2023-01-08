use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use aoc_utils::cartography::{CardinalDirection, Point2D};

const PROBLEM_NAME: &str = "Perfectly Spherical Houses in a Vacuum";
const PROBLEM_INPUT_FILE: &str = "./input/day03.txt";
const PROBLEM_DAY: u64 = 3;

/// Processes the AOC 2015 Day 03 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 03 input file into the format required by the solver functions.
/// Returned value is vector of cardinal directions indicated by the characters in the input file.
fn process_input_file(filename: &str) -> Vec<CardinalDirection> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut directions: Vec<CardinalDirection> = vec![];
    for c in raw_input.trim().chars() {
        match c {
            '^' => directions.push(CardinalDirection::North),
            '>' => directions.push(CardinalDirection::East),
            'v' => directions.push(CardinalDirection::South),
            '<' => directions.push(CardinalDirection::West),
            _ => panic!("Bad character in input file: {}", c),
        }
    }
    directions
}

/// Solves AOC 2015 Day 03 Part 1 // Determines the number of houses that receive at least one
/// present with only Santa delivering presents.
fn solve_part1(directions: &[CardinalDirection]) -> usize {
    let loc_start = Point2D::new(0, 0);
    let mut visited: HashSet<Point2D> = HashSet::from([loc_start]);
    deliver_presents(loc_start, &mut directions.iter(), &mut visited);
    visited.len()
}

/// Solves AOC 2015 Day 03 Part 2 // Determines the number of houses that receive at least one
/// present with Santa and Robo-Santa alternating movements.
fn solve_part2(directions: &[CardinalDirection]) -> usize {
    let loc_start = Point2D::new(0, 0);
    let mut visited: HashSet<Point2D> = HashSet::from([loc_start]);
    deliver_presents(loc_start, &mut directions.iter().step_by(2), &mut visited);
    deliver_presents(
        loc_start,
        &mut directions.iter().skip(1).step_by(2),
        &mut visited,
    );
    visited.len()
}

/// Adjusts the location, starting at the given location, based on the direction in the given
/// iterator.
fn deliver_presents(
    loc_start: Point2D,
    directions: &mut dyn Iterator<Item = &CardinalDirection>,
    visited: &mut HashSet<Point2D>,
) {
    let mut loc_santa = loc_start;
    for dirn in directions {
        match dirn {
            CardinalDirection::North => loc_santa.shift(0, -1),
            CardinalDirection::East => loc_santa.shift(1, 0),
            CardinalDirection::South => loc_santa.shift(0, 1),
            CardinalDirection::West => loc_santa.shift(-1, 0),
        }
        visited.insert(loc_santa);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 03 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day03_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(2572, solution);
    }

    /// Tests the Day 03 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day03_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(2631, solution);
    }
}
