use crate::Debug;

#[cfg(feature = "enable_debug")]
use model::{
    CustomData,
    Vec2F32,
};

use model::{
    Item,
    Tile,
    WeaponType,
};

#[cfg(feature = "enable_debug")]
use crate::my_strategy::{
    color_from_heat,
    get_tile_index,
};

use crate::my_strategy::{
    Location,
    Positionable,
    Rect,
    Rectangular,
    TilePathInfo,
    Vec2,
    Vec2i,
    World,
    as_score,
    get_hit_probability_by_spread,
    get_hit_probability_by_spread_with_target,
    get_hit_probability_over_obstacles,
    get_level_size_x,
    get_level_size_y,
};

pub fn get_optimal_tile(world: &World, optimal_tiles: &Vec<Option<(f64, Location)>>, debug: &mut Debug) -> Option<(f64, Location)> {
    let mut optimal: Option<(f64, Location)> = None;
    #[cfg(feature = "enable_debug")]
    let mut tiles: Vec<Option<f64>> = std::iter::repeat(None)
        .take(get_level_size_x(world.level()) * get_level_size_y(world.level()))
        .collect();
    for x in 1 .. get_level_size_x(world.level()) - 1 {
        for y in 1 .. get_level_size_y(world.level()) - 2 {
            let location = Location::new(x, y);
            let tile = world.tile(location);
            if tile == Tile::Wall || is_busy_by_other(location, world.me_index(), optimal_tiles) {
                continue;
            }
            if let Some(path_info) = world.path_info(world.me().location(), location) {
                let candidate_score = get_tile_score(world, location, path_info);
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
                        color: color_from_heat(0.25, ((score - min) / (max - min)) as f32),
                    });
                }
            }
        }
        if let Some((score, location)) = optimal {
            debug.draw(CustomData::Log {
                text: format!("optimal_tile: {:?} {:?} {:?}", location, score, get_tile_score_components(world, location, world.path_info(world.me().location(), location).unwrap())),
            });
        }
    }
    optimal
}

pub fn is_busy_by_other(location: Location, me_index: usize, optimal_tiles: &Vec<Option<(f64, Location)>>) -> bool {
    for i in 0 .. optimal_tiles.len() {
        if i == me_index {
            continue;
        }
        if let Some((_, v)) = optimal_tiles[i].as_ref() {
            if *v == location {
                return true;
            }
        }
    }
    false
}

pub fn get_tile_score(world: &World, location: Location, path_info: &TilePathInfo) -> f64 {
    get_tile_score_components(world, location, path_info).iter().sum()
}

pub fn get_tile_score_components(world: &World, location: Location, path_info: &TilePathInfo) -> [f64; 16] {
    let position = Vec2::new(location.x() as f64 + 0.5, location.y() as f64);
    let center = Vec2::new(location.x() as f64 + 0.5, location.y() as f64 + world.me().size.y * 0.5);
    let me = Rect::new(center, Vec2::from_model(&world.me().size));
    let max_distance = world.size().norm();
    let tile_rect = Rect::new(
        Vec2::new(location.x() as f64 + 0.5, location.y() as f64 + 0.5),
        Vec2::new(0.5, 0.5)
    );
    let distance_to_opponent_score = world.units().iter()
        .filter(|unit| unit.player_id != world.me().player_id)
        .map(|unit| {
            get_hit_probability_over_obstacles(&me, &unit.rect(), world.level()) * center.distance(unit.position())
        })
        .sum::<f64>() / (world.units().len() as f64 * max_distance);
    let distance_to_position_score = path_info.distance() / max_distance;
    let health_pack_score = match world.tile_item(location) {
        Some(&Item::HealthPack { .. }) => 2.0 - world.me().health as f64 / world.properties().unit_max_health as f64,
        _ => 0.0,
    };
    let first_weapon_score = (world.me().weapon.is_none() && match world.tile_item(location) {
        Some(&Item::Weapon { .. }) => true,
        _ => false,
    }) as i32 as f64;
    let swap_weapon_score = (world.me().weapon.is_some() && match world.tile_item(location) {
        Some(&Item::Weapon { ref weapon_type }) => {
            get_weapon_score(&world.me().weapon.as_ref().unwrap().typ) < get_weapon_score(weapon_type)
        },
        _ => false,
    }) as i32 as f64;
    let number_of_opponents = world.units().iter()
        .filter(|unit| unit.player_id != world.me().player_id)
        .count();
    let hit_by_opponent_score = world.units().iter()
        .filter(|unit| unit.player_id != world.me().player_id)
        .map(|unit| {
            if let Some(weapon) = unit.weapon.as_ref() {
                get_hit_probability_over_obstacles(&unit.rect(), &me, world.level())
                    * get_hit_probability_by_spread(unit.rect().center(), &me, weapon.spread)
            } else {
                0.0
            }
        })
        .sum::<f64>() / (number_of_opponents as f64);
    let opponent_obstacle_score = path_info.has_opponent_unit() as i32 as f64;
    let mine_obstacle_score = path_info.has_mine() as i32 as f64;
    let loot_box_mine_score = (match world.tile_item(location) {
        Some(&Item::Mine { }) => true,
        _ => false,
    }) as i32 as f64;
    let nearest_opponent = if let Some(weapon) = world.me().weapon.as_ref() {
        world.units().iter()
            .filter(|unit| world.is_opponent(unit) && should_shoot(&me, &unit.rect(), weapon.spread, world))
            .min_by_key(|unit| as_score(position.distance(unit.position())))
    } else {
        None
    };
    let hit_nearest_opponent_score = if let (Some(weapon), Some(unit)) = (world.me().weapon.as_ref(), nearest_opponent.as_ref()) {
        get_hit_probability_by_spread(center, &unit.rect(), weapon.spread)
            * get_hit_probability_over_obstacles(&me, &unit.rect(), world.level())
    } else {
        0.0
    };
    let height_score = location.y() as f64 / world.size().y();
    let over_ground_score = (world.tile(location + Vec2i::new(0, -1)) != Tile::Empty) as i32 as f64;
    let number_of_bullets = world.bullets().iter()
        .filter(|v| v.unit_id != world.me().id)
        .count();
    let bullets_score = if number_of_bullets > 0 {
        world.bullets().iter()
            .filter(|v| v.unit_id != world.me().id && v.rect().has_collision(&tile_rect))
            .count() as f64 / (number_of_bullets as f64)
    } else {
        0.0
    };
    let mines_score = if world.mines().len() > 0 {
        world.mines().iter()
            .filter(|v| v.rect().has_collision(&tile_rect))
            .count() as f64 / (world.mines().len() as f64)
    } else {
        0.0
    };
    let hit_teammates_score = if let (true, Some(weapon)) = (number_of_opponents > 0, world.me().weapon.as_ref()) {
        world.units().iter()
            .filter(|v| world.is_opponent(v))
            .map(|v| get_hit_teammates_probability(&me, v.rect().center(), weapon.spread, world))
            .sum::<f64>() / number_of_opponents as f64
    } else {
        0.0
    };
    let distance_to_teammates_score = if world.number_of_teammates() > 1 && world.me().weapon.is_some() {
        world.units().iter()
            .filter(|v| world.is_teammate(v) && v.weapon.is_none())
            .map(|v| v.rect().center().distance(center))
            .sum::<f64>() / (max_distance * (world.number_of_teammates() - 1) as f64)
    } else {
        0.0
    };

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
        distance_to_teammates_score * world.config().optimal_tile_distance_to_teammates_score_weight,
    ]
}

pub fn get_weapon_score(weapon_type: &WeaponType) -> u32 {
    match weapon_type {
        WeaponType::RocketLauncher => 1,
        WeaponType::AssaultRifle => 2,
        WeaponType::Pistol => 3,
    }
}

pub fn get_hit_teammates_probability(me: &Rect, target: Vec2, spread: f64, world: &World) -> f64 {
    world.units().iter()
        .filter(|unit| world.is_teammate(unit))
        .map(|unit| {
            get_hit_probability_over_obstacles(me, &unit.rect(), world.level())
                * get_hit_probability_by_spread_with_target(me.center(), target, &unit.rect(), spread, world.max_distance())
        })
        .sum::<f64>() / world.number_of_teammates() as f64
}

pub fn should_shoot(me: &Rect, opponent: &Rect, spread: f64, world: &World) -> bool {
    let hit_probability_by_spread = get_hit_probability_by_spread(me.center(), opponent, spread);
    let hit_probability_over_obstacles = get_hit_probability_over_obstacles(me, opponent, world.level());
    let hit_teammates_probability = get_hit_teammates_probability(me, opponent.center(), spread, &world);
    hit_probability_by_spread >= world.config().min_hit_probability_by_spread_to_shoot
    && hit_probability_over_obstacles >= world.config().min_hit_probability_over_obstacles_to_shoot
    && hit_teammates_probability <= world.config().max_hit_teammates_probability_to_shoot
}
