use model::{
    Level,
    Tile,
};
use crate::my_strategy::{
    Location,
    Rect,
    Vec2,
    WalkGrid,
    as_score,
    get_tile,
    get_tile_by_vec2,
};

pub fn get_hit_probability_by_spread(shooter: Vec2, target: &Rect, spread: f64) -> f64 {
    target.get_max_cross_section_from(shooter, spread)
}

pub fn get_hit_probability_over_obstacles(shooter: &Rect, target: &Rect, level: &Level) -> f64 {
    let begin = shooter.center();
    let end = target.center();
    if begin.x() as i32 == end.x() as i32 && begin.y() as i32 == end.y() as i32 {
        return (get_tile_by_vec2(level, begin) != Tile::Wall) as i32 as f64;
    }
    if begin.x() as i32 == end.x() as i32 {
        return will_hit_by_vertical(begin, end, level) as i32 as f64;
    }
    if begin.y() as i32 == end.y() as i32 {
        return will_hit_by_horizontal(begin, end, level) as i32 as f64;
    }
    let lower = target.center() - Vec2::new(0.0, target.half().y() / 2.0);
    let upper = target.center() + Vec2::new(0.0, target.half().y() / 2.0);
    (
        will_hit_by_line(begin, end, level) as i32
        + will_hit_by_line(begin, lower, level) as i32
        + will_hit_by_line(begin, upper, level) as i32
    ) as f64 / 3.0
}

pub fn will_hit_by_vertical(begin: Vec2, end: Vec2, level: &Level) -> bool {
    let x = begin.x() as isize;
    let mut y = begin.y() as isize;
    let end_y = end.y() as isize;
    let direction = (end_y - y).signum();
    while y != end_y {
        if get_tile(level, Location::new(x as usize, y as usize)) == Tile::Wall {
            return false;
        }
        y += direction;
    }
    get_tile(level, Location::new(x as usize, y as usize)) != Tile::Wall
}

pub fn will_hit_by_horizontal(begin: Vec2, end: Vec2, level: &Level) -> bool {
    let y = begin.y() as i32;
    let mut x = begin.x() as i32;
    let end_x = end.x() as i32;
    let direction = (end_x - x).signum();
    while x != end_x {
        if get_tile(level, Location::new(x as usize, y as usize)) == Tile::Wall {
            return false;
        }
        x += direction;
    }
    get_tile(level, Location::new(x as usize, y as usize)) != Tile::Wall
}

pub fn will_hit_by_line(begin: Vec2, end: Vec2, level: &Level) -> bool {
    for position in WalkGrid::new(begin, end) {
        if get_tile_by_vec2(level, position) == Tile::Wall {
            return false;
        }
    }
    true
}

pub fn get_hit_probability_by_spread_with_target(source: Vec2, target: Vec2, rect: &Rect, spread: f64, max_distance: f64) -> f64 {
    const N: usize = 10;
    let to_target = (target - source).normalized() * max_distance;
    (0 .. N + 1)
        .map(|i| {
            let angle = ((2 * i) as f64 / N as f64 - 1.0) * spread;
            let end = source + to_target.rotated(angle);
            rect.has_intersection_with_line(source, end) as i32
        })
        .sum::<i32>() as f64 / N as f64
}

pub fn get_distance_to_nearest_hit_obstacle(shooter: &Rect, target: Vec2, spread: f64, level: &Level) -> Option<f64> {
    const N: usize = 10;
    let begin = shooter.center();
    let to_target = target - begin;
    (0 .. N + 1)
        .filter_map(|i| {
            let angle = ((2 * i) as f64 / N as f64 - 1.0) * spread;
            let end = begin + to_target.rotated(angle);
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
