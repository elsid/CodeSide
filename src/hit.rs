use model::{
    Level,
    Tile,
};
use crate::my_strategy::{
    Location,
    Rect,
    Vec2,
    WalkGrid,
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
