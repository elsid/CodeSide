use model::Tile;

use crate::my_strategy::{
    Level,
    Location,
    Polygon,
    Rect,
    Vec2,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    border: Polygon,
    obstacles: Vec<Polygon>,
}

impl Map {
    pub fn new(border: Polygon, obstacles: Vec<Polygon>) -> Self {
        Self { border, obstacles }
    }

    pub fn from_level(level: &Level) -> Self {
        let border = make_border_polygon(level);
        let obstacles = Vec::new();
        Self { border, obstacles }
    }
}

fn make_border_polygon(level: &Level) -> Polygon {
    let (min_x, max_x, min_y, max_y) = make_bounding_rect(level);
    println!("{:?}", (min_x, max_x, min_y, max_y));
    Polygon::from_rect(&Rect::from_min_max(
        Vec2::new(min_x as f64, min_y as f64),
        Vec2::new(max_x as f64, max_y as f64)
    ))
}

fn make_bounding_rect(level: &Level) -> (usize, usize, usize, usize) {
    let mut min_x = 0;
    let mut max_x = level.size_x() - 1;
    let mut min_y = 0;
    let mut max_y = level.size_y() - 1;
    while min_x < max_x && (min_y .. max_y + 1).find(|&y| level.get_tile(Location::new(min_x, y)) == Tile::Wall).is_some() {
        println!("inc min_x {}", min_x);
        min_x += 1;
    }
    while min_y < max_y && (min_x .. max_x + 1).find(|&x| level.get_tile(Location::new(x, min_y)) == Tile::Wall).is_some() {
        println!("inc min_y {}", min_y);
        min_y += 1;
    }
    while max_x > min_x && (min_y .. max_y + 1).find(|&y| level.get_tile(Location::new(max_x, y)) == Tile::Wall).is_some() {
        println!("inc max_x {}", max_x);
        max_x -= 1;
    }
    while max_y > min_y && (min_x .. max_x + 1).find(|&x| level.get_tile(Location::new(x, max_y)) == Tile::Wall).is_some() {
        println!("inc max_y {}", max_y);
        max_y -= 1;
    }
    (min_x, max_x, min_y, max_y)
}
