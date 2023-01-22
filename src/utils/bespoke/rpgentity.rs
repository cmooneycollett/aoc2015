use std::cmp;

/// Represents a single entity (player or enemy) used in RPG Simulator 20XX from AOC 2015 Day 21
/// (https://adventofcode.com/2015/day/21).
pub struct RpgEntity {
    health: i64,
    damage: i64,
    armour: i64,
}

impl RpgEntity {
    pub fn new(health: i64, damage: i64, armour: i64) -> RpgEntity {
        RpgEntity {
            health,
            damage,
            armour,
        }
    }

    /// Checks if the entity is dead (non-positive health points remaining).
    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    /// Determines the number of turns required for the entity to defeat the other entity.
    pub fn turns_to_defeat(&self, other: &RpgEntity) -> u64 {
        let turn_damage = cmp::max(self.damage - other.armour, 1);
        let turns = {
            let temp = other.health / turn_damage;
            if other.health % turn_damage > 0 {
                temp + 1
            } else {
                temp
            }
        };
        turns as u64
    }
}
