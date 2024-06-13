use crate::calculator::Stats;

/// The `StatType` enum represents the different types of stats that can be added to a character through an echo
#[derive(Copy, Clone)]
pub enum StatType {
    // Main Stats
    AtkFlat,
    AtkMult,
    HpFlat,
    HpMult,
    DefFlat,
    DefMult,
    // Element Stats
    GlacioDmg,
    FusionDmg,
    ElectroDmg,
    AeroDmg,
    SpectroDmg,
    HavocDmg,
    // Crit, ER
    EnergyRegen,
    CritRate,
    CritDmg,
    Healing,
    // Skill Stats
    SkillDmg,
    BasicDmg,
    HeavyDmg,
    LiberationDmg,
    // Unrolled
    None,
}

impl StatType {
    fn add_to_stats(&self, stats: &mut Stats, value: f64) {
        match self {
            // Main Stats
            StatType::AtkFlat => stats.atk_flat += value,
            StatType::AtkMult => stats.atk_mult += value,
            StatType::HpFlat => stats.hp_flat += value,
            StatType::HpMult => stats.hp_mult += value,
            StatType::DefFlat => stats.def_flat += value,
            StatType::DefMult => stats.def_mult += value,
            // Element Stats
            StatType::GlacioDmg => stats.element_dmg[0] += value,
            StatType::FusionDmg => stats.element_dmg[1] += value,
            StatType::ElectroDmg => stats.element_dmg[2] += value,
            StatType::AeroDmg => stats.element_dmg[3] += value,
            StatType::SpectroDmg => stats.element_dmg[4] += value,
            StatType::HavocDmg => stats.element_dmg[5] += value,
            // Crit, ER
            StatType::EnergyRegen => stats.energy_regen += value,
            StatType::CritRate => stats.crit_rate += value,
            StatType::CritDmg => stats.crit_dmg += value,
            StatType::Healing => stats.healing_bonus += value,
            // Skill Stats
            StatType::SkillDmg => stats.skill_dmg[0] += value,
            StatType::BasicDmg => stats.skill_dmg[1] += value,
            StatType::HeavyDmg => stats.skill_dmg[2] += value,
            StatType::LiberationDmg => stats.skill_dmg[3] += value,
            // Unrolled
            StatType::None => (),
        }
    }
    
    fn remove_from_stats(&self, stats: &mut Stats, value: f64) {
        match self {
            // Main Stats
            StatType::AtkFlat => stats.atk_flat -= value,
            StatType::AtkMult => stats.atk_mult -= value,
            StatType::HpFlat => stats.hp_flat -= value,
            StatType::HpMult => stats.hp_mult -= value,
            StatType::DefFlat => stats.def_flat -= value,
            StatType::DefMult => stats.def_mult -= value,
            // Element Stats
            StatType::GlacioDmg => stats.element_dmg[0] -= value,
            StatType::FusionDmg => stats.element_dmg[1] -= value,
            StatType::ElectroDmg => stats.element_dmg[2] -= value,
            StatType::AeroDmg => stats.element_dmg[3] -= value,
            StatType::SpectroDmg => stats.element_dmg[4] -= value,
            StatType::HavocDmg => stats.element_dmg[5] -= value,
            // Crit, ER
            StatType::EnergyRegen => stats.energy_regen -= value,
            StatType::CritRate => stats.crit_rate -= value,
            StatType::CritDmg => stats.crit_dmg -= value,
            StatType::Healing => stats.healing_bonus -= value,
            // Skill Stats
            StatType::SkillDmg => stats.skill_dmg[0] -= value,
            StatType::BasicDmg => stats.skill_dmg[1] -= value,
            StatType::HeavyDmg => stats.skill_dmg[2] -= value,
            StatType::LiberationDmg => stats.skill_dmg[3] -= value,
            // Unrolled
            StatType::None => (),
        }
    }
}

/// The `Echo` struct represents an echo that can be added to a character
#[derive(Copy, Clone)]
pub struct Echo {
    pub cost: isize,
    pub main_stat_type: StatType,
    pub main_stat_value: f64,
    pub secondary_stat_type: StatType,
    pub secondary_stat_value: f64,
    pub sub_stats: [(StatType, f64); 5],
}

impl Echo {
    /// Adds the stats of the echo to the stats of a character
    /// 
    /// # Examples
    /// ```
    /// use wuwa_calculator::calculator::{BaseStats, Stats};
    /// use wuwa_calculator::echo::{Echo, StatType};
    ///
    /// let mut stats = Stats::new_from_base(BaseStats {hp: 7954.0, atk: 343.0, def: 899.0});
    /// 
    /// let echo = Echo {
    ///   cost: 4,
    ///   main_stat_type: StatType::CritRate,
    ///   main_stat_value: 0.22,
    ///   secondary_stat_type: StatType::AtkFlat,
    ///   secondary_stat_value: 150.0,
    ///   sub_stats: [
    ///       (StatType::CritRate, 0.087),
    ///       (StatType::None, 0.0),
    ///       (StatType::None, 0.0),
    ///       (StatType::None, 0.0),
    ///       (StatType::None, 0.0),
    ///     ],
    ///   };
    /// echo.add_to_stats(&mut stats);
    /// assert_eq!(stats.crit_rate, 0.05 + 0.22 + 0.087);
    /// assert_eq!(stats.atk_flat, 150.0);
    /// ```
    /// 
    pub fn add_to_stats(&self, stats: &mut Stats) {
        self.main_stat_type.add_to_stats(stats, self.main_stat_value);
        self.secondary_stat_type.add_to_stats(stats, self.secondary_stat_value);
        for (stat_type, value) in &self.sub_stats {
            stat_type.add_to_stats(stats, *value);
        }
    }

    /// Removes the stats of the echo to the stats of a character
    ///
    /// # Examples
    /// ```
    /// use wuwa_calculator::calculator::{BaseStats, Stats};
    /// use wuwa_calculator::echo::{Echo, StatType};
    ///
    /// let mut stats = Stats::new_from_base(BaseStats {hp: 7954.0, atk: 343.0, def: 899.0});
    ///
    /// let echo = Echo {
    ///   cost: 4,
    ///   main_stat_type: StatType::CritRate,
    ///   main_stat_value: 0.22,
    ///   secondary_stat_type: StatType::AtkFlat,
    ///   secondary_stat_value: 150.0,
    ///   sub_stats: [
    ///       (StatType::CritRate, 0.087),
    ///       (StatType::None, 0.0),
    ///       (StatType::None, 0.0),
    ///       (StatType::None, 0.0),
    ///       (StatType::None, 0.0),
    ///     ],
    ///   };
    /// echo.add_to_stats(&mut stats);
    /// echo.remove_from_stats(&mut stats);
    /// assert!((stats.crit_rate - 0.05).abs() < f64::EPSILON);
    /// assert!((stats.atk_flat - 0.0).abs() < f64::EPSILON);
    /// ```
    ///
    pub fn remove_from_stats(&self, stats: &mut Stats) {
        self.main_stat_type.remove_from_stats(stats, self.main_stat_value);
        self.secondary_stat_type.remove_from_stats(stats, self.secondary_stat_value);
        for (stat_type, value) in &self.sub_stats {
            stat_type.remove_from_stats(stats, *value);
        }
    }
}
