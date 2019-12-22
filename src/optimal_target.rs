use model::{
    Unit,
};

use crate::my_strategy::{
    Positionable,
    Rectangular,
    World,
    as_score,
    should_shoot,
};

pub fn get_optimal_target(current_unit: &Unit, world: &World) -> Option<i32> {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        world.units().iter()
            .filter(|unit| {
                world.is_opponent_unit(unit)
                && should_shoot(current_unit.id, current_unit.center(), &unit, weapon, &world,
                    true, world.config().optimal_action_number_of_directions)
            })
            .min_by_key(|unit| as_score(current_unit.position().distance(unit.position())))
            .map(|unit| unit.id)
    } else {
        None
    }
}
