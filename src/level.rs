use model::{
    Tile,
};
use crate::my_strategy::{
    Location,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level {
    size_x: usize,
    size_y: usize,
    tiles: Vec<Tile>,
}

impl Level {
    pub fn new(size_x: usize, size_y: usize, tiles: Vec<Tile>) -> Self {
        Self { size_x, size_y, tiles }
    }

    pub fn from_model(level: &model::Level) -> Self {
        let size_x = level.tiles.len();
        let size_y = level.tiles[0].len();
        let mut tiles = Vec::with_capacity(size_x * size_y);
        for x in 0 .. size_x {
            for y in 0 .. size_y {
                tiles.push(level.tiles[x][y].clone());
            }
        }
        Self { size_x, size_y, tiles }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.tiles.len()
    }

    #[inline(always)]
    pub fn size_x(&self) -> usize {
        self.size_x
    }

    #[inline(always)]
    pub fn size_y(&self) -> usize {
        self.size_y
    }

    #[inline(always)]
    pub fn get_tile_index(&self, location: Location) -> usize {
        location.y() + location.x() * self.size_y
    }

    #[inline(always)]
    pub fn get_tile_location(&self, index: usize) -> Location {
        Location::new(index / self.size_y, index % self.size_y)
    }

    #[inline(always)]
    pub fn get_tile(&self, location: Location) -> Tile {
        self.get_tile_by_index(self.get_tile_index(location)).clone()
    }

    #[inline(always)]
    pub fn get_tile_by_index(&self, index: usize) -> Tile {
        self.tiles[index].clone()
    }

    pub fn as_model(&self) -> model::Level {
        let mut tiles = std::iter::repeat(std::iter::repeat(Tile::Empty).take(self.size_y).collect::<Vec<_>>()).take(self.size_x).collect::<Vec<_>>();
        for x in 0 .. self.size_x {
            for y in 0 .. self.size_y {
                tiles[x][y] = self.get_tile(Location::new(x, y));
            }
        }
        model::Level { tiles }
    }
}

pub fn dump_level(level: &Level) -> String {
    let mut buffer: Vec<u8> = std::iter::repeat('\n' as u8)
        .take((level.size_x() + 1) * level.size_y())
        .collect();
    let get_index = |x: usize, y: usize| -> usize { x + (level.size_y() - y - 1) * (level.size_x() + 1) };
    for x in 0 .. level.size_x() {
        for y in 0 .. level.size_y() {
            buffer[get_index(x, y)] = match level.get_tile(Location::new(x, y)) {
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

pub fn parse_level(text: &str) -> Level {
    let size_x = text.find('\n').unwrap();
    let size_y = text.lines().count();
    let mut tiles = std::iter::repeat(Tile::Wall).take(size_x * size_y).collect::<Vec<_>>();
    let mut x = 0;
    let mut y = size_y;
    for line in text.lines() {
        y -= 1;
        for symbol in line.chars() {
            tiles[y + x * size_y] = match symbol {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'H' => Tile::Ladder,
                'T' => Tile::JumpPad,
                '^' => Tile::Platform,
                _ => Tile::Empty,
            };
            x += 1
        }
        x = 0;
    }
    Level::new(size_x, size_y, tiles)
}
