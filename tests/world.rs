mod helpers;

use helpers::updated_world;

use aicup2019::{
    examples::{
        EXAMPLE_MY_UNIT_ID,
        example_world,
    },
    my_strategy::{
        Location,
    }
};

#[test]
fn test_find_tiles_path() {
    let world = updated_world(example_world());
    assert_eq!(
        world.find_reversed_tiles_path(EXAMPLE_MY_UNIT_ID, Location::new(37, 1), Location::new(29, 5)),
        vec![
            Location::new(29, 5), Location::new(30, 5), Location::new(31, 5), Location::new(32, 4),
            Location::new(33, 3), Location::new(34, 2), Location::new(35, 1), Location::new(36, 1),
        ]
    );
}

#[test]
fn test_find_shortcut_tiles_path() {
    let world = updated_world(example_world());
    assert_eq!(
        world.find_shortcut_tiles_path(EXAMPLE_MY_UNIT_ID, Location::new(37, 1), Location::new(29, 5)),
        vec![Location::new(31, 5), Location::new(29, 5)]
    );
    assert_eq!(
        world.find_shortcut_tiles_path(EXAMPLE_MY_UNIT_ID, Location::new(37, 1), Location::new(37, 5)),
        vec![Location::new(37, 5)]
    );
    assert_eq!(
        world.find_shortcut_tiles_path(EXAMPLE_MY_UNIT_ID, Location::new(37, 1), Location::new(37, 2)),
        vec![Location::new(37, 2)]
    );
    assert_eq!(
        world.find_shortcut_tiles_path(EXAMPLE_MY_UNIT_ID, Location::new(37, 1), Location::new(37, 1)),
        vec![]
    );
}
