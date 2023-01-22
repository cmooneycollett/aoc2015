/// Represents a single item used in RPG Simulator 20XX from AOC 2015 Day 21
/// (https://adventofcode.com/2015/day/21).
pub struct RpgItem {
    cost: i64,
    damage: i64,
    armour: i64,
}

impl RpgItem {
    pub fn new(cost: i64, damage: i64, armour: i64) -> RpgItem {
        RpgItem {
            cost,
            damage,
            armour,
        }
    }

    /// Gets the value of the "cost" field.
    pub fn cost(&self) -> i64 {
        self.cost
    }

    /// Gets the value of the "damage" field.
    pub fn damage(&self) -> i64 {
        self.damage
    }

    /// Gets the value of the "armour" field.
    pub fn armour(&self) -> i64 {
        self.armour
    }
}
