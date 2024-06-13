use crate::{Element, SkillType};

#[cfg(test)]
mod tests;

/// The `Target` struct holds all the information about an attack needed to calculate its damage
#[derive(Clone, Copy)]
pub struct Target {
    pub element: Element,
    pub skill_type: SkillType,
    pub skill_multiplier: f64,
    pub skill_scaling_bonus: f64,
}

/// The `BaseStats` struct holds the base stats of a character, which are used to initialize the `Stats` struct
#[derive(Clone, Copy)]
pub struct BaseStats {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
}

/// The `Stats` struct holds all the stats of a character
#[derive(Clone, PartialEq, Debug)] // probably not deriving copy because it's a very large struct?
pub struct Stats {
    pub base_atk: f64,
    pub base_hp: f64,
    pub base_def: f64,

    // Base Stats
    pub hp_flat: f64,
    pub atk_flat: f64,
    pub def_flat: f64,

    pub hp_mult: f64,
    pub atk_mult: f64,
    pub def_mult: f64,

    pub crit_rate: f64,
    pub crit_dmg: f64,

    pub energy_regen: f64,

    pub element_dmg: [f64; 6],
    pub skill_dmg: [f64; 4],

    pub healing_bonus: f64,
}

impl Stats {
    /// Creates a new `Stats` struct from a `BaseStats` struct
    /// 
    /// # Examples
    /// ```
    /// use wuwa_calculator::calculator::{BaseStats, Stats};
    /// 
    /// let jiyan_base_stats = BaseStats {hp: 7954.0, atk: 343.0, def: 899.0};
    /// let stats = Stats::new_from_base(jiyan_base_stats);
    /// ```
    pub fn new_from_base(base: BaseStats) -> Stats {
        Stats {
            base_atk: base.atk,
            base_hp: base.hp,
            base_def: base.def,

            hp_flat: 0.0,
            atk_flat: 0.0,
            def_flat: 0.0,

            hp_mult: 1.0,
            atk_mult: 1.0,
            def_mult: 1.0,

            crit_rate: 0.05,
            crit_dmg: 1.5,

            energy_regen: 1.0,

            element_dmg: [0.0; 6],
            skill_dmg: [0.0; 4],

            healing_bonus: 1.0,
        }
    }

    /// Returns the HP of the character
    pub fn hp(&self) -> f64 {
        self.base_hp * self.hp_mult + self.hp_flat
    }

    /// Returns the ATK of the character
    pub fn atk(&self) -> f64 {
        self.base_atk * self.atk_mult + self.atk_flat
    }

    /// Returns the DEF of the character
    pub fn def(&self) -> f64 { // TODO fix this
        self.base_def * self.def_mult + self.def_flat
    }

    fn hit_multiplier_noncrit(&self) -> f64 { // TODO deepen
        self.atk()
    }

    fn hit_multiplier_crit(&self) -> f64 { // TODO deepen
        self.atk() * self.crit_dmg
    }

    fn hit_multiplier_average(&self) -> f64 { // TODO deepen
        self.atk() * (1.0 + self.crit_rate * (self.crit_dmg - 1.0))
    }

    /// Returns the base damage of a skill (when it does not crit) without taking into account the enemy's resistance
    pub fn skill_base_damage_noncrit(&self, target: Target) -> f64 {
        target.skill_multiplier * target.skill_scaling_bonus * self.hit_multiplier_noncrit()
            * (1.0 + self.element_dmg[target.element as usize] + self.skill_dmg[target.skill_type as usize])
    }

    /// Returns the base damage of a skill (when it crits) without taking into account the enemy's resistance
    pub fn skill_base_damage_crit(&self, target: Target) -> f64 {
        target.skill_multiplier * target.skill_scaling_bonus * self.hit_multiplier_crit()
            * (1.0 + self.element_dmg[target.element as usize] + self.skill_dmg[target.skill_type as usize])
    }

    /// Returns the base damage of a skill (averaging crit and noncrit) without taking into account the enemy's resistance
    pub fn skill_base_damage_average(&self, target: Target) -> f64 {
        target.skill_multiplier * target.skill_scaling_bonus * self.hit_multiplier_average()
            * (1.0 + self.element_dmg[target.element as usize] + self.skill_dmg[target.skill_type as usize])
    }

    fn enemy_resistance(&self, character_level: isize, enemy_level: isize) -> f64 {
        let def_ignore = 0.0; // TODO
        let res_pen = 0.0; // TODO
        let ele_res = 1.0; // TODO
        let dmg_reduction = 1.0; // TODO

        let base_res = 0.1;
        let res_total = base_res + res_pen;
        let res_adjusted = match res_total {
            ..= 0.0 => 1.0 - res_total/2.0,
            0.0 ..= 0.8 => 1.0 - res_total,
            _ => 1.0 / (1.0 + 5.0 * res_total)
        };

        let enemy_def = 8.0 * enemy_level as f64 + 792.0;
        let character_level_part = 800.0 + 8.0 * character_level as f64;
        let def_mult = character_level_part / (character_level_part + enemy_def * (1.0 - def_ignore));

        res_adjusted * ele_res * def_mult * dmg_reduction
    }
    
    // TODO take in enemy resistance as an argument

    /// Returns the adjusted damage of a skill (when it does not crit) taking into account the enemy's resistance
    pub fn skill_adjusted_damage_noncrit(&self, target: Target, character_level: isize, enemy_level: isize) -> f64 {
        self.skill_base_damage_noncrit(target) * self.enemy_resistance(character_level, enemy_level)
    }

    /// Returns the adjusted damage of a skill (when it crits) taking into account the enemy's resistance
    pub fn skill_adjusted_damage_crit(&self, target: Target, character_level: isize, enemy_level: isize) -> f64 {
        self.skill_base_damage_crit(target) * self.enemy_resistance(character_level, enemy_level)
    }

    /// Returns the adjusted damage of a skill (averaging crit and noncrit) taking into account the enemy's resistance
    pub fn skill_adjusted_damage_average(&self, target: Target, character_level: isize, enemy_level: isize) -> f64 {
        self.skill_base_damage_average(target) * self.enemy_resistance(character_level, enemy_level)
    }
}

