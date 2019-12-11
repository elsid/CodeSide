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
    as_score,
    color_from_heat,
    get_tile_index,
};

use crate::my_strategy::{
    Location,
    TilePathInfo,
    Positionable,
    Rect,
    Rectangular,
    Vec2,
    World,
    get_hit_probability_over_obstacles,
    get_level_size_x,
    get_level_size_y,
};

pub fn get_optimal_tile(world: &World, debug: &mut Debug) -> Option<Location> {
    let mut optimal: Option<(f64, Location)> = None;
    #[cfg(feature = "enable_debug")]
    let mut tiles: Vec<Option<f64>> = std::iter::repeat(None)
        .take(get_level_size_x(world.level()) * get_level_size_y(world.level()))
        .collect();
    for x in 1 .. get_level_size_x(world.level()) - 1 {
        for y in 1 .. get_level_size_y(world.level()) - 2 {
            let location = Location::new(x, y);
            let tile = world.tile(location);
            if tile == Tile::Wall {
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
                        color: color_from_heat(0.33, ((score - min) / (max - min)) as f32),
                    });
                }
            }
        }
    }
    if let Some((_, location)) = optimal {
        Some(location)
    } else {
        None
    }
}

pub fn get_tile_score(world: &World, location: Location, path_info: &TilePathInfo) -> f64 {
    let center = Vec2::new(location.x() as f64 + 0.5, location.y() as f64 + world.me().size.y * 0.5);
    let me = Rect::new(center, Vec2::from_model(&world.me().size));
    let max_distance = world.size().norm();
    let distance_to_opponent_score = world.units().iter()
        .filter(|unit| unit.player_id != world.me().player_id)
        .map(|unit| {
            get_hit_probability_over_obstacles(&me, &unit.rect(), world.level()) * center.distance(unit.position())
        })
        .sum::<f64>() / (world.units().len() as f64 * max_distance);
    let distance_to_position_score = path_info.distance() / max_distance;
    let health_pack_score = match world.tile_item(location) {
        Some(&Item::HealthPack { .. }) => 1.0,
        _ => 0.0,
    };
    let weapon_score = (world.me().weapon.is_none() && match world.tile_item(location) {
        Some(&Item::Weapon { .. }) => true,
        _ => false,
    }) as i32 as f64;
    let swap_weapon_score = (world.me().weapon.is_some() && match world.tile_item(location) {
        Some(&Item::Weapon { ref weapon_type }) => {
            get_weapon_score(&world.me().weapon.as_ref().unwrap().typ) < get_weapon_score(weapon_type)
        },
        _ => false,
    }) as i32 as f64;
    let hit_score = world.units().iter()
        .filter(|unit| unit.player_id != world.me().player_id)
        .map(|unit| {
            max_distance - get_hit_probability_over_obstacles(&unit.rect(), &me, world.level()) * center.distance(unit.position())
        })
        .sum::<f64>() / (world.units().len() as f64 * max_distance);
    let opponent_obstacle_score = path_info.has_opponent_unit() as i32 as f64;

    distance_to_opponent_score * world.config().optimal_tile_distance_to_opponent_score_weight
    + distance_to_position_score * world.config().optimal_tile_distance_to_position_score_weight
    + health_pack_score * world.config().optimal_tile_health_pack_score_weight
    + weapon_score * world.config().optimal_tile_first_weapon_score_weight
    + swap_weapon_score * world.config().optimal_tile_swap_weapon_score_weight
    + hit_score * world.config().optimal_tile_hit_score_weight
    + opponent_obstacle_score * world.config().optimal_tile_opponent_obstacle_score_weight
}

pub fn get_weapon_score(weapon_type: &WeaponType) -> u32 {
    match weapon_type {
        WeaponType::Pistol => 1,
        WeaponType::AssaultRifle => 3,
        WeaponType::RocketLauncher => 2,
    }
}
