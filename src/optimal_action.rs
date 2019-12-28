use model::{
    Item,
    Unit,
    UnitAction,
};

use crate::my_strategy::{
    Debug,
    Plan,
    Positionable,
    Rectangular,
    Vec2,
    World,
    get_weapon_score,
};

pub fn get_miner_action(current_unit: &Unit, plant_mines: usize) -> UnitAction {
    UnitAction {
        velocity: 0.0,
        jump: false,
        jump_down: false,
        shoot: plant_mines == 0,
        aim: (current_unit.position() - current_unit.center()).as_model(),
        reload: false,
        swap_weapon: false,
        plant_mine: plant_mines > 0,
    }
}

#[inline(never)]
pub fn get_shooter_action(current_unit: &Unit, plan: &Plan, target: Option<Vec2>, world: &World,
        debug: &mut Debug) -> UnitAction {
    let (shoot, aim) = if let Some(position) = target {
        (true, position - current_unit.center())
    } else {
        (false, Vec2::zero())
    };

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
    debug.log(format!("[{}] plan_score={}, transitions: {:?}", current_unit.id, plan.score, plan.transitions.iter().map(|v| (v.kind, v.id)).collect::<Vec<_>>()));

    let mut action = if plan.transitions.is_empty() {
        UnitAction {
            velocity: 0.0,
            jump: false,
            jump_down: false,
            shoot: false,
            aim: Vec2::zero().as_model(),
            reload: false,
            swap_weapon: false,
            plant_mine: false,
        }
    } else {
        plan.transitions[0].action.clone()
    };

    action.shoot = shoot;
    action.aim = aim.as_model();
    action.swap_weapon = should_swap_weapon(current_unit, shoot, world);

    action
}

fn should_swap_weapon(current_unit: &Unit, should_shoot: bool, world: &World) -> bool {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        if should_shoot && weapon.magazine > 0 {
            return false;
        }
        match world.tile_item(current_unit.location()) {
            Some(&Item::Weapon { ref weapon_type }) => {
                get_weapon_score(&weapon.typ) < get_weapon_score(weapon_type)
            }
            _ => false,
        }
    } else {
        false
    }
}
