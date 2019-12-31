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
    Clamp1,
    Debug,
    Positionable,
    Rectangular,
    Vec2,
    World,
    as_score,
    get_shoot_score,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
use crate::my_strategy::{
    WalkGrid,
    normalize_angle,
};

pub enum Target {
    Mine {
        position: Vec2,
    },
    Unit {
        id: i32,
        shoot: bool,
    },
}

pub fn get_optimal_target(current_unit: &Unit, world: &World, debug: &mut Debug) -> Option<Target> {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        let mine = world.mines().iter()
            .find(|mine| world.is_teammate_mine(mine) && mine.position().distance(current_unit.position()) < 2.0 * current_unit.size.x)
            .map(|mine| mine.center());

        if let Some(position) = mine {
            return Some(Target::Mine { position });
        }

        let target_by_score = world.units().iter()
            .filter(|unit| world.is_opponent_unit(unit))
            .map(|unit| (unit.id, get_target_score(current_unit.id, current_unit.center(), &unit, weapon, &world)))
            .max_by_key(|(_, score)| *score);

        if let Some((unit_id, score)) = target_by_score {
            if score > 0 {
                #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
                render_target(current_unit, world.get_unit(unit_id), world, debug);

                return Some(Target::Unit { id: unit_id, shoot: true });
            }
        }

        let target_by_distance = world.units().iter()
            .filter(|unit| world.is_opponent_unit(unit))
            .min_by_key(|unit| as_score(current_unit.position().distance(unit.position())));

        #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
        {
            if let Some(unit) = target_by_distance {
                render_target(current_unit, unit, world, debug);
            }
        }

        target_by_distance.map(|unit| Target::Unit { id: unit.id, shoot: false })
    } else {
        None
    }
}

fn get_target_score(current_unit_id: i32, current_unit_center: Vec2, opponent: &Unit, weapon: &Weapon, world: &World) -> i32 {
    let spread = if let Some(last_angle) = weapon.last_angle {
        let current_direction = Vec2::i().rotated(last_angle);
        let required_direction = opponent.center() - current_unit_center;
        let required_rotation = required_direction.rotation(current_direction);

        (weapon.spread + (required_rotation - weapon.params.aim_speed) * world.tick_time_interval())
            .clamp1(weapon.params.min_spread, weapon.params.max_spread)
    } else {
        weapon.spread
    };

    get_shoot_score(current_unit_id, current_unit_center, spread, opponent, weapon,
        world, world.config().optimal_action_number_of_directions, &mut None)
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
        debug.draw(CustomData::Line {
            p1: unit.rect().center().as_debug(),
            p2: opponent.rect().center().as_debug(),
            width: 0.075,
            color: ColorF32 { a: 0.5, r: 0.66, g: 0.0, b: 0.0 },
        });
    }
}
