use model::{
    Unit,
    Weapon,
};

use crate::my_strategy::{
    Positionable,
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
            .filter(|unit| {
                world.is_opponent_unit(unit)
                && should_shoot(current_unit.id, current_unit.center(), &unit, weapon, &world)
            })
            .min_by_key(|unit| as_score(current_unit.position().distance(unit.position())))
            .map(|unit| unit.id)
    } else {
        None
    }
}

fn should_shoot(current_unit_id: i32, current_unit_center: Vec2, opponent: &Unit, weapon: &Weapon, world: &World) -> bool {
    let hit_probability_by_spread = get_hit_probability_by_spread(current_unit_center, &opponent.rect(), weapon.spread, weapon.params.bullet.size);

    if hit_probability_by_spread < world.config().optimal_target_min_hit_probability_by_spread_to_shoot {
        return false;
    }

    let direction = (opponent.center() - current_unit_center).normalized();
    let hit_probabilities = get_hit_probabilities(current_unit_id, current_unit_center, direction,
        &Target::from_unit(opponent), weapon.spread, weapon.params.bullet.size, world,
        world.config().optimal_target_number_of_directions);

    if let (Some(explosion), Some(min_distance)) = (weapon.params.explosion.as_ref(), hit_probabilities.min_distance) {
        if min_distance < explosion.radius + 2.0 {
            return false;
        }
    }

    (hit_probabilities.target + hit_probabilities.opponent_units) >= world.config().optimal_target_min_target_hits_to_shoot
    && hit_probabilities.teammate_units <= world.config().optimal_target_max_teammates_hits_to_shoot
}
