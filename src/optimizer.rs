use itertools::Itertools;
use crate::calculator::{Stats, Target};
use crate::echo::Echo;


// Temporary, rudimentary optimization function. Must receive at least 5 echoes. The number of combinations with this approach blows up quickly.
pub fn optimize(mut stats: Stats, echoes: &[Echo], target: Target, character_level: isize, enemy_level: isize) -> [Echo; 5] {
    let best_combo = echoes.iter()
        .tuple_combinations()
        .filter(|comb: &(&Echo, &Echo, &Echo, &Echo, &Echo)| {
            comb.0.cost + comb.1.cost + comb.2.cost + comb.3.cost + comb.4.cost <= 12
        })
        .map(|comb| {
            // This is ugly, but you can't iterate over a tuple
            // .combinations instead of tuple_combinations would clean this up,
            // but it creates a Vec to store each combination, which would be significantly less efficient
            comb.0.add_to_stats(&mut stats);
            comb.1.add_to_stats(&mut stats);
            comb.2.add_to_stats(&mut stats);
            comb.3.add_to_stats(&mut stats);
            comb.4.add_to_stats(&mut stats);
            
            let damage = stats.enemy_hit_noncrit(target, character_level, enemy_level);
            
            comb.0.remove_from_stats(&mut stats);
            comb.1.remove_from_stats(&mut stats);
            comb.2.remove_from_stats(&mut stats);
            comb.3.remove_from_stats(&mut stats);
            comb.4.remove_from_stats(&mut stats);
            (damage, comb)
        })
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap(); // TODO clean this unwrap -> means iterator was empty, aka either less than 5 echoes or no valid 5 echoes combination
    [*best_combo.1.0, *best_combo.1.1, *best_combo.1.2, *best_combo.1.3, *best_combo.1.4]
}
