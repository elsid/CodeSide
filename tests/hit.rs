mod helpers;

use model::{
    Level,
    Tile,
};
use helpers::make_unit_rect;
use aicup2019::examples::{
    example_properties,
};
use aicup2019::my_strategy::{
    Vec2,
    get_hit_probability_over_obstacles,
    will_hit_by_line,
};

#[test]
fn test_will_hit_by_line_with_only_empty_tiles() {
    let level = Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    };
    assert!(will_hit_by_line(Vec2::new(0.5, 0.5), Vec2::new(2.5, 1.5), &level));
}

#[test]
fn test_will_hit_by_line_through_wall() {
    let level = Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    };
    assert!(!will_hit_by_line(Vec2::new(0.2312, 0.6423), Vec2::new(2.653, 1.234), &level));
}

#[test]
fn test_get_hit_probability_over_obstacles() {
    let properties = example_properties();
    let level = Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    };
    let shooter = make_unit_rect(Vec2::new(0.2312, 0.6423), &properties);
    let target = Vec2::new(2.653, 1.234);
    assert_eq!(get_hit_probability_over_obstacles(&shooter, target, 0.5, &level), 0.0);
}
