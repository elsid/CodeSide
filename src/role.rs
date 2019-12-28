use model::{
    Unit,
};

use crate::my_strategy::{
    Positionable,
    Rect,
    Rectangular,
    Vec2,
    World,
};

#[derive(Debug, Clone)]
pub enum Role {
    Shooter,
    Miner {
        plant_mines: usize,
    }
}

pub fn is_shooter(role: &Role) -> bool {
    match role {
        Role::Shooter => true,
        _ => false,
    }
}

pub fn get_role(current_unit: &Unit, prev: &Role, world: &World) -> Role {
    match prev {
        Role::Shooter => {
            let plant_mines = get_mines_to_plant(current_unit, world);
            if plant_mines > 0 {
                Role::Miner { plant_mines }
            } else {
                Role::Shooter
            }
        },
        Role::Miner { plant_mines } => {
            if *plant_mines > 0 || has_collision_with_teammate_mine(current_unit, world) {
                prev.clone()
            } else {
                Role::Shooter
            }
        }
    }
}

fn get_mines_to_plant(current_unit: &Unit, world: &World) -> usize {
    if !current_unit.on_ground || current_unit.on_ladder || current_unit.mines == 0 || current_unit.weapon.is_none() {
        return 0;
    }

    let fire_time = current_unit.weapon.as_ref().map(|v| v.fire_timer.unwrap_or(0.0)).unwrap_or(std::f64::MAX);

    if fire_time >= 2.0 * world.tick_time_interval() {
        return 0;
    }

    let increased_radius = world.properties().mine_explosion_params.radius
        + world.properties().jump_pad_jump_speed * world.tick_time_interval();

    let increased_explosion_rect = Rect::new(current_unit.position(), Vec2::new(increased_radius, increased_radius));

    let collided_teammate_units = world.units().iter()
        .filter(|v| world.is_teammate_unit(v) && v.rect().has_collision(&increased_explosion_rect))
        .collect::<Vec<_>>();

    if collided_teammate_units.len() > 1 {
        return 0;
    }

    let reduced_radius = world.properties().mine_explosion_params.radius
        - world.properties().jump_pad_jump_speed * world.tick_time_interval();

    let reduced_explosion_rect = Rect::new(current_unit.position(), Vec2::new(reduced_radius, reduced_radius));

    let collided_opponent_units = world.units().iter()
        .filter(|v| world.is_opponent_unit(v) && v.rect().has_collision(&reduced_explosion_rect))
        .collect::<Vec<_>>();

    if collided_opponent_units.is_empty() {
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

    if max_my_total_score < world.properties().kill_score * world.properties().team_size {
        return 0;
    }

    optimal_plant_mines
}

fn has_collision_with_teammate_mine(unit: &Unit, world: &World) -> bool {
    world.mines().iter()
        .find(|v| world.is_teammate_mine(v) && v.rect().has_collision(&unit.rect()))
        .is_some()
}