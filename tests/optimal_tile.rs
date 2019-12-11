use model::Item;
use aicup2019::{
    Debug,
    examples::example_world,
    my_strategy::{
        Location,
        Positionable,
        World,
        get_optimal_tile,
        get_tile_score,
    },
};

#[test]
fn test_get_optimal_tile() {
    use std::io::BufWriter;

    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let world = updated_world(example_world());

    assert_eq!(get_optimal_tile(&world, &mut Debug(&mut stream)), Some(Location::new(10, 1)));
}

#[test]
fn test_get_tile_score_random_tiles() {
    let world = updated_world(example_world());
    {
        let location = Location::new(10, 5);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            -0.655
        );
    }
    {
        let location = Location::new(11, 5);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            -0.65
        );
    }
    {
        let location = Location::new(10, 3);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            -0.5173589016029712
        );
    }
    {
        let location = Location::new(10, 2);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            -0.5166620496359697
        );
    }
    {
        let location = Location::new(15, 4);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            -0.4892760153841629
        );
    }
    {
        let location = Location::new(36, 2);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            -0.33973476573298944
        );
    }
    {
        let location = Location::new(2, 2);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            -1.6515
        );
    }
}

#[test]
fn test_get_tile_score_optimal_tile() {
    let world = updated_world(example_world());
    let location = Location::new(10, 1);
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        2.478685919563607
    );
}

#[test]
fn test_get_tile_score_for_tile_with_bullet() {
    let world = updated_world(example_world());
    let location = world.bullets().iter()
        .find(|v| v.unit_id != world.me().id)
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        1.424442446900973
    );
}

#[test]
fn test_get_tile_score_for_tile_with_opponent() {
    let world = updated_world(example_world());
    let location = world.units().iter()
        .find(|v| v.player_id != world.me().player_id)
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        -1.6685710678118655
    );
}

#[test]
fn test_get_tile_score_for_tile_with_weapon() {
    let world = updated_world(example_world());
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Weapon { .. } = v.item { true } else { false })
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        2.26
    );
}

#[test]
fn test_get_tile_score_for_tile_with_health_pack() {
    let world = updated_world(example_world());
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::HealthPack { .. } = v.item { true } else { false })
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        1.23
    );
}

#[test]
fn test_get_tile_score_for_tile_with_loot_box_mine() {
    let world = updated_world(example_world());
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Mine { } = v.item { true } else { false })
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        -0.705
    );
}

#[test]
fn test_get_tile_score_for_tile_with_mine() {
    let world = updated_world(example_world());
    let location = world.mines()[0].location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        -0.6
    );
}

fn updated_world(mut world: World) -> World {
    let game = world.game().clone();
    let me = world.me().clone();
    world.update(&me, &game);
    world
}
