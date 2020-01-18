use model::{
    Tile,
    Unit,
};

use crate::my_strategy::{
    Debug as Dbg,
    Positionable,
    Rect,
    Rectangular,
    Vec2,
    Vec2i,
    World,
};

#[derive(Debug, Clone)]
pub enum Role {
    Shooter,
    Miner {
        plant_mines: usize,
        planted_mines: usize,
    }
}

pub fn get_role(unit: &Unit, prev: &Role, world: &World, debug: &mut Dbg) -> Role {
    if let Role::Miner { planted_mines, plant_mines } = prev {
        if *planted_mines == 0 && *planted_mines < *plant_mines
                || (has_collision_with_teammate_mine(unit, world) && will_explode_opponent_units(unit, world)) {
            return prev.clone();
        }
        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        debug.log(format!("[{}] abort miner plant_mines={} planted_mines={} has_collision_with_teammate_mine={} will_explode_opponent_units={}",
            unit.id, plant_mines, planted_mines, has_collision_with_teammate_mine(unit, world), will_explode_opponent_units(unit, world)));
    } else {
        let plant_mines = get_mines_to_plant(unit, world, debug);
        if plant_mines > 0 {
            return Role::Miner { plant_mines, planted_mines: 0 };
        }
    }

    Role::Shooter
}

pub fn update_role(unit: &Unit, role: &Role, world: &World) -> Role {
    match role {
        Role::Miner { plant_mines, .. } => Role::Miner {
            plant_mines: (unit.mines as usize).min(*plant_mines),
            planted_mines: world.mines().iter()
                .filter(|mine| world.is_teammate_mine(mine) && mine.rect().has_collision(&unit.rect()))
                .count()
        },
        v => v.clone(),
    }
}

fn get_mines_to_plant(current_unit: &Unit, world: &World, debug: &mut Dbg) -> usize {
    if current_unit.on_ladder || current_unit.mines == 0 || current_unit.weapon.is_none()
            || current_unit.position.y - current_unit.position.y.floor() > world.properties().unit_fall_speed * 2.0 as f64 * world.tick_time_interval() {
        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        debug.log(format!("[{}] reject miner 1", current_unit.id));
        return 0;
    }

    let tile = world.get_tile(current_unit.position().as_location() + Vec2i::only_y(-1));
    if tile != Tile::Wall && tile != Tile::Platform {
        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        debug.log(format!("[{}] reject miner 2", current_unit.id));
        return 0;
    }

    let fire_time = current_unit.weapon.as_ref().map(|v| v.fire_timer.unwrap_or(0.0)).unwrap_or(std::f64::MAX);

    if fire_time >= 2.0 * world.tick_time_interval() {
        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        debug.log(format!("[{}] reject miner 3", current_unit.id));
        return 0;
    }

    let mine_center = Vec2::new(current_unit.position.x, current_unit.position.y.floor())
        + Vec2::only_y(world.properties().mine_size.y / 2.0);

    let increased_radius = world.properties().mine_explosion_params.radius
        + world.properties().jump_pad_jump_speed * world.tick_time_interval();

    let increased_explosion_rect = Rect::new(mine_center, Vec2::new(increased_radius, increased_radius));

    let collided_teammate_units = world.units().iter()
        .filter(|v| world.is_teammate_unit(v) && v.rect().has_collision(&increased_explosion_rect))
        .collect::<Vec<_>>();

    if collided_teammate_units.len() > 1 {
        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        debug.log(format!("[{}] reject miner 4", current_unit.id));
        return 0;
    }

    let reduced_radius = world.properties().mine_explosion_params.radius
        - 2.0 * world.properties().jump_pad_jump_speed * world.tick_time_interval();

    let reduced_explosion_rect = Rect::new(mine_center, Vec2::new(reduced_radius, reduced_radius));

    let collided_opponent_units = world.units().iter()
        .filter(|v| world.is_opponent_unit(v) && v.rect().has_collision(&reduced_explosion_rect))
        .collect::<Vec<_>>();

    if collided_opponent_units.is_empty() {
        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        debug.log(format!("[{}] reject miner 5", current_unit.id));
        return 0;
    }

    let mut optimal_plant_mines = 0;
    let mut max_score_diff = 0;
    let mut max_my_total_score = 0;

    for plant_mines in 1 .. (current_unit.mines + 1) as usize {
        let my_explosion_score = collided_opponent_units.iter()
            .map(|v| {
                if v.health > world.properties().mine_explosion_params.damage * plant_mines as i32 {
                    v.health
                } else {
                    v.health + world.properties().kill_score
                }
            })
            .sum::<i32>();

        let opponent_explosion_score = collided_teammate_units.iter()
            .map(|v| {
                if v.health > world.properties().mine_explosion_params.damage * plant_mines as i32 {
                    0
                } else {
                    world.properties().kill_score
                }
            })
            .sum::<i32>();

        let my_total_score = my_explosion_score + world.my_player().score;

        let opponent_total_score = opponent_explosion_score + world.opponent_player().score;

        let score_diff = my_total_score - opponent_total_score;

        if max_score_diff < score_diff {
            max_score_diff = score_diff;
            optimal_plant_mines = plant_mines;
            max_my_total_score = my_total_score;
        }

        if my_total_score >= world.properties().kill_score * world.properties().team_size {
            break;
        }
    }

    if max_my_total_score < world.properties().kill_score {
        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        debug.log(format!("[{}] reject miner 6 max_my_total_score={} limit={}",
            current_unit.id, max_my_total_score, world.properties().kill_score));
        return 0;
    }

    optimal_plant_mines
}

fn has_collision_with_teammate_mine(unit: &Unit, world: &World) -> bool {
    world.mines().iter()
        .find(|v| world.is_teammate_mine(v) && v.rect().has_collision(&unit.rect()))
        .is_some()
}

fn will_explode_opponent_units(miner: &Unit, world: &World) -> bool {
    world.mines().iter()
        .filter(|mine| world.is_teammate_mine(mine) && mine.rect().has_collision(&miner.rect()))
        .find(|mine| {
            let radius = world.properties().mine_explosion_params.radius;
            let explosion_rect = Rect::new(mine.position(), Vec2::new(radius, radius));
            world.units().iter()
                .find(|unit| world.is_opponent_unit(unit) && explosion_rect.has_collision(&unit.rect()))
                .is_some()
        })
        .is_some()
}
