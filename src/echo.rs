use crate::calculator::Stats;

pub enum MainStatType {
    AtkFlat,
    AtkMult,
    HpFlat,
    HpMult,
    DefMult,
    GlacioDmg,
    FusionDmg,
    ElectroDmg,
    AeroDmg,
    SpectroDmg,
    HavocDmg,
    EnergyRegen,
    CritRate,
    CritDmg,
    Healing,
}

pub enum SecondaryStatType {
    AtkFlat,
    HpFlat,
}

pub enum SubStatType {
    HpFlat,
    HpMult,
    DefFlat,
    DefMult,
    AtkFlat,
    AtkMult,
    CritRate,
    CritDmg,
    EnergyRegen,
    SkillDmg,
    BasicDmg,
    HeavyDmg,
    LiberationDmg,
    None,
}

pub struct Echo {
    pub main_stat_type: MainStatType,
    pub main_stat_value: f64,
    pub secondary_stat_type: SecondaryStatType,
    pub secondary_stat_value: f64,
    pub sub_stats: Vec<(SubStatType, f64)>,
}

impl Echo {
    pub fn add_to_stats(&self, stats: &mut Stats) {
        // todo
    }
    
    pub fn remove_from_stats(&self, stats: &mut Stats) {
        // todo
    }
}
