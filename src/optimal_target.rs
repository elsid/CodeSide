use model::{
    Unit,
    Weapon,
    WeaponType,
};

use crate::my_strategy::{
    Rectangular,
    Target,
    Vec2,
    World,
    as_score,
    get_hit_probabilities,
};

pub fn get_optimal_target(current_unit: &Unit, world: &World) -> Option<i32> {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        world.units().iter()
            .filter(|unit| world.is_opponent_unit(unit))
            .max_by_key(|unit| get_target_score(current_unit, unit, weapon, world))
            .map(|unit| unit.id)
    } else {
        None
    }
}

pub fn get_target_score(current_unit: &Unit, target: &Unit, weapon: &Weapon, world: &World) -> i32 {
    as_score(get_target_score_components(current_unit, target, weapon, world).iter().sum())
}

pub fn get_target_score_components(current_unit: &Unit, target: &Unit, weapon: &Weapon, world: &World) -> [f64; 4] {
    let direction = (target.center() - current_unit.center()).normalized();

    let distance_score_factor = if weapon.typ == WeaponType::RocketLauncher {
        world.config().optimal_target_rocket_launcher_distance_factor
    } else {
        1.0
    };

    let distance_score = current_unit.center().distance(target.center()) * distance_score_factor / world.max_distance();

    let hit_probabilities = get_hit_probabilities(current_unit.id, current_unit.center(), direction, &Target::from_unit(target), weapon.spread, weapon.params.bullet.size, world);

    let hit_opponent_score = (hit_probabilities.target + hit_probabilities.opponent_units) as f64 / hit_probabilities.total as f64;

    let hit_teammates_score = hit_probabilities.teammate_units as f64 / hit_probabilities.total as f64;

    let aim_time_score = weapon.last_angle
        .map(|angle| Vec2::i().rotated(angle).rotation(direction).abs())
        .unwrap_or(std::f64::consts::PI) /  weapon.params.aim_speed;

    [
        distance_score * world.config().optimal_target_distance_score_weight,
        hit_opponent_score * world.config().optimal_target_hit_opponent_score_weight,
        hit_teammates_score * world.config().optimal_target_hit_teammates_score_weight,
        aim_time_score * world.config().optimal_target_aim_time_score_weight,
    ]
}
