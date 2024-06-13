use crate::calculator::{BaseStats, Stats, Target};
use crate::{Element, SkillType};
use crate::echo::{Echo, StatType};

// This is the tolerance for comparisons to numbers taken from the game.
const TOLERANCE: f64 = 1.0;

#[test]
fn test_calculate_base_damage_set1() {
    let jiyan_base_stats_70 = BaseStats {hp: 7954.0, atk: 343.0, def: 899.0};
    let mut stats = Stats::new_from_base(jiyan_base_stats_70);

    // Helios Cleaver lvl 70/70
    stats.base_atk += 312.0;
    stats.atk_mult += 0.251;

    // Talent stats
    stats.crit_rate += 0.012 + 0.012;
    stats.crit_dmg += 0.12;

    let echoes = [
        Echo {
            cost: 4,
            main_stat_type: StatType::CritRate,
            main_stat_value: 0.22,
            secondary_stat_type: StatType::AtkFlat,
            secondary_stat_value: 150.0,
            sub_stats: [
                (StatType::CritRate, 0.087),
                (StatType::SkillDmg, 0.094),
                (StatType::AtkFlat, 30.0),
                (StatType::EnergyRegen, 0.10),
                (StatType::HpMult, 0.079),
            ],
        },
        Echo {
            cost: 3,
            main_stat_type: StatType::AeroDmg,
            main_stat_value: 0.252,
            secondary_stat_type: StatType::AtkFlat,
            secondary_stat_value: 84.0,
            sub_stats: [
                (StatType::DefMult, 0.128),
                (StatType::CritRate, 0.063),
                (StatType::EnergyRegen, 0.076),
                (StatType::HeavyDmg, 0.094),
                (StatType::None, 0.0),
            ],
        },
        Echo {
            cost: 3,
            main_stat_type: StatType::AeroDmg,
            main_stat_value: 0.223,
            secondary_stat_type: StatType::AtkFlat,
            secondary_stat_value: 74.0,
            sub_stats: [
                (StatType::AtkFlat, 40.0),
                (StatType::AtkMult, 0.079),
                (StatType::CritRate, 0.075),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
            ],
        },
        Echo {
            cost: 1,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.18,
            secondary_stat_type: StatType::HpFlat,
            secondary_stat_value: 2280.0,
            sub_stats: [
                (StatType::CritDmg, 0.138),
                (StatType::SkillDmg, 0.094),
                (StatType::EnergyRegen, 0.10),
                (StatType::HpFlat, 470.0),
                (StatType::AtkMult, 0.079),
            ],
        },
        Echo {
            cost: 1,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.18,
            secondary_stat_type: StatType::HpFlat,
            secondary_stat_value: 2280.0,
            sub_stats: [
                (StatType::EnergyRegen, 0.108),
                (StatType::CritRate, 0.069),
                (StatType::CritDmg, 0.186),
                (StatType::HpMult, 0.094),
                (StatType::DefFlat, 50.0),
            ],
        },
    ];

    echoes[0].add_to_stats(&mut stats);
    echoes[1].add_to_stats(&mut stats);
    echoes[2].add_to_stats(&mut stats);
    echoes[3].add_to_stats(&mut stats);
    echoes[4].add_to_stats(&mut stats);

    // Echo set bonuses
    stats.element_dmg[Element::Aero as usize] += 0.10;

    // First hit of his basic attack
    let target = Target {
        element: Element::Aero,
        skill_type: SkillType::Basic,
        skill_multiplier: 0.5354,
        skill_scaling_bonus: 1.0,
    };
    
    // Game data
    let expected_wildlife_dmgs = (1296.0, 2519.0);
    let expected_dmgs_per_level = [
        (64, (596.0, 1158.0)),
        (65, (594.0, 1154.0)),
        (66, (592.0, 1151.0)),
        (69, (587.0, 1140.0)),
        (70, (585.0, 1137.0)),
    ];

    assert!((stats.expected_skill_hit_noncrit(target) - expected_wildlife_dmgs.0).abs() < TOLERANCE);
    assert!((stats.expected_skill_hit_crit(target) - expected_wildlife_dmgs.1).abs() < TOLERANCE);

    for (level, expected_dmgs) in expected_dmgs_per_level.iter() {
        assert!((stats.enemy_hit_noncrit(target, 70, *level) - expected_dmgs.0).abs() < TOLERANCE);
        assert!((stats.enemy_hit_crit(target, 70, *level) - expected_dmgs.1).abs() < TOLERANCE);
    }
}

#[test]
fn test_calculate_base_damage_set2() {
    let jiyan_base_stats_70 = BaseStats {hp: 7954.0, atk: 343.0, def: 899.0};
    let mut stats = Stats::new_from_base(jiyan_base_stats_70);

    // Verdant Summit 60/60
    stats.base_atk += 374.0;
    stats.crit_dmg += 0.359;
    for i in 0..6 { // weapon passive
        stats.element_dmg[i] += 0.12;
    }

    // Talent stats
    stats.crit_rate += 0.012 + 0.012;
    stats.atk_mult += 0.018 + 0.018;
    stats.crit_dmg += 0.12;

    let echoes = [
        Echo {
            cost: 4,
            main_stat_type: StatType::CritRate,
            main_stat_value: 0.22,
            secondary_stat_type: StatType::AtkFlat,
            secondary_stat_value: 150.0,
            sub_stats: [
                (StatType::HpFlat, 470.0),
                (StatType::EnergyRegen, 0.10),
                (StatType::AtkFlat, 50.0),
                (StatType::CritDmg, 0.174),
                (StatType::LiberationDmg, 0.079),
            ],
        },
        Echo {
            cost: 3,
            main_stat_type: StatType::EnergyRegen,
            main_stat_value: 0.268,
            secondary_stat_type: StatType::AtkFlat,
            secondary_stat_value: 84.0,
            sub_stats: [
                (StatType::CritDmg, 0.15),
                (StatType::AtkFlat, 50.0),
                (StatType::HpFlat, 320.0),
                (StatType::AtkMult, 0.086),
                (StatType::None, 0.0),
            ],
        },
        Echo {
            cost: 3,
            main_stat_type: StatType::AeroDmg,
            main_stat_value: 0.30,
            secondary_stat_type: StatType::AtkFlat,
            secondary_stat_value: 100.0,
            sub_stats: [
                (StatType::CritRate, 0.075),
                (StatType::AtkFlat, 50.0),
                (StatType::CritDmg, 0.15),
                (StatType::HeavyDmg, 0.094),
                (StatType::HpFlat, 430.0),
            ],
        },
        Echo {
            cost: 1,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.151,
            secondary_stat_type: StatType::HpFlat,
            secondary_stat_value: 1915.0,
            sub_stats: [
                (StatType::AtkFlat, 50.0),
                (StatType::AtkMult, 0.086),
                (StatType::CritDmg, 0.138),
                (StatType::EnergyRegen, 0.076),
                (StatType::None, 0.0),
            ],
        },
        Echo {
            cost: 1,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.122,
            secondary_stat_type: StatType::HpFlat,
            secondary_stat_value: 1550.0,
            sub_stats: [
                (StatType::BasicDmg, 0.101),
                (StatType::CritDmg, 0.174),
                (StatType::DefFlat, 50.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
            ],
        },
    ];

    echoes[0].add_to_stats(&mut stats);
    echoes[1].add_to_stats(&mut stats);
    echoes[2].add_to_stats(&mut stats);
    echoes[3].add_to_stats(&mut stats);
    echoes[4].add_to_stats(&mut stats);

    // Echo set bonuses
    stats.atk_mult += 0.10;
    stats.element_dmg[Element::Aero as usize] += 0.10;

    // First hit of his basic attack
    let target = Target {
        element: Element::Aero,
        skill_type: SkillType::Basic,
        skill_multiplier: 0.5007,
        skill_scaling_bonus: 1.0,
    };

    // Game data
    let expected_wildfile_dmgs = (1353.0, 3743.0);
    let expected_dmgs_per_level = [
        (62, (626.0, 1730.0)),
        (63, (624.0, 1725.0)),
        (64, (622.0, 1720.0)),
        (65, (620.0, 1715.0)),
        (67, (617.0, 1705.0)),
        (70, (611.0, 1689.0)),
    ];

    assert!((stats.expected_skill_hit_noncrit(target) - expected_wildfile_dmgs.0).abs() < TOLERANCE);
    assert!((stats.expected_skill_hit_crit(target) - expected_wildfile_dmgs.1).abs() < TOLERANCE);

    for (level, expected_dmgs) in expected_dmgs_per_level.iter() {
        assert!((stats.enemy_hit_noncrit(target, 70, *level) - expected_dmgs.0).abs() < TOLERANCE);
        assert!((stats.enemy_hit_crit(target, 70, *level) - expected_dmgs.1).abs() < TOLERANCE);
    }
}
