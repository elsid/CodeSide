use model::{
    Unit,
};

use crate::my_strategy::{
    Debug,
    Location,
    Plan,
    Planner,
    Positionable,
    Simulator,
    World,
    XorShiftRng,
};

#[inline(never)]
pub fn get_optimal_plan(current_unit: &Unit, tiles_path: &Vec<Location>, world: &World,
        rng: &mut XorShiftRng, debug: &mut Debug) -> Plan {
    let local_destination = if let Some(v) = tiles_path.first() {
        v.bottom()
    } else {
        current_unit.position()
    };

    let simulator = Simulator::new(&world, current_unit.id);
    let planner = Planner::new(local_destination, world.config(), simulator, world.max_distance(), world.max_score());

    planner.make(world.current_tick(), rng, debug)
}
