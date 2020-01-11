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
    Vec2,
    World,
    get_hit_probability_by_spread,
    get_weapon_score,
    minimize1d,
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
pub fn get_shooter_action(current_unit: &Unit, plan: &Plan, target: &Option<i32>, world: &World,
        debug: &mut Debug) -> UnitAction {
    let (shoot, aim) = if let (Some(target_unit_id), Some(weapon)) = (target, current_unit.weapon.as_ref()) {
        let target_unit = world.get_unit(*target_unit_id);
        if weapon.params.explosion.is_some() {
            let required_direction = target_unit.center() - current_unit.center();
            let tick_time = world.tick_time_interval();
            let aim_direction = if let Some(last_angle) = weapon.last_angle {
                let current_direction = Vec2::i().rotated(last_angle);
                let required_rotation = required_direction.rotation(current_direction);
                let target_unit_rect = target_unit.rect();
                let max_rotation = minimize1d(1e-3, required_rotation, 10,
                    |v| -get_hit_rate(v, required_rotation, current_unit.center(), &target_unit_rect, weapon, tick_time)
                );

                limit_rotation_to(required_direction, current_direction, max_rotation)
            } else {
                required_direction
            };

            if aim_direction.rotation(required_direction) < 1e-3 {
                (true, aim_direction)
            } else {
                (false, aim_direction)
            }
        } else {
            (true, target_unit.center() - current_unit.center())
        }
    } else {
        (false, Vec2::zero())
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
        + aim_time.max(tick_time_interval) * (rotation - weapon.params.aim_speed)
        - weapon.params.aim_speed * (time - aim_time).max(0.0)
    ).clamp1(weapon.params.min_spread, weapon.params.max_spread);

    get_hit_probability_by_spread(source, target, spread, weapon.params.bullet.size) / (time + time * time / 3.0)
}
