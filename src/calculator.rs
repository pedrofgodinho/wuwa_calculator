use crate::{Element, SkillType};

#[derive(Clone, Copy)]
pub struct BaseStats {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
}

#[derive(Clone, Copy)]
pub struct Target {
    pub element: Element,
    pub skill_type: SkillType,
    pub skill_multiplier: f64,
    pub skill_scaling_bonus: f64,
}

#[derive(Clone)] // probably not deriving copy because it's a very large struct?
pub struct Stats {
    pub base_atk: f64,

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
    pub fn new_from_base(base: BaseStats) -> Stats {
        Stats {
            base_atk: base.atk,

            hp_flat: base.hp,
            atk_flat: 0.0,
            def_flat: base.def,

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

    pub fn hp(&self) -> f64 {
        self.hp_flat * self.hp_mult
    }

    pub fn atk(&self) -> f64 {
        self.base_atk * self.atk_mult + self.atk_flat
    }

    pub fn def(&self) -> f64 { // TODO fix this
        self.def_flat * self.def_mult
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

    pub fn expected_skill_hit_noncrit(&self, target: Target) -> f64 {
        target.skill_multiplier * target.skill_scaling_bonus * self.hit_multiplier_noncrit()
            * (1.0 + self.element_dmg[target.element as usize] + self.skill_dmg[target.skill_type as usize])
    }

    pub fn expected_skill_hit_crit(&self, target: Target) -> f64 {
        target.skill_multiplier * target.skill_scaling_bonus * self.hit_multiplier_crit()
            * (1.0 + self.element_dmg[target.element as usize] + self.skill_dmg[target.skill_type as usize])
    }

    pub fn expected_skill_hit_average(&self, target: Target) -> f64 {
        target.skill_multiplier * target.skill_scaling_bonus * self.hit_multiplier_average()
            * (1.0 + self.element_dmg[target.element as usize] + self.skill_dmg[target.skill_type as usize])
    }

    fn enemy_resistance(&self, character_level: isize, enemy_level: isize) -> f64 {
        let def_ignore = 0.0; // TODO
        let res_pen = 0.0; // TODO
        let ele_res = 1.0; // TODO
        let dmg_reduction = 1.0; // TODO

        let base_res = 0.9;
        let res_total = base_res + res_pen;

        let enemy_def = 8.0 * enemy_level as f64 + 792.0;
        let character_level_part = 800.0 + 8.0 * character_level as f64;
        let def_mult = character_level_part / (character_level_part + enemy_def * (1.0 - def_ignore));

        res_total * ele_res * def_mult * dmg_reduction
    }

    pub fn enemy_hit_noncrit(&self, target: Target, character_level: isize, enemy_level: isize) -> f64 {
        self.expected_skill_hit_noncrit(target) * self.enemy_resistance(character_level, enemy_level)
    }

    pub fn enemy_hit_crit(&self, target: Target, character_level: isize, enemy_level: isize) -> f64 {
        self.expected_skill_hit_crit(target) * self.enemy_resistance(character_level, enemy_level)
    }

    pub fn enemy_hit_average(&self, target: Target, character_level: isize, enemy_level: isize) -> f64 {
        self.expected_skill_hit_average(target) * self.enemy_resistance(character_level, enemy_level)
    }
}

