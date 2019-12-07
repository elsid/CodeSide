use model::{
    JumpState,
    Level,
    Properties,
    Tile,
    Unit,
};
use my_strategy::examples::{
    example_properties,
};
use my_strategy::my_strategy::{
    Vec2,
    get_hit_probalibity,
    will_hit_by_horizontal,
    will_hit_by_line,
    will_hit_by_vertical,
};

#[test]
fn test_will_hit_by_vertical_with_only_empty_tiles() {
    let level = Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    };
    assert!(will_hit_by_vertical(Vec2::new(0.5, 0.5), Vec2::new(0.5, 2.5), &level));
}

#[test]
fn test_will_hit_by_horizontal_with_only_empty_tiles() {
    let level = Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    };
    assert!(will_hit_by_horizontal(Vec2::new(0.5, 0.5), Vec2::new(2.5, 0.5), &level));
}

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
fn test_get_hit_probalibity() {
    let properties = example_properties();
    let level = Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    };
    let shooter = make_unit_at(Vec2::new(0.2312, 0.6423), &properties);
    let target = make_unit_at(Vec2::new(2.653, 1.234), &properties);
    assert_eq!(get_hit_probalibity(&shooter, &target, &level), 0.0);
}

fn make_unit_at(position: Vec2, properties: &Properties) -> Unit {
    Unit {
        player_id: 3,
        id: 4,
        health: 100,
        position: position.as_model(),
        size: properties.unit_size.clone(),
        jump_state: JumpState {
            can_jump: false,
            speed: 0.0,
            max_time: 0.0,
            can_cancel: false,
        },
        walked_right: false,
        stand: true,
        on_ground: false,
        on_ladder: false,
        mines: 0,
        weapon: None,
    }
}