use model::{
    Level,
    Tile,
};
use crate::my_strategy::vec2::Vec2;

pub fn get_tile_by_vec2(level: &Level, position: Vec2) -> Tile {
    get_tile_by_f64(level, position.x(), position.y())
}

pub fn get_tile_by_f64(level: &Level, x: f64, y: f64) -> Tile {
    get_tile(level, x as usize, y as usize)
}

pub fn get_tile(level: &Level, x: usize, y: usize) -> Tile {
    level.tiles[x][y].clone()
}
