mod helpers;

use helpers::updated_world;

use aicup2019::{
    examples::{
        EXAMPLE_MY_UNIT_ID,
        example_world,
    },
    my_strategy::{
        Debug,
        Location,
        get_optimal_path,
        shortcut_tiles_path,
        update_locations_score,
    }
};

#[test]
fn test_get_optimal_path() {
    use std::io::BufWriter;

    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let mut debug = aicup2019::Debug(&mut stream);
    let mut locations_score = std::iter::repeat(0).take(world.level().size()).collect();

    update_locations_score(unit, &world, &Vec::new(), &mut locations_score, &mut Debug::new(&mut debug));

    assert_eq!(
        get_optimal_path(&unit, &locations_score, &world, &mut Debug::new(&mut debug)),
        (
            1223333,
            vec![Location::new(33, 1), Location::new(32, 1), Location::new(29, 1)]
        )
    );
}

#[test]
fn test_shortcut_tiles_path_with_wall() {
    let world = updated_world(example_world());
    assert_eq!(
        shortcut_tiles_path(
            Location::new(37, 1),
            vec![
                Location::new(36, 1), Location::new(35, 1), Location::new(34, 2), Location::new(33, 3),
                Location::new(32, 4), Location::new(31, 5), Location::new(30, 5), Location::new(29, 5),
            ],
            world.level()
        ),
        vec![Location::new(31, 5), Location::new(29, 5)]
    );
}

#[test]
fn test_shortcut_tiles_path_with_jump_pad() {
    let world = updated_world(example_world());
    assert_eq!(
        shortcut_tiles_path(
            Location::new(37, 1),
            vec![
                Location::new(36, 1), Location::new(35, 1), Location::new(34, 1), Location::new(33, 1),
                Location::new(32, 1), Location::new(31, 1), Location::new(30, 1), Location::new(29, 1),
            ],
            world.level()
        ),
        vec![Location::new(33, 1), Location::new(32, 1), Location::new(29, 1)]
    );
}

#[test]
fn test_shortcut_tiles_path_empty() {
    let world = updated_world(example_world());
    assert_eq!(
        shortcut_tiles_path(
            Location::new(37, 1),
            vec![],
            world.level()
        ),
        vec![]
    );
}

#[test]
fn test_shortcut_tiles_path_without_obstacles() {
    let world = updated_world(example_world());
    assert_eq!(
        shortcut_tiles_path(
            Location::new(37, 1),
            vec![
                Location::new(36, 1), Location::new(35, 1), Location::new(34, 2), Location::new(33, 3),
                Location::new(32, 4), Location::new(31, 5),
            ],
            world.level()
        ),
        vec![Location::new(31, 5)]
    );
}

#[test]
fn test_shortcut_tiles_path_with_single_element() {
    let world = updated_world(example_world());
    assert_eq!(
        shortcut_tiles_path(
            Location::new(37, 1),
            vec![Location::new(36, 1)],
            world.level()
        ),
        vec![Location::new(36, 1)]
    );
}
