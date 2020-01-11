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
    Vec2,
    World,
    as_score,
    is_allowed_to_shoot,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
use crate::my_strategy::{
    WalkGrid,
    normalize_angle,
};

pub fn get_optimal_target(current_unit: &Unit, world: &World, debug: &mut Debug) -> Option<i32> {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        let unit = world.units().iter()
            .filter(|unit| {
                world.is_opponent_unit(unit)
                && should_shoot(current_unit.id, current_unit.center(), &unit, weapon, &world, debug)
            })
            .min_by_key(|unit| as_score(current_unit.position().distance(unit.position())));

        #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_target"))]
        {
            if let Some(opponent) = unit {
                render_target(current_unit, opponent, world, debug);
            }
        }

        unit.map(|unit| unit.id)
    } else {
        None
    }
}

fn should_shoot(current_unit_id: i32, current_unit_center: Vec2, opponent: &Unit, weapon: &Weapon, world: &World, debug: &mut Debug) -> bool {
    is_allowed_to_shoot(current_unit_id, current_unit_center, weapon.spread, opponent, weapon,
        world, world.config().optimal_action_number_of_directions, &mut Some(debug))
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
