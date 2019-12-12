use model::{
    Level,
    Tile,
};
use crate::my_strategy::{
    Location,
    Vec2,
};

#[inline(always)]
pub fn get_tile_by_vec2(level: &Level, position: Vec2) -> Tile {
    get_tile(level, position.as_location())
}

#[inline(always)]
pub fn get_tile(level: &Level, location: Location) -> Tile {
    level.tiles[location.x()][location.y()].clone()
}

#[inline(always)]
pub fn get_tile_index(level: &Level, location: Location) -> usize {
    location.y() + location.x() * get_level_size_y(level)
}

#[inline(always)]
pub fn get_tile_location(level: &Level, index: usize) -> Location {
    Location::new(index / get_level_size_y(level), index % get_level_size_y(level))
}

#[inline(always)]
pub fn get_level_size_x(level: &Level) -> usize {
    level.tiles.len()
}

#[inline(always)]
pub fn get_level_size_y(level: &Level) -> usize {
    level.tiles[0].len()
}

pub fn dump_level(level: &Level) -> String {
    let mut buffer: Vec<u8> = std::iter::repeat('\n' as u8)
        .take((get_level_size_x(level) + 1) * get_level_size_y(level))
        .collect();
    let get_index = |x: usize, y: usize| -> usize { x + (get_level_size_y(level) - y - 1) * (get_level_size_x(level) + 1) };
    for x in 0 .. get_level_size_x(level) {
        for y in 0 .. get_level_size_y(level) {
            buffer[get_index(x, y)] = match level.tiles[x][y] {
                Tile::Empty => '.' as u8,
                Tile::Wall => '#' as u8,
                Tile::Platform => '^' as u8,
                Tile::Ladder => 'H' as u8,
                Tile::JumpPad => 'T' as u8,
            };
        }
    }
    String::from_utf8(buffer).unwrap()
}
