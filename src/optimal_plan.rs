use model::{
    Unit,
    UnitAction,
};

#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
};

use crate::my_strategy::{
    Debug,
    Plan,
    Planner,
    Positionable,
    Simulator,
    Vec2,
    World,
    XorShiftRng,
};

#[cfg(feature = "enable_debug")]
use crate::my_strategy::{
    Location,
    Rectangular,
};

#[inline(never)]
pub fn get_optimal_plan(current_unit: &Unit, global_destination: Vec2, other: &[(i32, Plan)], world: &World,
        rng: &mut XorShiftRng, debug: &mut Debug) -> Plan {
    let tiles_path = world.find_shortcut_tiles_path(current_unit.id, current_unit.location(), global_destination.as_location());

    #[cfg(feature = "enable_debug")]
    render_tiles_path(current_unit, &tiles_path, debug);

    let local_destination = if !tiles_path.is_empty() {
        tiles_path[0].bottom()
    } else {
        global_destination
    };

    #[cfg(feature = "enable_debug")]
    debug.log(format!("[{}] global_destination: {:?} local_destination: {:?}", current_unit.id, global_destination, local_destination));

    let simulator = Simulator::new(&world, current_unit.id);
    let planner = Planner::new(local_destination, world.config(), simulator, world.max_distance(),
        world.max_score(), make_get_unit_action_at(other));

    planner.make(world.current_tick(), rng, debug)
}

pub fn make_get_unit_action_at<'r>(other: &'r [(i32, Plan)]) -> impl Clone + Fn(i32, i32) -> Option<&'r UnitAction> {
    move |unit_id: i32, tick: i32| -> Option<&'r UnitAction> {
        other.iter()
            .find(|(id, _)| *id == unit_id)
            .map(|(_, plan)| {
                plan.transitions.get(tick as usize)
                    .map(|transition| &transition.action)
            })
            .unwrap_or(None)
    }
}

#[cfg(feature = "enable_debug")]
fn render_tiles_path(unit: &Unit, tiles_path: &Vec<Location>, debug: &mut Debug) {
    if tiles_path.is_empty() {
        return;
    }

    debug.draw(CustomData::Line {
        p1: unit.center().as_model_f32(),
        p2: tiles_path[0].center().as_model_f32(),
        width: 0.1,
        color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
    });

    for tile in 0 .. tiles_path.len() - 1 {
        debug.draw(CustomData::Line {
            p1: tiles_path[tile].center().as_model_f32(),
            p2: tiles_path[tile + 1].center().as_model_f32(),
            width: 0.1,
            color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
        });
    }
}
