use model::{
    Item,
    Unit,
    UnitAction,
    Vec2F64,
    Weapon,
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
    HitTarget,
    get_hit_damage,
    get_hit_probability_by_spread,
    get_opponent_score_for_hit,
    get_player_score_for_hit,
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
        Some(Target::Mine { rect }) => (true, rect.center() - current_unit.center()),
        Some(Target::Unit(hit_target)) => if let Some(weapon) = current_unit.weapon.as_ref() {
            (
                should_shoot(current_unit, hit_target, weapon, world),
                hit_target.rect().center() - current_unit.center()
            )
        } else {
            (false, Vec2::zero())
        },
        _ => (false, Vec2::zero()),
    };

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
    debug.log(format!("[{}] plan_score={}, transitions: {:?}", current_unit.id, plan.score, plan.transitions.iter().map(|v| (v.kind, v.id)).collect::<Vec<_>>()));

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

fn should_shoot(current_unit: &Unit, target: &HitTarget, weapon: &Weapon, world: &World) -> bool {
    let hit_probability_by_spread = get_hit_probability_by_spread(current_unit.center(), target.rect(),
        weapon.spread, weapon.params.bullet.size);

    if hit_probability_by_spread < world.config().min_hit_probability_by_spread_to_shoot {
        return false;
    }

    let direction = weapon.last_angle.map(|v| Vec2::i().rotated(v))
        .unwrap_or_else(|| (target.rect().center() - current_unit.center()).normalized());
    let number_of_directions = world.config().optimal_action_number_of_directions;
    let hit_damage = get_hit_damage(current_unit.id, current_unit.center(), direction, target,
        weapon.spread, &weapon.params.bullet, &weapon.params.explosion, world, number_of_directions);

    get_player_score_for_hit(&hit_damage, world.properties().kill_score, number_of_directions)
        > get_opponent_score_for_hit(&hit_damage, world.properties().kill_score, number_of_directions)
}
