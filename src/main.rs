use wuwa_calculator::{Element, SkillType};
use wuwa_calculator::calculator::{BaseStats, Stats, Target};
use wuwa_calculator::echo::{Echo, StatType};
use wuwa_calculator::optimizer::optimize;

fn main() {
    let mut stats = Stats::new_from_base(BaseStats {hp: 7955.23, atk: 344.05, def: 900.28});
    
    let echoes = [
        Echo { // Bad artifact, should not be picked
            cost: 4,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.151,
            secondary_stat_type: StatType::AtkFlat,
            secondary_stat_value: 100.0,
            sub_stats: [
                (StatType::EnergyRegen, 0.076),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
            ],
        },
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
        Echo { // Bad artifact, should not be picked
            cost: 1,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.122,
            secondary_stat_type: StatType::HpFlat,
            secondary_stat_value: 1550.0,
            sub_stats: [
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
            ],
        },
        Echo { // Bad artifact, should not be picked
            cost: 1,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.122,
            secondary_stat_type: StatType::HpFlat,
            secondary_stat_value: 1550.0,
            sub_stats: [
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
            ],
        },
        Echo { // Bad artifact, should not be picked
            cost: 1,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.122,
            secondary_stat_type: StatType::HpFlat,
            secondary_stat_value: 1550.0,
            sub_stats: [
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
            ],
        },
        Echo { // Bad artifact, should not be picked
            cost: 1,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.122,
            secondary_stat_type: StatType::HpFlat,
            secondary_stat_value: 1550.0,
            sub_stats: [
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
            ],
        },
        Echo { // Bad artifact, should not be picked
            cost: 1,
            main_stat_type: StatType::AtkMult,
            main_stat_value: 0.122,
            secondary_stat_type: StatType::HpFlat,
            secondary_stat_value: 1550.0,
            sub_stats: [
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
                (StatType::None, 0.0),
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

    let optimization_target = Target {
        element: Element::Aero,
        skill_type: SkillType::Basic,
        skill_multiplier: 0.5007,
        skill_scaling_bonus: 1.0,
    };
    
    let echoes = optimize(stats.clone(), &echoes, optimization_target, 70, 70);
    
    echoes[0].add_to_stats(&mut stats);
    echoes[1].add_to_stats(&mut stats);
    echoes[2].add_to_stats(&mut stats);
    echoes[3].add_to_stats(&mut stats);
    echoes[4].add_to_stats(&mut stats);

    // TODO add echo sets functionality. Hardcoding for now. 
    // Set bonus
    stats.atk_mult += 0.10;
    stats.element_dmg[Element::Aero as usize] += 0.10;
    
    // TODO add weapon functionality. Hardcoding for now
    // Weapon
    stats.base_atk += 374.0;
    stats.crit_dmg += 0.359;
    for i in 0..6 { // weapon passive
        stats.element_dmg[i] += 0.12;
    }
    
    // Forte Stats
    stats.crit_rate += 0.012 + 0.012;
    stats.atk_mult += 0.018 + 0.018;
    stats.crit_dmg += 0.12;

    println!("HP: {}", stats.hp());
    println!("ATK: {}", stats.atk());
    println!("DEF: {}", stats.def());
    println!("Crit Rate: {}", stats.crit_rate);
    println!("Crit Damage: {}", stats.crit_dmg);
    println!("Energy Regen: {}", stats.energy_regen);
    println!("Element Damage: {:?}", stats.element_dmg);
    println!("Skill Damage: {:?}", stats.skill_dmg);


    println!();
    println!("No Defense hit noncrit: {}", stats.expected_skill_hit_noncrit(optimization_target));
    println!("No Defense hit crit:    {}", stats.expected_skill_hit_crit(optimization_target));
    println!("No defense average:     {}", stats.expected_skill_hit_average(optimization_target));
    println!();
    for i in 62..=70 {
        println!("Lvl {} non-crit: {}", i, stats.enemy_hit_noncrit(optimization_target, 70, i));
        println!("Lvl {} crit:     {}", i, stats.enemy_hit_crit(optimization_target, 70, i));
        println!();
    }
}


