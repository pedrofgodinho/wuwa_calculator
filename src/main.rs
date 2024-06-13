use wuwa_calculator::{Element, SkillType};
use wuwa_calculator::calculator::{BaseStats, Stats, Target};

fn main() {
    let mut stats = Stats::new_from_base(BaseStats {hp: 7955.23, atk: 344.05, def: 900.28});

    // Echo1
    stats.crit_rate += 0.22;
    stats.atk_flat += 150.0;
    stats.hp_flat += 470.0;
    stats.energy_regen += 0.10;
    stats.atk_flat += 50.0;
    stats.crit_dmg += 0.174;
    stats.skill_dmg[SkillType::Liberation as usize] += 0.079;

    // Echo2 ER 26.8  ATK 84  CritDmg 15.0  Atk 50  Hp 320  Atk 8.6
    stats.energy_regen += 0.268;
    stats.atk_flat += 84.0;
    stats.crit_dmg += 0.15;
    stats.atk_flat += 50.0;
    stats.hp_flat += 320.0;
    stats.atk_mult += 0.086;

    // Echo3 Aero 30.0  Atk 100  CritRate 7.5  Atk 50  CritDmg 15.0  Heavy 9.4  HP 430
    stats.element_dmg[Element::Aero as usize] += 0.30;
    stats.atk_flat += 100.0;
    stats.crit_rate += 0.075;
    stats.atk_flat += 50.0;
    stats.crit_dmg += 0.15;
    stats.skill_dmg[SkillType::Heavy as usize] += 0.094;
    stats.hp_flat += 430.0;

    // Echo4 Atk 15.1%  HP 1915  Atk 50  Atk 8.6%  CritDmg 13.8  ER 7.6
    stats.atk_mult += 0.151;
    stats.hp_flat += 1915.0;
    stats.atk_flat += 50.0;
    stats.atk_mult += 0.086;
    stats.crit_dmg += 0.138;
    stats.energy_regen += 0.076;

    // Echo5 Atk 12.2%  HP 1550  Basic 10.1  CritDmg 17.4  Def 50
    stats.atk_mult += 0.122;
    stats.hp_flat += 1550.0;
    stats.skill_dmg[SkillType::Basic as usize] += 0.101;
    stats.crit_dmg += 0.174;
    stats.def_flat += 50.0;

    // Set bonus
    stats.atk_mult += 0.10;
    stats.element_dmg[Element::Aero as usize] += 0.10;


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

    let first_hit = Target {
        element: Element::Aero,
        skill_type: SkillType::Basic,
        skill_multiplier: 0.5007,
        skill_scaling_bonus: 1.0,
    };

    println!();
    println!("No Defense hit noncrit: {}", stats.expected_skill_hit_noncrit(first_hit));
    println!("No Defense hit crit:    {}", stats.expected_skill_hit_crit(first_hit));
    println!();
    for i in 62..=70 {
        println!("Lvl {} non-crit: {}", i, stats.enemy_hit_noncrit(first_hit, 70, i));
        println!("Lvl {} crit:     {}", i, stats.enemy_hit_crit(first_hit, 70, i));
        println!();
    }
}


