use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Aunt Sue";
const PROBLEM_INPUT_FILE: &str = "./input/day16.txt";
const PROBLEM_DAY: u64 = 16;

lazy_static! {
    static ref AUNT_SUE_ITEMS: HashMap<Category, u64> = HashMap::from([
        (Category::Children, 3),
        (Category::Cats, 7),
        (Category::Samoyeds, 2),
        (Category::Pomeranians, 3),
        (Category::Akitas, 0),
        (Category::Vizslas, 0),
        (Category::Goldfish, 5),
        (Category::Trees, 3),
        (Category::Cars, 2),
        (Category::Perfumes, 1),
    ]);
}

/// Represents the different category of items that Aunt Sue can have.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl Category {
    /// Returns the corresponding Category based on the given candidate string.
    pub fn from_string(candidate: &str) -> Option<Category> {
        match candidate {
            "children" => Some(Category::Children),
            "cats" => Some(Category::Cats),
            "samoyeds" => Some(Category::Samoyeds),
            "pomeranians" => Some(Category::Pomeranians),
            "akitas" => Some(Category::Akitas),
            "vizslas" => Some(Category::Vizslas),
            "goldfish" => Some(Category::Goldfish),
            "trees" => Some(Category::Trees),
            "cars" => Some(Category::Cars),
            "perfumes" => Some(Category::Perfumes),
            _ => None,
        }
    }
}

/// Processes the AOC 2015 Day 16 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 16 input file into the format required by the solver functions.
/// Returned value is vector of hashmaps containing the categories and quantities for each of the
/// Aunts Sue listed in the input file.
fn process_input_file(filename: &str) -> Vec<HashMap<Category, u64>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut aunts: Vec<HashMap<Category, u64>> = vec![];
    let regex_line = Regex::new(concat!(
        r#"(children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes): (\d+), "#,
        r#"(children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes): (\d+), "#,
        r#"(children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes): (\d+)"#,
    ))
    .unwrap();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(Some(caps)) = regex_line.captures(line) {
            let mut aunt_sue: HashMap<Category, u64> = HashMap::new();
            for i in [1, 3, 5] {
                let category = Category::from_string(&caps[i]).unwrap();
                let quantity = caps[i + 1].parse::<u64>().unwrap();
                aunt_sue.insert(category, quantity);
            }
            aunts.push(aunt_sue);
        } else {
            panic!("Bad format input line! // {line}");
        }
    }
    aunts
}

/// Solves AOC 2015 Day 16 Part 1 // Determines the number of the Aunt Sue that gave the gift to the
/// protagonist.
fn solve_part1(aunts: &[HashMap<Category, u64>]) -> usize {
    for (i, candidate) in aunts.iter().enumerate() {
        if check_aunt_sue(candidate) {
            return i + 1;
        }
    }
    panic!("Did not find the gift-giving Aunt Sue!");
}

/// Solves AOC 2015 Day 16 Part 2 // ###
fn solve_part2(_aunts: &[HashMap<Category, u64>]) -> usize {
    0
}

/// Checks if the candidate Aunt Sue quantities align with the expected Aunt Sue items based on the
/// MFCSAM print-out.
fn check_aunt_sue(candidate: &HashMap<Category, u64>) -> bool {
    for (category, quantity) in candidate.iter() {
        if AUNT_SUE_ITEMS.get(category).unwrap() != quantity {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 16 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day16_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(373, solution);
    }

    /// Tests the Day 16 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day16_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(260, solution);
    }
}
