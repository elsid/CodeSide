use model::{
    Item,
    Unit,
    UnitAction,
    Vec2F64,
};

use crate::my_strategy::{
    Debug,
    Plan,
    Positionable,
    Rectangular,
    Target,
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
pub fn get_shooter_action(current_unit: &Unit, plan: &Plan, target: &Option<Target>, world: &World,
        debug: &mut Debug) -> UnitAction {
    let (shoot, aim) = match target {
        Some(Target::Mine { position }) => (true, *position - current_unit.center()),
        Some(Target::Unit { id, shoot }) => (*shoot, world.get_unit(*id).center() - current_unit.center()),
        _ => (false, Vec2::zero()),
    };

    let mut action = if plan.transitions.is_empty() {
        default_action()
    } else {
        plan.transitions[0].get_action(world.properties())
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

pub const fn default_action() -> UnitAction {
    UnitAction {
        velocity: 0.0,
        jump: false,
        jump_down: false,
        shoot: false,
        aim: Vec2F64 { x: 0.0, y: 0.0 },
        reload: false,
        swap_weapon: false,
        plant_mine: false,
    }
}
