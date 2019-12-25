use model::{
    Unit,
    Weapon,
};

use crate::my_strategy::{
    Rectangular,
    Target,
    Vec2,
    World,
    as_score,
    get_hit_probabilities,
    get_hit_probability_by_spread,
};

pub fn get_optimal_target(current_unit: &Unit, world: &World) -> Option<i32> {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        world.units().iter()
            .filter(|unit| world.is_opponent_unit(unit))
            .filter_map(|unit| get_target_score(current_unit.id, current_unit.center(), &unit, weapon, &world).map(|score| (unit.id, score)))
            .max_by_key(|&(_, score)| score)
            .map(|(id, _)| id)
    } else {
        None
    }
}

fn get_target_score(current_unit_id: i32, current_unit_center: Vec2, opponent: &Unit, weapon: &Weapon, world: &World) -> Option<i32> {
    let hit_probability_by_spread = get_hit_probability_by_spread(current_unit_center, &opponent.rect(), weapon.spread, weapon.params.bullet.size);

    if hit_probability_by_spread < world.config().min_hit_probability_by_spread_to_shoot {
        return None;
    }

    let opponent_center = opponent.center();
    let direction = (opponent_center - current_unit_center).normalized();
    let hit_probabilities = get_hit_probabilities(current_unit_id, current_unit_center, direction,
        &Target::from_unit(opponent), weapon.spread, weapon.params.bullet.size, world,
        world.config().optimal_action_number_of_directions);

    if let (Some(explosion), Some(min_distance)) = (weapon.params.explosion.as_ref(), hit_probabilities.min_distance) {
        if min_distance < explosion.radius + 2.0 {
            return None;
        }
    }

    if (hit_probabilities.target + hit_probabilities.opponent_units) < world.config().min_target_hits_to_shoot
        || hit_probabilities.teammate_units > world.config().max_teammates_hits_to_shoot {
        return None;
    }

    Some(as_score(get_target_score_components(current_unit_center, opponent_center, world).iter().sum()))
}

fn get_target_score_components(current_unit_center: Vec2, opponent_center: Vec2, world: &World) -> [f64; 1] {
    let distance_score = current_unit_center.distance(opponent_center);

    [
        distance_score * world.config().optimal_target_distance_score_weight,
    ]
}
