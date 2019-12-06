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

pub fn dump_level(level: &Level) -> String {
    let get_index = |x: usize, y: usize| -> usize { x + (level.tiles[0].len() - y - 1) * (level.tiles.len() + 1) };
    let mut buffer: Vec<u8> = std::iter::repeat('\n' as u8)
        .take((level.tiles.len() + 1) * level.tiles[0].len())
        .collect();
    for x in 0 .. level.tiles.len() {
        for y in 0 .. level.tiles[0].len() {
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
