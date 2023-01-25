use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use strum::IntoEnumIterator;

use aoc2015::utils::bespoke::{MagicEntity, Spell};

const PROBLEM_NAME: &str = "Wizard Simulator 20XX";
const PROBLEM_INPUT_FILE: &str = "./input/day22.txt";
const PROBLEM_DAY: u64 = 22;

// Player starting values
const PLAYER_HEALTH: i64 = 50;
const PLAYER_MANA: i64 = 500;

/// Processes the AOC 2015 Day 22 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 22 input file into the format required by the solver functions.
/// Returned value is MagicEntity specified by the health and damage points in the input file
/// (armour and mana are set to 0).
fn process_input_file(filename: &str) -> MagicEntity {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let input_regex = Regex::new(r"Hit Points: (\d+)\nDamage: (\d+)").unwrap();
    if let Ok(Some(caps)) = input_regex.captures(&raw_input) {
        let health = caps[1].parse::<i64>().unwrap();
        let damage = caps[2].parse::<i64>().unwrap();
        return MagicEntity::new(health, damage, 0, 0);
    }
    panic!("Invalid format input file!")
}

/// Solves AOC 2015 Day 22 Part 1 // Determines the minimum amount of mana needed for the player to
/// defeat the enemy in Wizard Simulator 20XX (easy mode).
fn solve_part1(enemy: &MagicEntity) -> i64 {
    let player = &MagicEntity::new(PLAYER_HEALTH, 0, 0, PLAYER_MANA);
    if let Some(min_mana) = conduct_fight(player, enemy, false) {
        return min_mana;
    }
    panic!("Player was unable to defeat the enemy on easy mode!");
}

/// Solves AOC 2015 Day 22 Part 2 // ###
fn solve_part2(_enemy: &MagicEntity) -> i64 {
    0
}

/// Conducts the fight between the player and the enemy. Returns an Option containing the minimum
/// amount of mana needed by the player to defeat the enemy. Returned value is None if the player
/// is unable to defeat the enemy.
fn conduct_fight(player: &MagicEntity, enemy: &MagicEntity, hard_mode: bool) -> Option<i64> {
    let mut min_mana: Option<i64> = None;
    conduct_fight_recursive(player, enemy, hard_mode, &mut min_mana);
    min_mana
}

fn conduct_fight_recursive(
    player: &MagicEntity,
    enemy: &MagicEntity,
    hard_mode: bool,
    min_mana: &mut Option<i64>,
) {
    for spell in Spell::iter() {
        // Clone the player and the enemy
        let mut new_player = player.clone();
        let mut new_enemy = enemy.clone();
        // Player turn
        // // Apply hard mode damage and check if player is dead
        if hard_mode {
            new_player.deal_damage(1, true);
        }
        if new_player.is_dead() {
            return;
        }
        // // Process player effects then check if enemy is dead
        new_player.process_effects(&mut new_enemy);
        if new_enemy.is_dead() {
            if min_mana.is_none() || min_mana.unwrap() > new_player.total_mana_spent() {
                *min_mana = Some(new_player.total_mana_spent());
            }
            return;
        }
        // // Continue to next spell if player cannot cast the spell
        if !new_player.can_cast(spell) {
            return;
        }
        if new_player.is_effect_active(spell) {
            continue;
        }
        // // Cast the spell
        match new_player.cast_spell(spell, &mut new_enemy, false) {
            Ok(_) => (),
            Err(_message) => continue,
        }
        // // Check if the enemy is dead
        if new_enemy.is_dead() {
            if min_mana.is_none() || min_mana.unwrap() > new_player.total_mana_spent() {
                *min_mana = Some(new_player.total_mana_spent());
            }
            return;
        }
        // Enemy turn
        // // Process player effects and check if enemy is dead
        new_player.process_effects(&mut new_enemy);
        if new_enemy.is_dead() {
            if min_mana.is_none() || min_mana.unwrap() > new_player.total_mana_spent() {
                *min_mana = Some(new_player.total_mana_spent());
            }
            return;
        }
        // // Enemy deals damage to the player and check if player is dead
        new_player.deal_damage(new_enemy.damage(), false);
        if new_player.is_dead() {
            return;
        }
        // Go to the next set of turns
        conduct_fight_recursive(&new_player, &new_enemy, hard_mode, min_mana);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 22 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day22_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1824, solution);
    }

    /// Tests the Day 22 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day22_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1937, solution);
    }
}
