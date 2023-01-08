/// Represents a single ingredient as described in the AOC 2015 Day 15 problem
/// (<https://adventofcode.com/2015/day/15>).
pub struct Ingredient {
    capacity: i64,
    durability: i64,
    flavour: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    pub fn new(capacity: i64, durability: i64, flavour: i64, texture: i64, calories: i64) -> Ingredient {
        Ingredient {
            capacity,
            durability,
            flavour,
            texture,
            calories,
        }
    }

    /// Gets the value of the "capacity" field.
    pub fn capacity(&self) -> i64 {
        self.capacity
    }

    /// Gets the value of the "durability" field.
    pub fn durability(&self) -> i64 {
        self.durability
    }

    /// Gets the value of the "flavour" field.
    pub fn flavour(&self) -> i64 {
        self.flavour
    }

    /// Gets the value of the "texture" field.
    pub fn texture(&self) -> i64 {
        self.texture
    }

    /// Gets the value of the "calories" field.
    pub fn calories(&self) -> i64 {
        self.calories
    }
}

