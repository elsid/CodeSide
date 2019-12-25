mod helpers;

use model::{
    Tile,
};

use aicup2019::my_strategy::{
    Level,
    Rect,
    Vec2,
    get_hit_probability_by_spread,
    get_hit_probability_by_spread_with_destination,
    get_distance_to_nearest_hit_wall_by_horizontal,
    get_distance_to_nearest_hit_wall_by_line,
    get_distance_to_nearest_hit_wall_by_vertical,
};

#[test]
fn test_get_distance_to_nearest_hit_wall_by_vertical_with_only_empty_tiles() {
    let level = Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    });
    assert_eq!(
        get_distance_to_nearest_hit_wall_by_vertical(Vec2::new(0.5, 0.5), Vec2::new(0.5, 2.5), &level),
        None
    );
}

#[test]
fn test_get_distance_to_nearest_hit_wall_by_horizontal_with_only_empty_tiles() {
    let level = Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    });
    assert_eq!(
        get_distance_to_nearest_hit_wall_by_horizontal(Vec2::new(0.5, 0.5), Vec2::new(2.5, 0.5), &level),
        None
    );
}

#[test]
fn test_get_distance_to_nearest_hit_wall_by_line_with_only_empty_tiles() {
    let level = Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    });
    assert_eq!(
        get_distance_to_nearest_hit_wall_by_line(Vec2::new(0.5, 0.5), Vec2::new(2.5, 1.5), &level),
        None
    );
}

#[test]
fn test_get_distance_to_nearest_hit_wall_by_line_through_wall() {
    let level = Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    });
    assert_eq!(
        get_distance_to_nearest_hit_wall_by_line(Vec2::new(0.2312, 0.6423), Vec2::new(2.653, 1.234), &level),
        Some(1.0)
    );
}

#[test]
fn test_get_hit_probability_by_spread() {
    assert_eq!(
        get_hit_probability_by_spread(Vec2::new(0.5, 0.3), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.2880293914297168
    );
    assert_eq!(
        get_hit_probability_by_spread(Vec2::new(0.5, 0.3), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.05, 0.4),
        1.0
    );
    assert_eq!(
        get_hit_probability_by_spread(Vec2::new(19.5, 0.7), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.2880293914297171
    );
}

#[test]
fn test_get_hit_probability_by_spread_with_destination() {
    assert_eq!(
        get_hit_probability_by_spread_with_destination(Vec2::new(0.5, 0.3), Vec2::new(10.0, 0.5), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.2880293914297168
    );
    assert_eq!(
        get_hit_probability_by_spread_with_destination(Vec2::new(0.5, 0.3), Vec2::new(10.0, 0.5), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.05, 0.4),
        1.0
    );
    assert_eq!(
        get_hit_probability_by_spread_with_destination(Vec2::new(19.5, 0.7), Vec2::new(10.0, 0.5), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.2880293914297171
    );
    assert_eq!(
        get_hit_probability_by_spread_with_destination(Vec2::new(0.5, 0.3), Vec2::new(10.0, 4.0), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.13299230152296315
    );
}
