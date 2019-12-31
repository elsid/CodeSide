use model::{
    Item,
    Unit,
    UnitAction,
    Vec2F64,
    Weapon,
};

use crate::my_strategy::{
    Clamp1,
    Debug,
    Plan,
    Positionable,
    Rect,
    Rectangular,
    Target,
    Vec2,
    World,
    get_hit_probability_by_spread,
    get_weapon_score,
    minimize1d,
    should_shoot,
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
    let (shoot, aim) = if let (Some(target), Some(weapon)) = (target, current_unit.weapon.as_ref()) {
        let required_direction = target.rect().center() - current_unit.center();
        let tick_time = world.tick_time_interval();

        let (aim_direction, rotation) = if let Some(last_angle) = weapon.last_angle {
            let current_direction = Vec2::i().rotated(last_angle);
            let required_rotation = required_direction.rotation(current_direction);
            let max_rotation = minimize1d(1e-3, required_rotation, 10,
                |v| -get_hit_rate(v, required_rotation, current_unit.center(), target.rect(), weapon, tick_time)
            );
            let aim_direction = limit_rotation_to(required_direction, current_direction, max_rotation);

            (aim_direction, aim_direction.rotation(current_direction))
        } else {
            (required_direction, 0.0)
        };

        let spread = get_spread(rotation, tick_time, weapon);

        if aim_direction.rotation(required_direction) < 1e-3
            && should_shoot(current_unit.id, current_unit.center(), aim_direction, spread, target, weapon, &world) {
            (true, aim_direction)
        } else {
            (false, aim_direction)
        }
    } else {
        (false, Vec2::zero())
    };

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
    debug.log(format!("[{}] plan_score={}, transitions: {:?}", current_unit.id, plan.score, plan.transitions.iter().map(|v| (v.kind, v.id)).collect::<Vec<_>>()));

    let mut action = if plan.transitions.is_empty() {
        default_action()
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

pub fn limit_rotation_to(required_direction: Vec2, current_direction: Vec2, max_rotation: f64) -> Vec2 {
    let rotation = min_abs(sub_angle(required_direction.atan(), current_direction.atan()), max_rotation);
    current_direction.rotated(rotation)
}

fn sub_angle(lhs: f64, rhs: f64) -> f64 {
    let sub = lhs - rhs;
    if sub >= std::f64::consts::PI {
        sub - 2.0 * std::f64::consts::PI
    } else if sub <= -std::f64::consts::PI {
        sub + 2.0 * std::f64::consts::PI
    } else {
        sub
    }
}

fn min_abs(value: f64, min_abs: f64) -> f64 {
    value.abs().min(min_abs).copysign(value)
}

fn get_hit_rate(rotation: f64, required_rotation: f64, source: Vec2, target: &Rect, weapon: &Weapon, tick_time_interval: f64) -> f64 {
    let aim_time = required_rotation / rotation;
    let next_shoot_time = weapon.fire_timer.unwrap_or(0.0).min(tick_time_interval);
    let time = aim_time.max(next_shoot_time);
    let spread = (
        weapon.spread
        + aim_time * (rotation - weapon.params.aim_speed)
        - weapon.params.aim_speed * (time - aim_time).max(0.0)
    ).clamp1(weapon.params.min_spread, weapon.params.max_spread);

    get_hit_probability_by_spread(source, target, spread, weapon.params.bullet.size) / (time + time * time / 3.0)
}

fn get_spread(rotation: f64, tick_time_interval: f64, weapon: &Weapon) -> f64 {
    (
        weapon.spread + tick_time_interval * (rotation - weapon.params.aim_speed)
    ).clamp1(weapon.params.min_spread, weapon.params.max_spread)
}
