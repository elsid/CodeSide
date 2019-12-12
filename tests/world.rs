use aicup2019::{
    examples::example_world,
    my_strategy::{
        Location,
        World,
    }
};

#[test]
fn test_find_tiles_path() {
    let world = updated_world(example_world());
    assert_eq!(
        world.find_reversed_tiles_path(Location::new(37, 1), Location::new(29, 5)),
        vec![
            Location::new(29, 5), Location::new(30, 5), Location::new(31, 5), Location::new(32, 5),
            Location::new(33, 5), Location::new(34, 5), Location::new(35, 5), Location::new(36, 5),
            Location::new(37, 5), Location::new(37, 4), Location::new(37, 3), Location::new(37, 2),
        ]
    );
}

#[test]
fn test_find_shortcut_tiles_path() {
    let world = updated_world(example_world());
    assert_eq!(
        world.find_shortcut_tiles_path(Location::new(37, 1), Location::new(29, 5)),
        vec![Location::new(31, 5), Location::new(29, 5)]
    );
    assert_eq!(
        world.find_shortcut_tiles_path(Location::new(37, 1), Location::new(37, 5)),
        vec![Location::new(37, 5)]
    );
    assert_eq!(
        world.find_shortcut_tiles_path(Location::new(37, 1), Location::new(37, 2)),
        vec![Location::new(37, 2)]
    );
    assert_eq!(
        world.find_shortcut_tiles_path(Location::new(37, 1), Location::new(37, 1)),
        vec![]
    );
}

fn updated_world(mut world: World) -> World {
    let game = world.game().clone();
    let me = world.me().clone();
    world.update(&me, &game);
    world
}
