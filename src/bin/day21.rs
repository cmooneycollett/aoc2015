use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;

use aoc2015::utils::bespoke::{RpgEntity, RpgItem};

const PROBLEM_NAME: &str = "RPG Simulator 20XX";
const PROBLEM_INPUT_FILE: &str = "./input/day21.txt";
const PROBLEM_DAY: u64 = 21;

const PLAYER_START_HEALTH: i64 = 100;

lazy_static! {
    /// Weapons held by store
    static ref WEAPONS: Vec<RpgItem> = vec![
        RpgItem::new(8, 4, 0),  // Dagger
        RpgItem::new(10, 5, 0), // Shortsword
        RpgItem::new(25, 6, 0), // Warhammer
        RpgItem::new(40, 7, 0), // Longsword
        RpgItem::new(74, 8, 0), // Greataxe
    ];
    /// Armour held by store
    static ref ARMOUR: Vec<RpgItem> = vec![
        RpgItem::new(13, 0, 1),     // Leather
        RpgItem::new(31, 0, 2),     // Chainmail
        RpgItem::new(53, 0, 3),     // Splitmail
        RpgItem::new(75, 0, 4),     // Bandedmail
        RpgItem::new(102, 0, 5),    // Platemail
    ];
    /// Rings held by store
    static ref RINGS: Vec<RpgItem> = vec![
        RpgItem::new(25, 1, 0),     // Damage +1
        RpgItem::new(50, 2, 0),     // Damage +2
        RpgItem::new(100, 3, 0),    // Damage +3
        RpgItem::new(20, 0, 1),     // Defence +1
        RpgItem::new(40, 0, 2),     // Defence +2
        RpgItem::new(80, 0, 3),     // Defence +3
    ];
}

/// Processes the AOC 2015 Day 21 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 21 input file into the format required by the solver functions.
/// Returned value is the RpgEntity representing the boss entity specified in the input file.
fn process_input_file(filename: &str) -> RpgEntity {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let input_regex = Regex::new(r"Hit Points: (\d+)\nDamage: (\d+)\nArmor: (\d+)").unwrap();
    if let Ok(Some(caps)) = input_regex.captures(&raw_input) {
        let health = caps[1].parse::<i64>().unwrap();
        let damage = caps[2].parse::<i64>().unwrap();
        let armour = caps[3].parse::<i64>().unwrap();
        let enemy = RpgEntity::new(health, damage, armour);
        return enemy;
    }
    panic!("Invalid input file format!");
}

/// Solves AOC 2015 Day 21 Part 1 // Determines the least amount of gold the player can spend and
/// still win the fight.
fn solve_part1(enemy: &RpgEntity) -> i64 {
    let mut least_gold: Option<i64> = None;
    for (q_armour, q_rings) in iproduct!(0..=1, 0..=2) {
        for weapon_held in WEAPONS.iter() {
            for armour_held in ARMOUR.iter().combinations(q_armour) {
                for rings_held in RINGS.iter().combinations(q_rings) {
                    let mut cost = 0;
                    cost += weapon_held.cost();
                    cost += armour_held.iter().map(|elem| elem.cost()).sum::<i64>();
                    cost += rings_held.iter().map(|elem| elem.cost()).sum::<i64>();
                    let damage = weapon_held.damage()
                        + rings_held.iter().map(|elem| elem.damage()).sum::<i64>();
                    let armour = armour_held.iter().map(|elem| elem.armour()).sum::<i64>()
                        + rings_held.iter().map(|elem| elem.armour()).sum::<i64>();
                    // Create player
                    let player = RpgEntity::new(PLAYER_START_HEALTH, damage, armour);
                    // Calculate turns to defeat
                    let player_turns = player.turns_to_defeat(enemy);
                    let enemy_turns = enemy.turns_to_defeat(&player);
                    if player_turns <= enemy_turns
                        && (least_gold.is_none() || cost < least_gold.unwrap())
                    {
                        least_gold = Some(cost);
                    }
                }
            }
        }
    }
    if let Some(cost) = least_gold {
        return cost;
    }
    panic!("Did not find the least amount of gold with player win outcome!");
}

/// Solves AOC 2015 Day 21 Part 2 // Determines the most amount of gold the player can spend and
/// still lose the fight.
fn solve_part2(enemy: &RpgEntity) -> i64 {
    let mut most_gold: Option<i64> = None;
    for (q_armour, q_rings) in iproduct!(0..=1, 0..=2) {
        for weapon_held in WEAPONS.iter() {
            for armour_held in ARMOUR.iter().combinations(q_armour) {
                for rings_held in RINGS.iter().combinations(q_rings) {
                    let mut cost = 0;
                    cost += weapon_held.cost();
                    cost += armour_held.iter().map(|elem| elem.cost()).sum::<i64>();
                    cost += rings_held.iter().map(|elem| elem.cost()).sum::<i64>();
                    let damage = weapon_held.damage()
                        + rings_held.iter().map(|elem| elem.damage()).sum::<i64>();
                    let armour = armour_held.iter().map(|elem| elem.armour()).sum::<i64>()
                        + rings_held.iter().map(|elem| elem.armour()).sum::<i64>();
                    // Create player
                    let player = RpgEntity::new(PLAYER_START_HEALTH, damage, armour);
                    // Calculate turns to defeat
                    let player_turns = player.turns_to_defeat(enemy);
                    let enemy_turns = enemy.turns_to_defeat(&player);
                    if player_turns > enemy_turns
                        && (most_gold.is_none() || cost > most_gold.unwrap())
                    {
                        most_gold = Some(cost);
                    }
                }
            }
        }
    }
    if let Some(cost) = most_gold {
        return cost;
    }
    panic!("Did not find the least amount of gold with player win outcome!");
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 21 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day21_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(78, solution);
    }

    /// Tests the Day 21 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day21_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(148, solution);
    }
}
