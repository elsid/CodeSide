use model::Tile;

use aicup2019::my_strategy::{
    Level,
    Map,
    Polygon,
    Vec2,
};

#[test]
fn test_map_from_level_square_bounded_by_walls() {
    let map = Map::from_level(&Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
        ]
    }));

    assert_eq!(map, Map::new(
        Polygon::from_vertices(vec![
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(2.0, 1.0),
        ]),
        Vec::new()
    ));
}

#[test]
fn test_map_from_level_rect_bounded_by_walls() {
    let map = Map::from_level(&Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
        ]
    }));

    assert_eq!(map, Map::new(
        Polygon::from_vertices(vec![
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(3.0, 2.0),
            Vec2::new(3.0, 1.0),
        ]),
        Vec::new()
    ));
}

#[test]
fn test_map_from_level_L_bounded_by_walls() {
    let map = Map::from_level(&Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        ]
    }));

    assert_eq!(map, Map::new(
        Polygon::from_vertices(vec![
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(2.0, 3.0),
            Vec2::new(3.0, 3.0),
            Vec2::new(3.0, 1.0),
        ]),
        Vec::new()
    ));
}
