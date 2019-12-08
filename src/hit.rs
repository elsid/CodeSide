use model::{
    Level,
    Tile,
    Unit,
};
use crate::my_strategy::{
    Rectangular,
    Vec2,
    WalkGrid,
    get_tile,
};

pub fn get_hit_probability(shooter: &Unit, target: &Unit, level: &Level) -> f64 {
    let begin = shooter.rect().center();
    let end = target.rect().center();
    if begin.x() as i32 == end.x() as i32 && begin.y() as i32 == end.y() as i32 {
        return (get_tile(level, begin.x() as usize, begin.y() as usize) != Tile::Wall) as i32 as f64;
    }
    if begin.x() as i32 == end.x() as i32 {
        return will_hit_by_vertical(begin, end, level) as i32 as f64;
    }
    if begin.y() as i32 == end.y() as i32 {
        return will_hit_by_horizontal(begin, end, level) as i32 as f64;
    }
    will_hit_by_line(begin, end, level) as i32 as f64
}

pub fn will_hit_by_vertical(begin: Vec2, end: Vec2, level: &Level) -> bool {
    let x = begin.x() as i32;
    let mut y = begin.y() as i32;
    let end_y = end.y() as i32;
    let direction = (end_y - y).signum();
    while y != end_y {
        if get_tile(level, x as usize, y as usize) == Tile::Wall {
            return false;
        }
        y += direction;
    }
    get_tile(level, x as usize, y as usize) != Tile::Wall
}

pub fn will_hit_by_horizontal(begin: Vec2, end: Vec2, level: &Level) -> bool {
    let y = begin.y() as i32;
    let mut x = begin.x() as i32;
    let end_x = end.x() as i32;
    let direction = (end_x - x).signum();
    while x != end_x {
        if get_tile(level, x as usize, y as usize) == Tile::Wall {
            return false;
        }
        x += direction;
    }
    get_tile(level, x as usize, y as usize) != Tile::Wall
}

pub fn will_hit_by_line(begin: Vec2, end: Vec2, level: &Level) -> bool {
    for position in WalkGrid::new(begin, end) {
        if get_tile(level, position.x() as usize, position.y() as usize) == Tile::Wall {
            return false;
        }
    }
    true
}