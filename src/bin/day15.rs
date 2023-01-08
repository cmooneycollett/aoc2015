use std::fs;
use std::time::Instant;

use fancy_regex::{Captures, Regex};

use aoc2015::utils::bespoke::Ingredient;

const PROBLEM_NAME: &str = "Science for Hungry People";
const PROBLEM_INPUT_FILE: &str = "./input/day15.txt";
const PROBLEM_DAY: u64 = 15;

const LIMIT_TSP: i64 = 100;
const TARGET_CALORIES: i64 = 500;

/// Processes the AOC 2015 Day 15 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 15 input file in the format required by the solver functions.
/// Returned value is vector of ingredients described in the inpu t file.
fn process_input_file(filename: &str) -> Vec<Ingredient> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut ingredients: Vec<Ingredient> = vec![];
    let regex_line = Regex::new(concat!(
        r#"^.*: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), "#,
        r#"texture (-?\d+), calories (-?\d+)$"#,
    ))
    .unwrap();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(Some(caps)) = regex_line.captures(line) {
            ingredients.push(parse_captures_to_ingredient(&caps));
        } else {
            panic!("Bad format input line! // {line}");
        }
    }
    ingredients
}

/// Solves AOC 2015 Day 15 Part 1 // Finds the total score of the higest-scoring cookie that can be
/// made within the quantity limit.
fn solve_part1(ingredients: &[Ingredient]) -> i64 {
    if let Some(score) = find_highest_cookie_score(ingredients, false) {
        score
    } else {
        panic!("Did not find the highest cookie score without calorie checking!");
    }
}

/// Solves AOC 2015 Day 15 Part 2 // Finds the total score of the highest-scoring cookie with the
/// target calorie count that can be made from the ingredients.
fn solve_part2(ingredients: &[Ingredient]) -> i64 {
    if let Some(score) = find_highest_cookie_score(ingredients, true) {
        score
    } else {
        panic!("Did not find the highest cookie score with calorie checking!");
    }
}

/// Parses the given regex captures into an ingredient. Assumption is that the captures result from
/// regex matching the input file line format for an ingredient.
fn parse_captures_to_ingredient(caps: &Captures) -> Ingredient {
    let capacity = caps[1].parse::<i64>().unwrap();
    let durability = caps[2].parse::<i64>().unwrap();
    let flavour = caps[3].parse::<i64>().unwrap();
    let texture = caps[4].parse::<i64>().unwrap();
    let calories = caps[5].parse::<i64>().unwrap();
    Ingredient::new(capacity, durability, flavour, texture, calories)
}

/// Finds the highest cookie score possible with the ingredients and quantity limit.
fn find_highest_cookie_score(ingredients: &[Ingredient], check_calories: bool) -> Option<i64> {
    let mut max_score: Option<i64> = None;
    find_highest_cookie_score_recursive(ingredients, &vec![], 0, check_calories, &mut max_score);
    max_score
}

/// Recursive helper function to find the highest cookie score possible with the ingredients and
/// quantity limit. Calorie checking is optional.
fn find_highest_cookie_score_recursive(
    ingredients: &[Ingredient],
    quantities: &Vec<i64>,
    running_total: i64,
    check_calories: bool,
    max_score: &mut Option<i64>,
) {
    // Check if the enough quantity values have been found
    if quantities.len() == ingredients.len() {
        calculate_cookie_score(quantities, ingredients, check_calories, max_score);
        return;
    }
    // Find collection of quantity values all less that limit that add up to exactly the limit
    for tsp in 0..=LIMIT_TSP {
        if tsp + running_total > LIMIT_TSP {
            return;
        }
        let mut quantities = quantities.clone();
        quantities.push(tsp);
        find_highest_cookie_score_recursive(
            ingredients,
            &quantities,
            running_total + tsp,
            check_calories,
            max_score,
        );
    }
}

/// Calculates the current cookie score and updates the max score if a new maximum cookie score is
/// found. Calorie checking is only performed to determine a valid cookie if check_calories is set
/// to true.
fn calculate_cookie_score(
    quantities: &[i64],
    ingredients: &[Ingredient],
    check_calories: bool,
    max_score: &mut Option<i64>,
) {
    // Calculate cookie component scores
    let mut capacity_score = 0;
    let mut durability_score = 0;
    let mut flavour_score = 0;
    let mut texture_score = 0;
    let mut calorie_score = 0;
    for (i, tsp) in quantities.iter().enumerate() {
        capacity_score += tsp * ingredients[i].capacity();
        durability_score += tsp * ingredients[i].durability();
        flavour_score += tsp * ingredients[i].flavour();
        texture_score += tsp * ingredients[i].texture();
        calorie_score += tsp * ingredients[i].calories();
    }
    // Ensure that component scores are set to 0 if they are initially below 0
    if capacity_score < 0 {
        capacity_score = 0;
    }
    if durability_score < 0 {
        durability_score = 0;
    }
    if flavour_score < 0 {
        flavour_score = 0;
    }
    if texture_score < 0 {
        texture_score = 0;
    }
    // Check if a new maximum cookie score has been found
    let cookie_score = capacity_score * durability_score * flavour_score * texture_score;
    if check_calories && calorie_score != TARGET_CALORIES {
        return;
    }
    if let Some(score) = max_score {
        if cookie_score < *score {
            return;
        }
    }
    *max_score = Some(cookie_score);
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 15 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day15_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(13882464, solution);
    }

    /// Tests the Day 15 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day15_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(11171160, solution);
    }
}
