use strum::EnumIter;

/// Represents the different spells that can be cast in the Wizard Simulator 20XX used in AOC 2015
/// Day 22 (https://adventofcode.com/2015/day/22).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    /// Gets the mana cost for the spell.
    pub fn mana(&self) -> i64 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    /// Gets the amount of damage dealt by the spell.
    pub fn damage(&self) -> i64 {
        match self {
            Spell::MagicMissile => 4,
            Spell::Drain => 2,
            Spell::Shield => 0,
            Spell::Poison => 3,
            Spell::Recharge => 0,
        }
    }

    /// Gets the amount of health points healed by the spell.
    pub fn health(&self) -> i64 {
        match self {
            Spell::MagicMissile => 0,
            Spell::Drain => 2,
            Spell::Shield => 0,
            Spell::Poison => 0,
            Spell::Recharge => 0,
        }
    }

    /// Gets the amount of armour points given by the spell.
    pub fn armour(&self) -> i64 {
        match self {
            Spell::MagicMissile => 0,
            Spell::Drain => 0,
            Spell::Shield => 7,
            Spell::Poison => 0,
            Spell::Recharge => 0,
        }
    }

    /// Gets the amount of mana restored by the spell.
    pub fn mana_heal(&self) -> i64 {
        match self {
            Spell::MagicMissile => 0,
            Spell::Drain => 0,
            Spell::Shield => 0,
            Spell::Poison => 0,
            Spell::Recharge => 101,
        }
    }

    /// Gets the number of turns that the spell effect lasts for.
    pub fn effect_turns(&self) -> i64 {
        match self {
            Spell::MagicMissile => 0,
            Spell::Drain => 0,
            Spell::Shield => 6,
            Spell::Poison => 6,
            Spell::Recharge => 5,
        }
    }
}
