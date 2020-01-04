use model::{
    Unit,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_plan"))]
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

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_plan"))]
use crate::my_strategy::{
    Location,
    Rectangular,
};

#[inline(never)]
pub fn get_optimal_plan(current_unit: &Unit, global_destination: Vec2, world: &World,
        rng: &mut XorShiftRng, debug: &mut Debug) -> Plan {
    let tiles_path = world.find_shortcut_tiles_path(current_unit.id, current_unit.location(), global_destination.as_location());

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_plan"))]
    render_tiles_path(current_unit, &tiles_path, debug);

    let local_destination = if !tiles_path.is_empty() {
        tiles_path[0].bottom()
    } else {
        global_destination
    };

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_plan", feature = "enable_debug_log"))]
    debug.log(format!("[{}] global_destination: {:?} local_destination: {:?} tiles_path: {:?}", current_unit.id, global_destination, local_destination, tiles_path));

    let simulator = Simulator::new(&world, current_unit.id);
    let planner = Planner::new(local_destination, world.config(), simulator, world.max_distance(), world.max_score());

    planner.make(world.current_tick(), rng, debug)
}

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_plan"))]
fn render_tiles_path(unit: &Unit, tiles_path: &Vec<Location>, debug: &mut Debug) {
    if tiles_path.is_empty() {
        return;
    }

    debug.draw(CustomData::Line {
        p1: unit.center().as_debug(),
        p2: tiles_path[0].center().as_debug(),
        width: 0.1,
        color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
    });

    for tile in 0 .. tiles_path.len() - 1 {
        debug.draw(CustomData::Line {
            p1: tiles_path[tile].center().as_debug(),
            p2: tiles_path[tile + 1].center().as_debug(),
            width: 0.1,
            color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
        });
    }
}
