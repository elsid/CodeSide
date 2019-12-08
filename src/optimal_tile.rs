use crate::Debug;

#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
    Vec2F32,
};

use model::Tile;

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
                world.config().optimal_tile_distance_to_opponent_weight.abs()
                + world.config().optimal_tile_distance_to_position_weight.abs()
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
    world.units().iter()
        .filter(|unit| unit.player_id != world.me().player_id)
        .map(|unit| {
            get_hit_probability(&me, &unit.rect(), world.level()) * center.distance(unit.position()) * world.config().optimal_tile_distance_to_opponent_weight
            + world.me().position().distance(center) * world.config().optimal_tile_distance_to_position_weight
        })
        .sum()
}
