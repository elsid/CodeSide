#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
use model::{
    CustomData,
    Vec2F32,
};

use model::{
    Item,
    Tile,
    Unit,
    Weapon,
    WeaponType,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
use crate::my_strategy::{
    color_from_heat,
};

use crate::my_strategy::{
    Debug,
    Location,
    Positionable,
    Rect,
    Rectangular,
    TilePathInfo,
    Vec2,
    Vec2i,
    World,
    as_score,
    get_hit_probability_by_spread_with_destination,
    is_allowed_to_shoot,
    simulate_shoot,
};

#[inline(never)]
pub fn get_optimal_location(unit: &Unit, optimal_locations: &Vec<(i32, Option<Location>)>, world: &World, debug: &mut Debug) -> Option<(f64, Location)> {
    let mut optimal: Option<(f64, Location)> = None;

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
    let mut tiles: Vec<Option<f64>> = std::iter::repeat(None)
        .take(world.level().size())
        .collect();

    let unit_index = world.get_unit_index(unit.id);

    for x in 1 .. world.level().size_x() - 1 {
        for y in 1 .. world.level().size_y() - 2 {
            let location = Location::new(x, y);
            let tile = world.get_tile(location);
            if tile == Tile::Wall || is_busy_by_other(location, unit.id, optimal_locations, world) {
                continue;
            }
            if let Some(path_info) = world.get_path_info(unit_index, location) {
                let candidate_score = get_location_score(location, unit, world, &path_info);
                if optimal.is_none() || optimal.unwrap().0 < candidate_score {
                    optimal = Some((candidate_score, location));
                }
                #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
                {
                    tiles[world.level().get_tile_index(location)] = Some(candidate_score);
                }
            }
        }
    }

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
    {
        let min = tiles.iter().filter_map(|&v| v).min_by_key(|&v| as_score(v)).unwrap();
        let max = tiles.iter().filter_map(|&v| v).max_by_key(|&v| as_score(v)).unwrap();
        for x in 1 .. world.level().size_x() - 1 {
            for y in 1 .. world.level().size_y() - 2 {
                let location = Location::new(x, y);
                if let Some(score) = tiles[world.level().get_tile_index(location)] {
                    debug.draw(CustomData::Rect {
                        pos: location.as_debug(),
                        size: Vec2F32 { x: 1.0, y: 1.0 },
                        color: color_from_heat(0.1, ((score - min) / (max - min)) as f32),
                    });
                }
            }
        }
        #[cfg(feature = "enable_debug_log")]
        {
            if let Some((score, location)) = optimal {
                let path_info = world.get_path_info(unit_index, location).unwrap();
                debug.log(format!("[{}] optimal_location: {:?} {:?} {:?}", unit.id, location, score, get_location_score_components(location, unit, world, &path_info)));
                if let Some(v) = optimal_locations.iter().find(|(id, _)| *id == unit.id).unwrap().1 {
                    if let Some(path_info) = world.get_path_info(unit_index, v) {
                        debug.log(format!("[{}] previous_location: {:?} {:?} {:?}", unit.id, v, score, get_location_score_components(v, unit, world, &path_info)));
                    }
                }
            }
        }
    }

    optimal
}

fn is_busy_by_other(location: Location, unit_id: i32, optimal_locations: &Vec<(i32, Option<Location>)>, world: &World) -> bool {
    for i in 0 .. optimal_locations.len() {
        if optimal_locations[i].0 == unit_id {
            continue;
        }
        if let Some(v) = optimal_locations[i].1 {
            if v == location || v + Vec2i::only_y(-1) == location {
                return true;
            }
        }
    }
    world.units().iter()
        .filter(|v| v.id != unit_id)
        .find(|v| {
            if !world.is_complex_level() || world.is_opponent_unit(v) {
                v.rect().has_collision(&make_location_rect(location))
            } else if world.is_teammate_unit(v) {
                let radius = world.properties().mine_explosion_params.radius;
                let explosion_rect = Rect::new(location.bottom(), Vec2::new(radius, radius));

                v.rect().has_collision(&explosion_rect)
            } else {
                false
            }
        })
        .is_some()
}

pub fn get_location_score(location: Location, current_unit: &Unit, world: &World, path_info: &TilePathInfo) -> f64 {
    get_location_score_components(location, current_unit, world, path_info).iter().sum()
}

pub fn get_location_score_components(location: Location, current_unit: &Unit, world: &World, path_info: &TilePathInfo) -> [f64; 17] {
    let current_unit_position = Vec2::new(location.x() as f64 + 0.5, location.y() as f64);
    let current_unit_center = Vec2::new(location.x() as f64 + 0.5, location.y() as f64 + current_unit.size.y * 0.5);
    let location_rect = Rect::new(
        Vec2::new(location.x() as f64 + 0.5, location.y() as f64 + 0.5),
        Vec2::new(0.5, 0.5)
    );
    let distance_to_position_score = path_info.distance() / world.max_path_distance();
    let health_pack_score = match world.tile_item(location) {
        Some(&Item::HealthPack { .. }) => 1.0 - current_unit.health as f64 / world.properties().unit_max_health as f64,
        _ => 0.0,
    };
    let first_weapon_score = if current_unit.weapon.is_none() {
        match world.tile_item(location) {
            Some(&Item::Weapon { .. }) => 1.0 - distance_to_position_score,
            _ => 0.0,
        }
    } else {
        0.0
    };
    let swap_weapon_score = (current_unit.weapon.is_some() && match world.tile_item(location) {
        Some(&Item::Weapon { ref weapon_type }) => {
            get_weapon_score(&current_unit.weapon.as_ref().unwrap().typ) < get_weapon_score(weapon_type)
        },
        _ => false,
    }) as i32 as f64;
    let hit_by_opponent_score = world.units().iter()
        .filter(|unit| world.is_opponent_unit(unit))
        .map(|unit| {
            if let Some(weapon) = unit.weapon.as_ref() {
                let fire_timer_factor = get_fire_timer_factor(weapon.fire_timer, world.config().optimal_location_min_fire_timer);
                if fire_timer_factor > 0.0 {
                    let direction = (current_unit_center - unit.center()).normalized();
                    let shoot_result = simulate_shoot(unit.id, unit.center(), direction,
                        current_unit.id, current_unit_position, get_mean_spread(weapon),
                        &weapon.typ, &weapon.params.bullet, &weapon.params.explosion,
                        world, world.config().optimal_location_number_of_directions, &mut None);
                    (shoot_result.player_score - shoot_result.opponent_score) as f64
                        / (world.max_score() * world.config().optimal_location_number_of_directions as i32) as f64 * fire_timer_factor
                } else {
                    0.0
                }
            } else {
                0.0
            }
        })
        .sum::<f64>();
    let opponent_obstacle_score = path_info.has_opponent_unit() as i32 as f64;
    let teammate_obstacle_score = path_info.has_teammate_unit() as i32 as f64;
    let mine_obstacle_score = path_info.has_mine() as i32 as f64;
    let bullet_obstacle_score = path_info.has_bullet() as i32 as f64;
    let loot_box_mine_score = (match world.tile_item(location) {
        Some(&Item::Mine { }) => true,
        _ => false,
    }) as i32 as f64;
    let nearest_opponent = if let Some(weapon) = current_unit.weapon.as_ref() {
        world.units().iter()
            .filter(|unit| {
                world.is_opponent_unit(unit)
                && may_shoot(current_unit.id, current_unit_center, &unit, weapon, world)
            })
            .min_by_key(|unit| as_score(current_unit_position.distance(unit.position())))
    } else {
        None
    };
    let hit_nearest_opponent_score = if let (Some(weapon), Some(unit)) = (current_unit.weapon.as_ref(), nearest_opponent.as_ref()) {
        let fire_timer_factor = get_fire_timer_factor(weapon.fire_timer, world.config().optimal_location_min_fire_timer);
        if fire_timer_factor > 0.0 {
            let direction = (unit.center() - current_unit_center).normalized();
            let shoot_result = simulate_shoot(current_unit.id, current_unit_center, direction,
                unit.id, unit.position(), get_mean_spread(weapon),
                &weapon.typ, &weapon.params.bullet, &weapon.params.explosion,
                world, world.config().optimal_location_number_of_directions, &mut None);
            (shoot_result.player_score - shoot_result.opponent_score) as f64
                / (world.max_score() * world.config().optimal_location_number_of_directions as i32) as f64 * fire_timer_factor
        } else {
            0.0
        }
    } else {
        0.0
    };
    let height_score = location.y() as f64 / world.size().y();
    let over_ground_score = (world.get_tile(location + Vec2i::new(0, -1)) != Tile::Empty) as i32 as f64;
    let number_of_bullets = world.bullets().iter()
        .filter(|v| v.unit_id != current_unit.id)
        .count();
    let bullets_score = if number_of_bullets > 0 {
        world.bullets().iter()
            .filter(|v| v.unit_id != current_unit.id && v.rect().has_collision(&location_rect))
            .count() as f64
    } else {
        0.0
    };
    let mines_score = if world.mines().len() > 0 {
        let mine_half = Vec2::both(world.properties().mine_explosion_params.radius + 1.0);
        world.mines().iter()
            .filter(|v| Rect::new(v.position(), mine_half).has_collision(&location_rect))
            .count() as f64
    } else {
        0.0
    };
    let hit_teammates_score = if let (Some(weapon), Some(opponent)) = (current_unit.weapon.as_ref(), nearest_opponent) {
        let fire_timer_factor = get_fire_timer_factor(weapon.fire_timer, world.config().optimal_location_min_fire_timer);
        if fire_timer_factor > 0.0 {
            let opponent_rect = opponent.rect();
            let max_damage = weapon.params.bullet.damage + weapon.params.explosion.as_ref().map(|v| v.damage).unwrap_or(0);
            world.units().iter()
                .filter(|v| {
                    world.is_opponent_unit(v)
                    && get_hit_probability_by_spread_with_destination(current_unit_center, opponent_rect.center(), &opponent_rect,
                        get_mean_spread(weapon), weapon.params.bullet.size) > 0.0
                })
                .map(|unit| {
                    let direction = (opponent.center() - current_unit_center).normalized();
                    simulate_shoot(current_unit.id, current_unit_center, direction,
                        unit.id, unit.position(), get_mean_spread(weapon),
                        &weapon.typ, &weapon.params.bullet, &weapon.params.explosion,
                        world, world.config().optimal_location_number_of_directions, &mut None)
                })
                .map(|shoot_result| {
                    (shoot_result.teammates_damage + shoot_result.unit_damage) as f64
                        / (max_damage * world.config().optimal_location_number_of_directions as i32) as f64
                })
                .sum::<f64>() * fire_timer_factor
        } else {
            0.0
        }
    } else {
        0.0
    };
    let number_of_opponents_mines = world.units().iter()
        .filter(|unit| world.is_opponent_unit(unit))
        .map(|unit| unit.mines)
        .sum::<i32>();
    let opponent_mine_explosion_score = if number_of_opponents_mines > 0 {
        world.units().iter()
            .filter(|unit| world.is_opponent_unit(unit) && unit.mines > 0 && unit.weapon.is_some())
            .map(|unit| {
                let fire_timer_factor = get_fire_timer_factor(unit.weapon.as_ref().unwrap().fire_timer, world.config().optimal_location_min_fire_timer);
                if fire_timer_factor == 0.0 {
                    return 0.0;
                }
                let mine_center = Vec2::new(unit.position.x, unit.position.y.floor())
                    + Vec2::only_y(world.properties().mine_size.y / 2.0);
                let explosion_radius = world.properties().mine_explosion_params.radius + 1.0;
                let explosion_rect = Rect::new(mine_center, Vec2::both(explosion_radius));
                (explosion_rect.has_collision(&location_rect) as i32 * unit.mines) as f64 * fire_timer_factor
            })
            .sum::<f64>() / number_of_opponents_mines as f64
    } else {
        0.0
    };

    [
        distance_to_position_score * world.config().optimal_location_distance_to_position_score_weight,
        health_pack_score * world.config().optimal_location_health_pack_score_weight,
        first_weapon_score * world.config().optimal_location_first_weapon_score_weight,
        swap_weapon_score * world.config().optimal_location_swap_weapon_score_weight,
        hit_by_opponent_score * world.config().optimal_location_hit_by_opponent_score_weight,
        opponent_obstacle_score * world.config().optimal_location_opponent_obstacle_score_weight,
        loot_box_mine_score * world.config().optimal_location_loot_box_mine_score_weight,
        mines_score * world.config().optimal_location_mines_score_weight,
        hit_nearest_opponent_score * world.config().optimal_location_hit_nearest_opponent_score_weight,
        height_score * world.config().optimal_location_height_score_weight,
        over_ground_score * world.config().optimal_location_over_ground_score_weight,
        bullets_score * world.config().optimal_location_bullets_score_weight,
        mine_obstacle_score * world.config().optimal_location_mine_obstacle_score_weight,
        hit_teammates_score * world.config().optimal_location_hit_teammates_score_weight,
        teammate_obstacle_score * world.config().optimal_location_teammate_obstacle_score_weight,
        bullet_obstacle_score * world.config().optimal_location_bullet_obstacle_score_weight,
        opponent_mine_explosion_score * world.config().optimal_location_opponent_mine_explosion_score_weight,
    ]
}

pub fn get_weapon_score(weapon_type: &WeaponType) -> u32 {
    match weapon_type {
        WeaponType::RocketLauncher => 1,
        WeaponType::AssaultRifle => 2,
        WeaponType::Pistol => 3,
    }
}

fn may_shoot(current_unit_id: i32, current_unit_center: Vec2, opponent: &Unit, weapon: &Weapon, world: &World) -> bool {
    is_allowed_to_shoot(current_unit_id, current_unit_center, get_mean_spread(weapon), opponent,
        weapon, world, world.config().optimal_location_number_of_directions, &mut None)
}

pub fn make_location_rect(location: Location) -> Rect {
    Rect::new(location.center(), Vec2::new(0.5, 0.5))
}

fn get_mean_spread(weapon: &Weapon) -> f64 {
    (weapon.params.max_spread + weapon.params.min_spread) / 2.0
}

fn get_fire_timer_factor(fire_timer: Option<f64>, optimal_location_min_fire_timer: f64) -> f64 {
    fire_timer.map(|v| 1.0 - (v / optimal_location_min_fire_timer).min(1.0)).unwrap_or(1.0)
}
