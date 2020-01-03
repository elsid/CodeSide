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
    Debug,
    Positionable,
    Rectangular,
    Target,
    Vec2,
    World,
    as_score,
    get_hit_damage,
    get_hit_probability_by_spread,
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

pub fn get_optimal_target(current_unit: &Unit, world: &World, debug: &mut Debug) -> Option<Vec2> {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        let mine = world.mines().iter()
            .find(|mine| world.is_teammate_mine(mine) && mine.position().distance(current_unit.position()) < 2.0 * current_unit.size.x)
            .map(|mine| mine.center());

        if let Some(position) = mine {
            return Some(position);
        }

        let unit = world.units().iter()
            .filter(|unit| {
                world.is_opponent_unit(unit)
                && should_shoot(current_unit.id, current_unit.center(), &unit, weapon, &world)
            })
            .min_by_key(|unit| as_score(current_unit.position().distance(unit.position())));

        #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
        {
            if let Some(opponent) = unit {
                render_target(current_unit, opponent, world, debug);
            }
        }

        unit.map(|unit| unit.center())
    } else {
        None
    }
}

fn should_shoot(current_unit_id: i32, current_unit_center: Vec2, opponent: &Unit, weapon: &Weapon, world: &World) -> bool {
    let hit_probability_by_spread = get_hit_probability_by_spread(current_unit_center, &opponent.rect(), weapon.spread, weapon.params.bullet.size);

    if hit_probability_by_spread < world.config().min_hit_probability_by_spread_to_shoot {
        return false;
    }

    let direction = (opponent.center() - current_unit_center).normalized();
    let number_of_directions = world.config().optimal_action_number_of_directions;
    let hit_damage = get_hit_damage(current_unit_id, current_unit_center, direction, &Target::from_unit(opponent),
        weapon.spread, &weapon.params.bullet, &weapon.params.explosion, world, number_of_directions);

    get_player_score_for_hit(&hit_damage, world.properties().kill_score, number_of_directions)
        > get_opponent_score_for_hit(&hit_damage, world.properties().kill_score, number_of_directions)
}

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
fn render_target(unit: &Unit, opponent: &Unit, world: &World, debug: &mut Debug) {
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
            if let Some(hit) = get_nearest_hit(unit.id, src, dst, &Target::from_unit(opponent), &world) {
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
