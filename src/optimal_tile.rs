use crate::Debug;

#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
    Vec2F32,
};

use model::{
    Item,
    Tile,
    WeaponType,
};

#[cfg(feature = "enable_debug")]
use crate::my_strategy::color_from_heat;

use crate::my_strategy::{
    Positionable,
    Rect,
    Rectangular,
    Vec2,
    World,
    get_hit_probability,
};

pub fn get_optimal_tile(world: &World, debug: &mut Debug) -> Option<(usize, usize)> {
    let mut optimal: Option<(f64, usize, usize)> = None;
    for x in 1 .. world.level().tiles.len() - 2 {
        for y in 1 .. world.level().tiles[0].len() - 2 {
            let tile = world.tile(x, y);
            if tile == Tile::Wall {
                continue;
            }
            let candidate_score = get_tile_score(world, x, y);
            if optimal.is_none() || optimal.unwrap().0 < candidate_score {
                optimal = Some((candidate_score, x, y));
            }
            #[cfg(feature = "enable_debug")]
            {
                let max = Vec2::new(40.0, 30.0).norm() * 2.0;
                debug.draw(CustomData::Rect {
                    pos: Vec2F32 { x: x as f32, y: y as f32 },
                    size: Vec2F32 { x: 1.0, y: 1.0 },
                    color: color_from_heat(0.33, (candidate_score / max) as f32 + 0.5),
                });
            }
        }
    }
    if let Some(v) = optimal {
        #[cfg(feature = "enable_debug")]
        {
            let max = Vec2::new(40.0, 30.0).norm() * (
                world.config().optimal_tile_distance_to_position_score_weight.abs()
                + world.config().optimal_tile_distance_to_opponent_score_weight.abs()
                + world.config().optimal_tile_health_pack_score_weight.abs()
                + world.config().optimal_tile_first_weapon_score_weight.abs()
                + world.config().optimal_tile_swap_weapon_score_weight.abs()
            );
            debug.draw(CustomData::Rect {
                pos: Vec2F32 { x: v.1 as f32, y: v.2 as f32 },
                size: Vec2F32 { x: 1.0, y: 1.0 },
                color: color_from_heat(0.66, (v.0 / max) as f32 + 0.5),
            });
        }
        Some((v.1, v.2))
    } else {
        None
    }
}

pub fn get_tile_score(world: &World, x: usize, y: usize) -> f64 {
    let center = Vec2::new(x as f64 + 0.5, y as f64 + world.me().size.y * 0.5);
    let me = Rect::new(center, Vec2::from_model(&world.me().size));
    let max_distance = world.size().norm();
    let hit_score = world.units().iter()
        .filter(|unit| unit.player_id != world.me().player_id)
        .map(|unit| {
            get_hit_probability(&me, &unit.rect(), world.level()) * center.distance(unit.position())
        })
        .sum::<f64>() / (world.units().len() as f64 * max_distance);
    let distance_to_position_score = world.me().position().distance(center) / max_distance;
    let health_pack_score = match world.tile_item(x, y) {
        Some(&Item::HealthPack { .. }) => 1.0,
        _ => 0.0,
    };
    let weapon_score = (world.me().weapon.is_none() && match world.tile_item(x, y) {
        Some(&Item::Weapon { .. }) => true,
        _ => false,
    }) as i32 as f64;
    let swap_weapon_score = (world.me().weapon.is_some() && match world.tile_item(x, y) {
        Some(&Item::Weapon { ref weapon_type }) => {
            get_weapon_score(&world.me().weapon.as_ref().unwrap().typ) < get_weapon_score(weapon_type)
        },
        _ => false,
    }) as i32 as f64;

    hit_score * world.config().optimal_tile_distance_to_opponent_score_weight
    + distance_to_position_score * world.config().optimal_tile_distance_to_position_score_weight
    + health_pack_score * world.config().optimal_tile_health_pack_score_weight
    + weapon_score * world.config().optimal_tile_first_weapon_score_weight
    + swap_weapon_score * world.config().optimal_tile_swap_weapon_score_weight
}

pub fn get_weapon_score(weapon_type: &WeaponType) -> u32 {
    match weapon_type {
        WeaponType::Pistol => 1,
        WeaponType::AssaultRifle => 3,
        WeaponType::RocketLauncher => 2,
    }
}
