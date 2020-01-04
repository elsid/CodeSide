use model::{
    Unit,
    Weapon,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
use model::{
    ColorF32,
    CustomData,
    Vec2F32,
};

use crate::my_strategy::{
    Debug as Dbg,
    HitTarget,
    Positionable,
    Rect,
    Rectangular,
    Vec2,
    World,
    as_score,
    get_hit_damage,
    get_hit_probabilities,
    get_hit_probability_by_spread,
    get_mean_spread,
    get_opponent_score_for_hit,
    get_player_score_for_hit,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
use crate::my_strategy::{
    ObjectType,
    WalkGrid,
    get_nearest_hit,
    normalize_angle,
};

#[derive(Debug)]
pub enum Target {
    Mine {
        rect: Rect,
    },
    Unit(HitTarget),
}

pub fn get_optimal_target(current_unit: &Unit, world: &World, debug: &mut Dbg) -> Option<Target> {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        let mine_rect = world.mines().iter()
            .find(|mine| world.is_teammate_mine(mine) && mine.position().distance(current_unit.position()) < 2.0 * current_unit.size.x)
            .map(|mine| mine.rect());

        if let Some(rect) = mine_rect {
            return Some(Target::Mine { rect });
        }

        let target_unit = world.units().iter()
            .filter(|unit| world.is_opponent_unit(unit))
            .map(|unit| (unit, get_target_score(current_unit, unit, weapon, world)))
            .max_by_key(|(_, score)| as_score(*score));

        let target = target_unit.map(|(unit, _)| Target::Unit(HitTarget::from_unit(unit)));

        #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
        {
            if let Some((unit, score)) = target_unit {
                render_target(current_unit, unit, world, debug);
                #[cfg(feature = "enable_debug_log")]
                debug.log(format!("[{}] optimal_target: {:?} {:?} {:?}", current_unit.id, target, score,
                    get_target_score_components(current_unit, unit, weapon, world)));
            }
        }

        target
    } else {
        None
    }
}

fn get_target_score(current_unit: &Unit, target: &Unit, weapon: &Weapon, world: &World) -> f64 {
    get_target_score_components(current_unit, target, weapon, world).iter().sum()
}

fn get_target_score_components(current_unit: &Unit, target: &Unit, weapon: &Weapon, world: &World) -> [f64; 2] {
    let current_unit_center = current_unit.center();
    let unit_direction = (target.center() - current_unit.center()).normalized();
    let current_direction = weapon.last_angle.map(|v| Vec2::i().rotated(v)).unwrap_or(unit_direction);

    let current_state_shoot_score = get_shoot_score(current_unit.id, current_unit_center, current_direction,
        weapon.spread, target, weapon, world);

    let possible_state_shoot_score = get_shoot_score(current_unit.id, current_unit_center, unit_direction,
        get_mean_spread(weapon), target, weapon, world);

    [
        current_state_shoot_score,
        possible_state_shoot_score,
    ]
}

fn get_shoot_score(current_unit_id: i32, current_unit_center: Vec2, current_unit_direction: Vec2, spread: f64, opponent: &Unit, weapon: &Weapon, world: &World) -> f64 {
    let hit_probability_by_spread = get_hit_probability_by_spread(current_unit_center, &opponent.rect(), weapon.spread, weapon.params.bullet.size);

    if hit_probability_by_spread < world.config().min_hit_probability_by_spread_to_shoot {
        return 0.0;
    }

    let direction = (opponent.center() - current_unit_center).normalized();
    let hit_probabilities = get_hit_probabilities(current_unit_id, current_unit_center, direction,
        &HitTarget::from_unit(opponent), weapon.spread, weapon.params.bullet.size, world,
        world.config().optimal_action_number_of_directions);

    if weapon.params.explosion.is_some() {
        let number_of_directions = world.config().optimal_action_number_of_directions;
        let hit_damage = get_hit_damage(current_unit_id, current_unit_center, current_unit_direction,
            &HitTarget::from_unit(opponent), spread, &weapon.params.bullet, &weapon.params.explosion, world,
            number_of_directions);

        if hit_damage.teammate_units_kills > 0 || hit_damage.shooter_kills > 0
                || hit_damage.shooter_damage_from_teammate > weapon.params.bullet.damage
                || hit_damage.teammate_units_damage_from_teammate > weapon.params.bullet.damage {
            return -(
                hit_damage.shooter_damage_from_teammate
                + hit_damage.shooter_damage_from_opponent
                + hit_damage.teammate_units_damage_from_opponent
                + hit_damage.teammate_units_damage_from_teammate
                + (hit_damage.shooter_kills + hit_damage.teammate_units_kills) as i32 * world.properties().kill_score
            ) as f64 / number_of_directions as f64;
        }

        return get_player_score_for_hit(&hit_damage, world.properties().kill_score, number_of_directions)
            - get_opponent_score_for_hit(&hit_damage, world.properties().kill_score, number_of_directions);
    }

    if (hit_probabilities.target + hit_probabilities.opponent_units) >= world.config().min_target_hits_to_shoot
            && hit_probabilities.teammate_units <= world.config().max_teammates_hits_to_shoot {
        return 0.0;
    }

    world.max_distance() - current_unit_center.distance(opponent.center())
}

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
fn render_target(unit: &Unit, opponent: &Unit, world: &World, debug: &mut Dbg) {
    for position in WalkGrid::new(unit.rect().center(), opponent.rect().center()) {
        debug.draw(CustomData::Rect {
            pos: position.as_location().as_debug(),
            size: Vec2F32 { x: 1.0, y: 1.0 },
            color: ColorF32 { a: 0.5, r: 0.66, g: 0.0, b: 0.66 },
        });
    }
    if let Some(weapon) = unit.weapon.as_ref() {
        let source = unit.rect().center();
        let direction = (opponent.rect().center() - source).normalized();
        let to_target = direction * world.max_distance();
        let left = direction.left() * weapon.params.bullet.size;
        let right = direction.right() * weapon.params.bullet.size;
        let number_of_directions = world.config().optimal_action_number_of_directions;

        for i in 0 .. number_of_directions {
            let angle = ((2 * i) as f64 / (number_of_directions - 1) as f64 - 1.0) * weapon.spread;
            let destination = source + to_target.rotated(normalize_angle(angle));
            let (src, dst) = if i == 0 {
                (source + right, destination + right)
            } else if i == number_of_directions - 1 {
                (source + left, destination + left)
            } else {
                (source, destination)
            };
            if let Some(hit) = get_nearest_hit(unit.id, src, dst, &HitTarget::from_unit(opponent), &world) {
                let color = match hit.object_type {
                    ObjectType::Wall => ColorF32 { a: 0.5, r: 0.66, g: 0.66, b: 0.66 },
                    ObjectType::Unit => if hit.is_teammate {
                        ColorF32 { a: 0.5, r: 0.66, g: 0.33, b: 0.0 }
                    } else {
                        ColorF32 { a: 0.5, r: 0.0, g: 0.66, b: 0.33 }
                    },
                    ObjectType::Mine => if hit.is_teammate {
                        ColorF32 { a: 0.5, r: 0.33, g: 0.5, b: 0.0 }
                    } else {
                        ColorF32 { a: 0.5, r: 0.5, g: 0.33, b: 0.0 }
                    },
                };
                debug.draw(CustomData::Line {
                    p1: src.as_debug(),
                    p2: (src + (dst - src).normalized() * hit.distance).as_debug(),
                    width: 0.075,
                    color,
                });
            }
        }
    }
}
