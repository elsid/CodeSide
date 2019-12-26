use model::{
    Unit,
    Weapon,
};

#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
};

use crate::my_strategy::{
    Clamp1,
    Debug,
    HitProbabilities,
    Rectangular,
    Simulator,
    Target,
    Vec2,
    World,
    XorShiftRng,
    as_score,
    get_hit_probabilities,
    get_hit_probability_by_spread,
};

pub struct AimTarget {
    pub unit_id: i32,
    pub hit_probabilities: HitProbabilities,
    pub position: Vec2,
}

pub fn get_optimal_target(current_unit: &Unit, world: &World, rng: &mut XorShiftRng, debug: &mut Debug) -> Option<AimTarget> {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        let candidates = world.units().iter()
            .filter(|unit| {
                world.is_opponent_unit(unit)
                    && get_hit_probability_by_spread(current_unit.center(), &unit.rect(), weapon.params.min_spread, weapon.params.bullet.size)
                        >= world.config().optimal_target_min_hit_probability_by_spread_to_shoot
            })
            .collect::<Vec<_>>();

        if candidates.is_empty() {
            return None;
        }

        let current_unit_center = current_unit.center();
        let mut times_to_hit = candidates.iter()
            .filter(|v| !v.jump_state.can_cancel)
            .map(|v| (v.id, v.center().distance(current_unit_center) / weapon.params.bullet.speed))
            .collect::<Vec<_>>();

        if times_to_hit.is_empty() {
            return candidates.iter()
                .map(|unit| {
                    let (score, hit_probabilities) = get_target_score(current_unit.id, current_unit.center(), unit, weapon, &world);
                    (unit.id, score, hit_probabilities, unit.center())
                })
                .max_by_key(|&(_, score, _, _)| score)
                .map(|(unit_id, _, hit_probabilities, position)| AimTarget { unit_id, hit_probabilities, position })
        }

        times_to_hit.sort_by_key(|&(_, time)| as_score(time));

        let time_to_shoot = weapon.fire_timer.unwrap_or(0.0);
        let mut simulator = Simulator::new(&world, current_unit.id);

        if simulator.current_time() < time_to_shoot {
            simulator.tick(time_to_shoot, (time_to_shoot / world.tick_time_interval()).ceil() as usize, rng);
        }

        #[cfg(feature = "enable_debug")]
        for unit in simulator.units().iter() {
            let rect = unit.base().rect();
            debug.draw(CustomData::Rect {
                pos: (rect.center() - rect.half()).as_model_f32(),
                size: (rect.half() * 2.0).as_model_f32(),
                color: ColorF32 { a: 0.66, r: 0.0, g: 0.33, b: 1.0 },
            });
        }

        let optimal_by_prediction = times_to_hit.iter()
            .map(|&(unit_id, time_to_hit)| {
                if simulator.current_time() < time_to_shoot + time_to_hit {
                    simulator.tick(time_to_shoot + time_to_hit - simulator.current_time(), (time_to_hit / world.tick_time_interval()).ceil() as usize, rng);
                }
                let unit = simulator.units().iter().find(|v| v.base().id == unit_id).unwrap().base();
                let (score, hit_probabilities) = get_target_score(current_unit.id, current_unit.center(), unit, weapon, &world);
                (unit.id, score, hit_probabilities, unit.center())
            })
            .max_by_key(|&(_, score, _, _)| score);

        let optimal_by_current_state = candidates.iter()
            .filter(|unit| times_to_hit.iter().find(|(id, _)| *id == unit.id).is_none())
            .map(|unit| {
                let (score, hit_probabilities) = get_target_score(current_unit.id, current_unit.center(), unit, weapon, &world);
                (unit.id, score, hit_probabilities, unit.center())
            })
            .max_by_key(|&(_, score, _, _)| score);

        if let (Some(by_prediction), Some(by_current_state)) = (optimal_by_prediction.as_ref(), optimal_by_current_state.as_ref()) {
            if by_prediction.1 > by_current_state.1 {
                Some(AimTarget { unit_id: by_prediction.0, hit_probabilities: by_prediction.2.clone(), position: by_prediction.3 })
            } else {
                Some(AimTarget { unit_id: by_current_state.0, hit_probabilities: by_current_state.2.clone(), position: by_prediction.3 })
            }
        } else if let Some((unit_id, _, hit_probabilities, position)) = optimal_by_prediction {
            Some(AimTarget { unit_id, hit_probabilities, position })
        } else if let Some((unit_id, _, hit_probabilities, position)) = optimal_by_current_state {
            Some(AimTarget { unit_id, hit_probabilities, position })
        } else {
            None
        }
    } else {
        None
    }
}

fn get_target_score(current_unit_id: i32, source: Vec2, opponent: &Unit, weapon: &Weapon, world: &World) -> (i32, HitProbabilities) {
    let destination = opponent.center();

    let spread = if let Some(last_angle) = weapon.last_angle {
        let time_to_shoot = weapon.fire_timer.unwrap_or(0.0);
        let rotation_speed = Vec2::i().rotated(last_angle).rotation(destination - source);
        let aim_speed = weapon.params.aim_speed * (world.tick_time_interval() + time_to_shoot);
        (weapon.spread + rotation_speed - aim_speed).clamp1(weapon.params.min_spread, weapon.params.max_spread)
    } else {
        weapon.spread
    };

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
