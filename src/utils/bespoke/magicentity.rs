use std::collections::HashMap;

use super::Spell;

/// Represents an entity with magical abilities.
#[derive(Clone)]
pub struct MagicEntity {
    health: i64,
    damage: i64,
    armour: i64,
    mana: i64,
    total_mana_spent: i64,
    active_effects: HashMap<Spell, i64>,
}

impl MagicEntity {
    pub fn new(health: i64, damage: i64, armour: i64, mana: i64) -> MagicEntity {
        MagicEntity {
            health,
            damage,
            armour,
            mana,
            total_mana_spent: 0,
            active_effects: HashMap::new(),
        }
    }

    /// Gets the value of the "health" field.
    pub fn health(&self) -> i64 {
        self.health
    }

    /// Gets the value of the "damage" field.
    pub fn damage(&self) -> i64 {
        self.damage
    }

    /// Gets the value of the "armour" field.
    pub fn armour(&self) -> i64 {
        self.armour
    }

    /// Gets the value of the "mana" field.
    pub fn mana(&self) -> i64 {
        self.mana
    }

    /// Gets the value of the "total_mana_spent" field.
    pub fn total_mana_spent(&self) -> i64 {
        self.total_mana_spent
    }

    /// Deals the specified amount of damage to the MagicEntity, ignoring the armour rating if
    /// required.
    pub fn deal_damage(&mut self, damage: i64, ignore_armour: bool) {
        // Determine the amount of damage to deal
        let mut damage_to_deal = damage;
        if !ignore_armour {
            damage_to_deal -= self.armour;
        }
        if damage_to_deal < 1 {
            damage_to_deal = 1;
        }
        // Deal damage
        self.health -= damage_to_deal;
    }

    /// Increases the health points of the MagicEntity by the given amount.
    pub fn heal(&mut self, health: i64) {
        self.health += health;
    }

    /// Checks if the MagicEntity is dead (i.e., has 0 health points remaining).
    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    /// Checks if the MagicEntity has enough mana remaining to cast the specified spell.
    pub fn can_cast(&self, spell: Spell) -> bool {
        self.mana >= spell.mana()
    }

    /// Checks if the specified spell effect is active.
    pub fn is_effect_active(&self, spell: Spell) -> bool {
        self.active_effects.contains_key(&spell)
    }

    /// Has the MagicEntity cast the specified spell and applies damage effects to the other
    /// MagicEntity.
    pub fn cast_spell(
        &mut self,
        spell: Spell,
        other: &mut MagicEntity,
        ignore_armour: bool,
    ) -> Result<(), &str> {
        // Check if the MagicEntity can cast the spell
        if !self.can_cast(spell) || self.is_effect_active(spell) {
            return Err("Cannot cast the spell!");
        }
        // Expend mana and cast spell
        self.total_mana_spent += spell.mana();
        self.mana -= spell.mana();
        match spell {
            Spell::MagicMissile => other.deal_damage(spell.damage(), ignore_armour),
            Spell::Drain => {
                other.deal_damage(spell.damage(), ignore_armour);
                self.heal(spell.health());
            }
            Spell::Shield => {
                self.active_effects.insert(spell, spell.effect_turns());
                self.armour += spell.armour();
            }
            Spell::Poison => _ = self.active_effects.insert(spell, spell.effect_turns()),
            Spell::Recharge => _ = self.active_effects.insert(spell, spell.effect_turns()),
        }
        Ok(())
    }

    /// Process the effects currently active on the MagicEntity and applies consequences to the
    /// other MagicEntity.
    pub fn process_effects(&mut self, other: &mut MagicEntity) {
        let mut effects_to_remove: Vec<Spell> = vec![];
        for (effect, turns) in self.active_effects.iter_mut() {
            *turns -= 1;
            match effect {
                Spell::MagicMissile => (),
                Spell::Drain => (),
                Spell::Shield => {
                    if *turns <= 0 {
                        self.armour -= effect.armour();
                        effects_to_remove.push(*effect);
                    }
                }
                Spell::Poison => {
                    other.deal_damage(effect.damage(), true);
                    if *turns <= 0 {
                        effects_to_remove.push(*effect);
                    }
                }
                Spell::Recharge => {
                    self.mana += effect.mana_heal();
                    if *turns <= 0 {
                        effects_to_remove.push(*effect);
                    }
                }
            }
        }
        // Remove effects that have expired
        for effect in effects_to_remove {
            self.active_effects.remove(&effect);
        }
    }
}
