use model::{
    Item,
    Unit,
    UnitAction,
    Vec2F64,
    Weapon,
};

use crate::my_strategy::{
    Clamp1,
    Debug as Dbg,
    Plan,
    Positionable,
    Rect,
    Rectangular,
    Target,
    Vec2,
    World,
    get_hit_probability_by_spread,
    get_weapon_score,
    is_allowed_to_shoot_with_direction,
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
pub fn get_shooter_action(current_unit: &Unit, plan: &Plan, target: &Option<Target>, world: &World,
        debug: &mut Dbg) -> UnitAction {
    let (shoot, aim) = match (target, current_unit.weapon.as_ref()) {
        (Some(Target::Mine { position }), Some(_)) => (true, *position - current_unit.center()),
        (Some(Target::Unit { id, shoot }), Some(weapon)) => {
            let target_unit = world.get_unit(*id);
            let target_center = target_unit.center();
            let target_rect = target_unit.rect();
            let required_direction = (target_center - current_unit.center()).normalized();

            let aim_direction = if weapon.params.explosion.is_some() {
                let tick_time = world.tick_time_interval();

                if let Some(last_angle) = weapon.last_angle {
                    let current_direction = Vec2::i().rotated(last_angle);
                    let required_rotation = required_direction.rotation(current_direction);
                    let max_rotation = minimize1d(1e-3, required_rotation, 10,
                        |v| -get_hit_rate(v, required_rotation, current_unit.center(), &target_rect, weapon, tick_time)
                    );
                    let aim_direction = limit_rotation_to(required_direction, current_direction, max_rotation);

                    aim_direction
                } else {
                    required_direction
                }
            } else {
                required_direction
            };

            (
                *shoot && should_shoot(current_unit, aim_direction, target_unit, weapon, world, debug),
                aim_direction
            )
        },
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

fn should_shoot(current_unit: &Unit, aim_direction: Vec2, opponent: &Unit, weapon: &Weapon, world: &World, debug: &mut Dbg) -> bool {
    let spread = if let Some(last_angle) = weapon.last_angle {
        let current_direction = Vec2::i().rotated(last_angle);
        let aim_rotation = aim_direction.rotation(current_direction);

        (weapon.spread + (aim_rotation - weapon.params.aim_speed) * world.tick_time_interval())
            .clamp1(weapon.params.min_spread, weapon.params.max_spread)
    } else {
        weapon.spread
    };

    is_allowed_to_shoot_with_direction(current_unit.id, current_unit.center(), aim_direction, spread, opponent,
        weapon, world, world.config().optimal_action_number_of_directions, &mut Some(debug))
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

    get_hit_probability_by_spread(source, target, spread, weapon.params.bullet.size) / time
}
