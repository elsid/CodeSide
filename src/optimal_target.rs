use model::{
    Unit,
    Weapon,
};

use crate::my_strategy::{
    Clamp1,
    HitProbabilities,
    Rectangular,
    Target,
    Vec2,
    World,
    as_score,
    get_hit_probabilities,
    get_hit_probability_by_spread,
};

pub struct AimTarget {
    pub unit_id: i32,
    pub hit_probabilities: HitProbabilities,
}

pub fn get_optimal_target(current_unit: &Unit, world: &World) -> Option<AimTarget> {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        world.units().iter()
            .filter(|unit| {
                world.is_opponent_unit(unit)
                    && get_hit_probability_by_spread(current_unit.center(), &unit.rect(), weapon.params.min_spread, weapon.params.bullet.size)
                        >= world.config().optimal_target_min_hit_probability_by_spread_to_shoot
            })
            .map(|unit| {
                let (score, hit_probabilities) = get_target_score(current_unit.id, current_unit.center(), unit, weapon, &world);
                (unit.id, score, hit_probabilities)
            })
            .max_by_key(|&(_, score, _)| score)
            .map(|(unit_id, _, hit_probabilities)| AimTarget { unit_id, hit_probabilities })
    } else {
        None
    }
}

fn get_target_score(current_unit_id: i32, source: Vec2, opponent: &Unit, weapon: &Weapon, world: &World) -> (i32, HitProbabilities) {
    let destination = opponent.center();
    let last_angle = weapon.last_angle.unwrap_or(0.0);
    let time_to_shoot = weapon.fire_timer.unwrap_or(world.tick_time_interval());
    let rotation_speed = Vec2::i().rotated(last_angle).rotation(destination - source);
    let aim_speed = weapon.params.aim_speed * time_to_shoot;
    let spread = (weapon.spread + rotation_speed - aim_speed).clamp1(weapon.params.min_spread, weapon.params.max_spread);
    let direction = (destination - source).normalized();
    let hit_probabilities = get_hit_probabilities(current_unit_id, source, direction,
        &Target::from_unit(opponent), spread, weapon.params.bullet.size, world,
        world.config().optimal_target_number_of_directions);

    (
        as_score(get_target_score_components(source, destination, &hit_probabilities, world).iter().sum()),
        hit_probabilities
    )
}

fn get_target_score_components(source: Vec2, destination: Vec2, hit_probabilities: &HitProbabilities, world: &World) -> [f64; 3] {
    let distance_score = source.distance(destination) / world.max_distance();

    let hit_opponents_score = (hit_probabilities.target + hit_probabilities.opponent_units) as f64 / hit_probabilities.total as f64;

    let hit_teammates_score = hit_probabilities.teammate_units as f64 / hit_probabilities.total as f64;

    [
        distance_score * world.config().optimal_target_distance_score_weight,
        hit_opponents_score * world.config().optimal_target_hit_opponents_score_weight,
        hit_teammates_score * world.config().optimal_target_hit_teammates_score_weight,
    ]
}
