#[cfg(feature = "enable_debug")]
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

#[cfg(feature = "enable_debug")]
use crate::my_strategy::{
    color_from_heat,
    get_tile_index,
};

use crate::my_strategy::{
    Debug,
    Location,
    Positionable,
    Rect,
    Rectangular,
    Target,
    TilePathInfo,
    Vec2,
    Vec2i,
    World,
    as_score,
    get_hit_probabilities,
    get_hit_probability_by_spread,
    get_hit_probability_over_obstacles,
    get_level_size_x,
    get_level_size_y,
};

#[inline(never)]
pub fn get_optimal_location(unit: &Unit, optimal_locations: &Vec<(i32, Option<Location>)>, world: &World, debug: &mut Debug) -> Option<(f64, Location)> {
    let mut optimal: Option<(f64, Location)> = None;

    #[cfg(feature = "enable_debug")]
    let mut tiles: Vec<Option<f64>> = std::iter::repeat(None)
        .take(get_level_size_x(world.level()) * get_level_size_y(world.level()))
        .collect();

    let unit_index = world.get_unit_index(unit.id);

    for x in 1 .. get_level_size_x(world.level()) - 1 {
        for y in 1 .. get_level_size_y(world.level()) - 2 {
            let location = Location::new(x, y);
            let tile = world.tile(location);
            if tile == Tile::Wall || is_busy_by_other(location, unit.id, optimal_locations, world) {
                continue;
            }
            if let Some(path_info) = world.get_path_info(unit_index, unit.location(), location) {
                let candidate_score = get_location_score(location, unit, world, path_info);
                if optimal.is_none() || optimal.unwrap().0 < candidate_score {
                    optimal = Some((candidate_score, location));
                }
                #[cfg(feature = "enable_debug")]
                {
                    tiles[get_tile_index(world.level(), location)] = Some(candidate_score);
                }
            }
        }
    }

    #[cfg(feature = "enable_debug")]
    {
        let min = tiles.iter().filter_map(|&v| v).min_by_key(|&v| as_score(v)).unwrap();
        let max = tiles.iter().filter_map(|&v| v).max_by_key(|&v| as_score(v)).unwrap();
        for x in 1 .. get_level_size_x(world.level()) - 1 {
            for y in 1 .. get_level_size_y(world.level()) - 2 {
                let location = Location::new(x, y);
                if let Some(score) = tiles[get_tile_index(world.level(), location)] {
                    debug.draw(CustomData::Rect {
                        pos: location.as_model_f32(),
                        size: Vec2F32 { x: 1.0, y: 1.0 },
                        color: color_from_heat(0.1, ((score - min) / (max - min)) as f32),
                    });
                }
            }
        }
        if let Some((score, location)) = optimal {
            let path_info = world.get_path_info(unit_index, unit.location(), location).unwrap();
            debug.log(format!("[{}] optimal_tile: {:?} {:?} {:?}", unit.id, location, score, get_location_score_components(location, unit, world, path_info)));
        }
    }

    optimal
}

fn is_busy_by_other(location: Location, unit_id: i32, optimal_tiles: &Vec<(i32, Option<Location>)>, world: &World) -> bool {
    for i in 0 .. optimal_tiles.len() {
        if optimal_tiles[i].0 == unit_id {
            continue;
        }
        if let Some(v) = optimal_tiles[i].1 {
            if v == location {
                return true;
            }
        }
    }
    world.units().iter()
        .filter(|v| v.id != unit_id)
        .find(|v| {
            for x in -1 .. 2 {
                for y in -1 .. 3 {
                    if v.location() == location + Vec2i::new(x, y) {
                        return true;
                    }
                }
            }
            false
        })
        .is_some()
}

pub fn get_location_score(location: Location, current_unit: &Unit, world: &World, path_info: &TilePathInfo) -> f64 {
    get_location_score_components(location, current_unit, world, path_info).iter().sum()
}

pub fn get_location_score_components(location: Location, current_unit: &Unit, world: &World, path_info: &TilePathInfo) -> [f64; 16] {
    let current_unit_position = Vec2::new(location.x() as f64 + 0.5, location.y() as f64);
    let current_unit_center = Vec2::new(location.x() as f64 + 0.5, location.y() as f64 + current_unit.size.y * 0.5);
    let current_unit_rect = Rect::new(current_unit_center, Vec2::from_model(&current_unit.size) / 2.0);
    let max_distance = world.size().norm();
    let tile_rect = Rect::new(
        Vec2::new(location.x() as f64 + 0.5, location.y() as f64 + 0.5),
        Vec2::new(0.5, 0.5)
    );
    let distance_to_opponent_score = world.units().iter()
        .filter(|unit| world.is_opponent_unit(unit))
        .map(|unit| {
            get_hit_probability_over_obstacles(&current_unit_rect, &unit.rect(), world.level()) * current_unit_center.distance(unit.center())
        })
        .sum::<f64>() / max_distance;
    let distance_to_position_score = path_info.distance() / max_distance;
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
    let target = Target::new(current_unit.id, current_unit_rect.clone());
    let hit_by_opponent_score = world.units().iter()
        .filter(|unit| world.is_opponent_unit(unit))
        .map(|unit| {
            if let Some(weapon) = unit.weapon.as_ref() {
                if weapon.fire_timer.is_none() || weapon.fire_timer.unwrap() < world.config().optimal_tile_min_fire_timer {
                    let direction = (current_unit_center - unit.center()).normalized();
                    let hit_probabilities = get_hit_probabilities(unit.id, unit.center(), direction, &target, weapon.spread, weapon.params.bullet.size, world);
                    (hit_probabilities.target + hit_probabilities.teammate_units) as f64 / hit_probabilities.total as f64
                } else {
                    0.0
                }
            } else {
                0.0
            }
        })
        .sum::<f64>();
    let opponent_obstacle_score = path_info.has_opponent_unit() as i32 as f64;
    let mine_obstacle_score = path_info.has_mine() as i32 as f64;
    let loot_box_mine_score = (match world.tile_item(location) {
        Some(&Item::Mine { }) => true,
        _ => false,
    }) as i32 as f64;
    let nearest_opponent = if let Some(weapon) = current_unit.weapon.as_ref() {
        world.units().iter()
            .filter(|unit| world.is_opponent_unit(unit) && should_shoot(current_unit.id, current_unit_center, &unit, weapon, world, false))
            .min_by_key(|unit| as_score(current_unit_position.distance(unit.position())))
    } else {
        None
    };
    let hit_nearest_opponent_score = if let (Some(weapon), Some(unit)) = (current_unit.weapon.as_ref(), nearest_opponent.as_ref()) {
        let by_spread = get_hit_probability_by_spread(current_unit_center, &unit.rect(), weapon.params.min_spread, weapon.params.bullet.size);
        if by_spread == 0.0 {
            0.0
        } else {
            if weapon.fire_timer.is_none() || weapon.fire_timer.unwrap() < world.config().optimal_tile_min_fire_timer {
                let direction = (unit.center() - current_unit_center).normalized();
                let hit_probabilities = get_hit_probabilities(current_unit.id, current_unit_center, direction, &Target::from_unit(unit), weapon.params.min_spread, weapon.params.bullet.size, world);
                by_spread * hit_probabilities.target as f64 / hit_probabilities.total as f64
            } else {
                0.0
            }
        }
    } else {
        0.0
    };
    let height_score = location.y() as f64 / world.size().y();
    let over_ground_score = (world.tile(location + Vec2i::new(0, -1)) != Tile::Empty) as i32 as f64;
    let number_of_bullets = world.bullets().iter()
        .filter(|v| v.unit_id != current_unit.id)
        .count();
    let bullets_score = if number_of_bullets > 0 {
        world.bullets().iter()
            .filter(|v| v.unit_id != current_unit.id && v.rect().has_collision(&tile_rect))
            .count() as f64
    } else {
        0.0
    };
    let mines_score = if world.mines().len() > 0 {
        world.mines().iter()
            .filter(|v| v.rect().center().distance(tile_rect.center()) <= 2.0 * world.properties().mine_trigger_radius)
            .count() as f64
    } else {
        0.0
    };
    let hit_teammates_score = if let Some(weapon) = current_unit.weapon.as_ref() {
        if weapon.fire_timer.is_none() || weapon.fire_timer.unwrap() < world.config().optimal_tile_min_fire_timer {
            world.units().iter()
                .filter(|v| world.is_opponent_unit(v))
                .map(|v| {
                    let direction = (v.center() - current_unit_center).normalized();
                    get_hit_probabilities(current_unit.id, current_unit_center, direction, &Target::from_unit(v), weapon.spread, weapon.params.bullet.size, world)
                })
                .map(|v| v.teammate_units as f64 / v.total as f64)
                .sum::<f64>()
        } else {
            0.0
        }
    } else {
        0.0
    };
    let direct_hit_by_opponent_score = world.units().iter()
        .filter(|unit| world.is_opponent_unit(unit))
        .map(|unit| {
            if let Some(weapon) = unit.weapon.as_ref() {
                if let Some(angle) = weapon.last_angle {
                    if weapon.fire_timer.is_none() || weapon.fire_timer.unwrap() < world.config().optimal_tile_min_fire_timer {
                        let hit_probabilities = get_hit_probabilities(unit.id, unit.rect().center(), Vec2::i().rotated(angle), &target, weapon.spread, weapon.params.bullet.size, world);
                        (hit_probabilities.target + hit_probabilities.teammate_units) as f64 / hit_probabilities.total as f64
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            } else {
                0.0
            }
        })
        .sum::<f64>();

    [
        distance_to_opponent_score * world.config().optimal_tile_distance_to_opponent_score_weight,
        distance_to_position_score * world.config().optimal_tile_distance_to_position_score_weight,
        health_pack_score * world.config().optimal_tile_health_pack_score_weight,
        first_weapon_score * world.config().optimal_tile_first_weapon_score_weight,
        swap_weapon_score * world.config().optimal_tile_swap_weapon_score_weight,
        hit_by_opponent_score * world.config().optimal_tile_hit_by_opponent_score_weight,
        opponent_obstacle_score * world.config().optimal_tile_opponent_obstacle_score_weight,
        loot_box_mine_score * world.config().optimal_tile_loot_box_mine_score_weight,
        mines_score * world.config().optimal_tile_mines_score_weight,
        hit_nearest_opponent_score * world.config().optimal_tile_hit_nearest_opponent_score_weight,
        height_score * world.config().optimal_tile_height_score_weight,
        over_ground_score * world.config().optimal_tile_over_ground_score_weight,
        bullets_score * world.config().optimal_tile_bullets_score_weight,
        mine_obstacle_score * world.config().optimal_tile_mine_obstacle_score_weight,
        hit_teammates_score * world.config().optimal_tile_hit_teammates_score_weight,
        direct_hit_by_opponent_score * world.config().optimal_tile_direct_hit_by_opponent_score_weight,
    ]
}

pub fn get_weapon_score(weapon_type: &WeaponType) -> u32 {
    match weapon_type {
        WeaponType::RocketLauncher => 1,
        WeaponType::AssaultRifle => 2,
        WeaponType::Pistol => 3,
    }
}

pub fn should_shoot(current_unit_id: i32, current_unit_center: Vec2, opponent: &Unit, weapon: &Weapon, world: &World, use_current_spread: bool) -> bool {
    let spread = if use_current_spread {
        weapon.spread
    } else {
        weapon.params.min_spread
    };

    let hit_probability_by_spread = get_hit_probability_by_spread(current_unit_center, &opponent.rect(), spread, weapon.params.bullet.size);

    if hit_probability_by_spread < world.config().min_hit_probability_by_spread_to_shoot {
        return false;
    }

    let direction = (opponent.center() - current_unit_center).normalized();
    let hit_probabilities = get_hit_probabilities(current_unit_id, current_unit_center, direction, &Target::from_unit(opponent), spread, weapon.params.bullet.size, world);

    if let (Some(explosion), Some(min_distance)) = (weapon.params.explosion.as_ref(), hit_probabilities.min_distance) {
        if min_distance < explosion.radius + 2.0 {
            return false;
        }
    }

    hit_probabilities.target.max(hit_probabilities.opponent_units) >= world.config().min_target_hits_to_shoot
    && hit_probabilities.teammate_units <= world.config().max_teammates_hits_to_shoot
}
