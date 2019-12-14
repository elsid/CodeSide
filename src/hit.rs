use model::{
    Level,
    Tile,
    Weapon,
};
use crate::my_strategy::{
    Config,
    Rect,
    Vec2,
    WalkGrid,
    as_score,
    get_tile_by_vec2,
};

pub fn should_shoot(shooter: &Rect, target: &Rect, weapon: &Weapon, level: &Level, config: &Config) -> bool {
    if let Some(explosion) = weapon.params.explosion.as_ref() {
        if let Some(distance_to_nearest_obstacle) = get_distance_to_nearest_hit_obstacle(shooter, target.center(), weapon.spread, level) {
            if distance_to_nearest_obstacle < explosion.radius + 1.0 {
                return false;
            }
        }
    }
    get_hit_probability_by_spread(shooter.center(), target, weapon.spread) >= config.min_hit_probability_by_spread_to_shoot
    && get_hit_probability_over_obstacles(shooter, target.center(), weapon.spread, level) >= config.min_hit_probability_over_obstacles_to_shoot
}

pub fn get_hit_probability_by_spread(shooter: Vec2, target: &Rect, spread: f64) -> f64 {
    target.get_max_cross_section_from(shooter, spread)
}

pub fn get_hit_probability_over_obstacles(shooter: &Rect, target: Vec2, spread: f64, level: &Level) -> f64 {
    const N: usize = 10;
    let begin = shooter.center();
    let to_target = target - begin;
    (0 .. N + 1)
        .map(|i| {
            let angle = ((2 * i) as f64 / N as f64 - 1.0) * spread;
            let end = begin + to_target.rotated(angle);
            will_hit_by_line(begin, end, level) as i32
        })
        .sum::<i32>() as f64 / N as f64
}

pub fn will_hit_by_line(begin: Vec2, end: Vec2, level: &Level) -> bool {
    for position in WalkGrid::new(begin, end) {
        if get_tile_by_vec2(level, position) == Tile::Wall {
            return false;
        }
    }
    true
}

pub fn get_distance_to_nearest_hit_obstacle(shooter: &Rect, target: Vec2, spread: f64, level: &Level) -> Option<f64> {
    const N: usize = 10;
    let begin = shooter.center();
    let to_target = target - begin;
    (0 .. N)
        .filter_map(|i| {
            let angle = ((2 * i) as f64 / N as f64 - 1.0) * spread;
            let end = to_target.rotated(angle);
            get_distance_to_nearest_hit_obstacle_by_line(begin, end, level)
        })
        .min_by_key(|&v| as_score(v))
}

pub fn get_distance_to_nearest_hit_obstacle_by_line(begin: Vec2, end: Vec2, level: &Level) -> Option<f64> {
    for position in WalkGrid::new(begin, end) {
        if get_tile_by_vec2(level, position) == Tile::Wall {
            return Some(begin.distance(position));
        }
    }
    None
}
